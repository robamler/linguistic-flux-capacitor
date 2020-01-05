use rand::RngCore;

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
        assert!(min_symbol as usize + frequencies.len() <= 256);
        let mut cdf = [0u8; 257];
        let mut inverse_cdf = [0u8; 256];

        let mut accum = 0u32;
        for ((frequency, dest_cdf), symbol) in frequencies
            .iter()
            .zip(cdf[min_symbol as usize..].iter_mut())
            .zip(min_symbol..)
        {
            *dest_cdf = accum as u8;
            let new_accum = accum + *frequency as u32;
            for dest_inverse_cdf in &mut inverse_cdf[accum as usize..new_accum as usize] {
                *dest_inverse_cdf = symbol as u8;
            }
            accum = new_accum;
        }

        assert_eq!(accum, 256);

        Self { cdf, inverse_cdf }
    }

    pub fn entropy(&self) -> f32 {
        let mut last_accum = 0;
        let f_log2f = self
            .cdf
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
        let mut buf: u32 = 0x0001_0000;

        for symbol in uncompressed.iter().rev() {
            // Invariant at this point: `buf >= 0x0001_0000`
            let cdf = self.cdf[*symbol as usize];
            let next_cdf = unsafe {
                // This is always safe because `self.cdf` has type `[u8; 257]` and `*symbol`
                // has type `u8`, so `*symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
                // Note: We could instead make `self.cdf` of length only `256` and wrap around
                //       at the end but this turns out to hurt performance.
                self.cdf.get_unchecked(*symbol as usize + 1)
            };
            let frequency = next_cdf.wrapping_sub(cdf);

            // If emitting two bytes and then pushing `symbol` on `buf` results in
            // `buf >= 0x0001_0000`, then do it. If not, then just pushing `symbol`
            // on `buf` is guaranteed not to overflow.
            if buf >= (frequency as u32) << 24 {
                compressed.push((buf & 0xffff) as u16);
                buf >>= 16;
                // This is the only time where `buf < 0x0100_0000`. Thus, the decoder,
                // which operates in the opposite order, can use a check for
                // `buf < 0x0100_0000` to see if it has to read the next byte.
            }

            // Push `symbol` on buf.
            let prefix = buf / frequency as u32;
            let suffix = buf.checked_rem(frequency as u32).ok_or(())? + cdf as u32;
            buf = (prefix << 8) | suffix;
        }

        for _ in 0..2 {
            compressed.push((buf & 0xffff) as u16);
            buf >>= 16;
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
            if state < 0x0001_0000 {
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
        if self.cursor == self.compressed.len() && self.state == 0x0001_0000 {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn create_distribution() {
        let min_symbol = 100;
        let frequencies = [10, 1, 15, 0, 0, 7, 100, 110, 13];
        let distribution = DistributionU8::new(min_symbol, &frequencies);

        let mut counts = [0u8; 256];
        let mut last_symbol = min_symbol;
        for symbol in distribution.inverse_cdf.iter() {
            assert!(*symbol >= last_symbol);
            assert!((*symbol as usize) < min_symbol as usize + frequencies.len());
            last_symbol = *symbol;
            counts[*symbol as usize] += 1;
        }

        assert_eq!(
            &counts[min_symbol as usize..min_symbol as usize + frequencies.len()],
            &frequencies
        );

        assert!((distribution.entropy() - 1.867_519_4).abs() < 1e-6);
    }

    fn make_distribution() -> DistributionU8 {
        DistributionU8::new(100, &[10, 1, 15, 0, 0, 7, 100, 110, 13])
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
