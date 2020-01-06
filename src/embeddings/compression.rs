use rand::RngCore;

const MIN_ENCODER_STATE: u32 = 0x0001_0000;
const INITIAL_ENCODER_STATE: u32 = MIN_ENCODER_STATE;

pub struct DistributionU8 {
    /// Last entry is always zero.
    cdf: [u8; 257],
    inverse_cdf: [u8; 256],
}

impl std::fmt::Debug for DistributionU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DistributionU8")
            .field(
                "cdf",
                &self
                    .cdf
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .field(
                "inverse_cdf",
                &self
                    .inverse_cdf
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .finish()
    }
}

impl DistributionU8 {
    pub fn new(min_symbol: u8, frequencies: &[u8]) -> Self {
        assert!(frequencies.len() <= 256);
        let mut cdf = [0u8; 257];
        let mut inverse_cdf = [0u8; 256];

        let mut accum = 0u32;
        for (frequency, symbol) in frequencies.iter().zip(min_symbol as usize..) {
            cdf[symbol & 0xff] = accum as u8;
            let new_accum = accum + *frequency as u32;
            for dest_inverse_cdf in &mut inverse_cdf[accum as usize..new_accum as usize] {
                *dest_inverse_cdf = symbol as u8;
            }
            accum = new_accum;
        }

        // This is an optimization. The encoder and decoder access `cdf` at positions
        // `symbol` and `symbol + 1`, where `symbol` is a `u8`. The `cdf` logically
        // wraps around after index `255` (i.e., after the 256th entry), so
        // `cdf[symbol + 1]` should resolve to `cdf[0]` if `symbol == 255`. We could
        // explicitly implement the wrapping on each lookup via
        // `cdf[symbol.wrapping_add(1) as usize]` but this turns out to hurt
        // performance, so we instead make `cdf` one entry longer and wrap it around
        // explicitly.
        cdf[256] = cdf[0];

        assert_eq!(accum, 256);

        Self { cdf, inverse_cdf }
    }

    pub fn entropy(&self) -> f32 {
        let mut last_accum = self.cdf[0];
        let f_log2f = self.cdf[1..]
            .iter()
            .map(|accum| {
                let freq = accum.wrapping_sub(last_accum);
                last_accum = *accum;
                if freq != 0 {
                    (freq as f32) * (freq as f32).log2()
                } else {
                    0.0
                }
            })
            .sum::<f32>();

        8.0 - f_log2f / 256.0
    }

    pub fn generate_samples(&self, amt: usize, rng: &mut impl RngCore) -> Vec<u8> {
        (0..amt)
            .map(|_| self.inverse_cdf[(rng.next_u32() & 0xff) as usize])
            .collect()
    }

    /// Encode (compress) a sequence of symbols using ANS.
    ///
    /// In contrast to decoding, encoding cannot be done in a streaming fashion
    /// because the encoder has to process the data in reverse direction.
    ///
    /// # Returns
    ///
    /// A vector of the compressed message or an error if `uncompressed` contains a
    /// symbol that should have zero frequency according to the distribution `self`.
    pub fn encode(&self, uncompressed: &[u8]) -> Result<Vec<u16>, ()> {
        let mut compressed = Vec::new();
        let mut state: u32 = INITIAL_ENCODER_STATE;

        for symbol in uncompressed.iter().rev() {
            // Invariant at this point: `state >= MIN_ENCODER_STATE`
            let cdf = self.cdf[*symbol as usize];
            let next_cdf = unsafe {
                // This is always safe because `self.cdf` has type `[u8; 257]` and `*symbol`
                // has type `u8`, so `*symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
                // Note: We could instead make `self.cdf` of length only `256` and wrap around
                //       at the end but this turns out to hurt performance.
                self.cdf.get_unchecked(symbol.wrapping_add(1) as usize)
            };
            let frequency = next_cdf.wrapping_sub(cdf);

            // If emitting two bytes and then pushing `symbol` on `state` results in
            // `state >= MIN_ENCODER_STATE`, then do it. If not, then just pushing
            // `symbol` on `state` is guaranteed not to overflow.
            if state >= (frequency as u32) << 24 {
                compressed.push((state & 0xffff) as u16);
                state >>= 16;
                // This is the only time where `state < MIN_ENCODER_STATE`. Thus, the
                // decoder, which operates in the reverse order, can use a check for
                // `state < MIN_ENCODER_STATE` to see if it has to read the next byte.
            }

            // Push `symbol` on `state`.
            let prefix = state / frequency as u32;
            let suffix = state.checked_rem(frequency as u32).ok_or(())? + cdf as u32;
            state = (prefix << 8) | suffix;
        }

        for _ in 0..2 {
            compressed.push((state & 0xffff) as u16);
            state >>= 16;
        }

        compressed.reverse();
        Ok(compressed)
    }

    pub fn decoder<'a, 'b>(&'a self, compressed: &'b [u16]) -> Result<Decoder<'a, 'b>, ()> {
        Decoder::new(self, compressed)
    }

    pub fn decode_all_to(&self, compressed: &[u16], uncompressed: &mut [u8]) -> Result<(), ()> {
        let mut decoder = self.decoder(compressed)?;
        decoder.decode_to(uncompressed)?;
        decoder.finish()
    }
}

pub struct Decoder<'a, 'b> {
    distribution: &'a DistributionU8,
    state: u32,
    cursor: usize,
    compressed: &'b [u16],
}

impl<'a, 'b> Decoder<'a, 'b> {
    fn new(distribution: &'a DistributionU8, compressed: &'b [u16]) -> Result<Self, ()> {
        let state =
            (*compressed.get(0).ok_or(())? as u32) << 16 | *compressed.get(1).ok_or(())? as u32;
        Ok(Self {
            distribution,
            compressed,
            state,
            cursor: 2,
        })
    }

    pub fn decode<I: Iterator>(
        &mut self,
        dest_iter: I,
        mut callback: impl FnMut(u8, I::Item),
    ) -> Result<(), ()> {
        // Dereference all fields just once before we enter the hot loop, and then
        // never dereference them in the loop. This turns out to improve
        // performance.
        let mut cursor = self.cursor;
        let mut state = self.state;
        let compressed = self.compressed;
        let cdf = &self.distribution.cdf;
        let inverse_cdf = &self.distribution.inverse_cdf;

        for dest in dest_iter {
            // Pop `symbol` off `state` and call `callback`.
            let suffix = state & 0xff;
            let symbol = inverse_cdf[suffix as usize];
            callback(symbol, dest);

            // Update `state`.
            let cdf_value = cdf[symbol as usize];
            let next_cdf_value = unsafe {
                // This is always safe because `self.cdf` has type `[u8; 257]` and `symbol`
                // has type `u8`, so `symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
                // Note: We could instead make `cdf` of length only `256` and wrap around
                //       at the end but this turns out to hurt performance.
                cdf.get_unchecked(symbol as usize + 1)
            };
            let frequency = next_cdf_value.wrapping_sub(cdf_value);
            state = frequency as u32 * (state >> 8) + suffix - cdf_value as u32;

            // Refill `state` from data source if necessary.
            if state < MIN_ENCODER_STATE {
                state = (state << 16) | *compressed.get(cursor).ok_or(())? as u32;
                cursor += 1;
            }
        }

        self.cursor = cursor;
        self.state = state;

        Ok(())
    }

    pub fn decode_to(&mut self, dest: &mut [u8]) -> Result<(), ()> {
        self.decode(dest.iter_mut(), |symbol, dest| *dest = symbol)
    }

    pub fn decode_wrapping_add(&mut self, dest: &mut [i8]) -> Result<(), ()> {
        self.decode(dest.iter_mut(), |symbol, dest| {
            *dest = dest.wrapping_add(symbol as i8)
        })
    }

    pub fn skip(&mut self, amt: usize) -> Result<(), ()> {
        self.decode(0..amt, |_, _| ())
    }

    /// Check if encoder is in a valid "end" state and then drop it.
    ///
    /// If you don't want to read a compressed stream to the end and want to stop
    /// early instead, you can just drop the decoder using `std::mem::drop`. There
    /// is no way to verify data integrity without reading to the end of the stream.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err(())` if there is either data left or if the
    /// decoder is not in the expected final state (indicating data corruption).
    pub fn finish(self) -> Result<(), ()> {
        if self.cursor == self.compressed.len() && self.state == MIN_ENCODER_STATE {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Check if decoder state is consistent with EOF, regardless of whether there
    /// is more uncompressed data available.
    ///
    /// This function is useful to verify data integrity after decoding in a
    /// scenario where the byte slice that was used to create the encoder may
    /// contain further unnecessary data at the end (e.g., if only the size of the
    /// decoded data is known, or if the size of the compressed data is encoded in
    /// the data itself). If the compressed data size is known at the time the
    /// decoder is constructed, then use [`finish`](#method.finish) instead.
    ///
    /// A return value of `true` from this method is a necessary but not a
    /// sufficient condition that the decoder has reached the end of a compressed
    /// stream.
    pub fn could_be_end(&self) -> bool {
        self.state == INITIAL_ENCODER_STATE
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn distribution() {
        let min_symbol = 250;
        let frequencies = [10, 1, 15, 0, 0, 7, 100, 110, 13];
        let distribution = DistributionU8::new(min_symbol, &frequencies);

        let mut counts = [0u8; 256];
        let mut last_symbol = min_symbol;
        let mut num_decreases = 0;
        for symbol in distribution.inverse_cdf.iter() {
            if *symbol < last_symbol {
                num_decreases += 1
            }
            last_symbol = *symbol;
            counts[*symbol as usize] += 1;
        }

        assert!(num_decreases <= 1);
        assert_eq!(&counts[250..], &frequencies[..6]);
        assert_eq!(&counts[..3], &frequencies[6..]);
        assert!((distribution.entropy() - 1.867_519_4).abs() < 1e-6);
    }

    fn make_distribution() -> DistributionU8 {
        DistributionU8::new(250, &[10, 1, 15, 0, 0, 7, 100, 110, 13])
    }

    fn test_single_roundtrip(uncompressed_len: usize, seed: u64) {
        let distribution = make_distribution();
        let mut rng = StdRng::seed_from_u64(seed);
        let uncompressed = distribution.generate_samples(uncompressed_len, &mut rng);

        let compressed = distribution.encode(&uncompressed).unwrap();
        dbg!(2 * compressed.len());
        dbg!(uncompressed.len() as f32 * distribution.entropy() / 8.0);

        let mut decompressed = vec![0u8; uncompressed_len];
        distribution
            .decode_all_to(&compressed, &mut decompressed)
            .unwrap();

        assert_eq!(&uncompressed, &decompressed);
    }

    #[test]
    fn roundtrip() {
        let mut rng = StdRng::seed_from_u64(1234);
        for uncompressed_len in 0..128 {
            test_single_roundtrip(uncompressed_len, rng.next_u64());
        }
        for uncompressed_len in &[1000, 3000, 5000, 10_000, 100_000, 1_000_000] {
            test_single_roundtrip(*uncompressed_len, rng.next_u64());
        }
    }
}
