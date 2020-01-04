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

    pub fn encode_u32_8(&self, uncompressed: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut buf: u32 = 0x0100_0000;

        for symbol in uncompressed.iter().rev() {
            // Invariant at this point: `buf >= 0x0100_0000`
            let cdf = self.cdf[*symbol as usize];
            let next_cdf = unsafe {
                // This is always safe because `self.cdf` has type `[u8; 257]` and `symbol`
                // has type `u8`, so `symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
                self.cdf.get_unchecked(*symbol as usize + 1)
            };
            let frequency = next_cdf.wrapping_sub(cdf);

            // If emitting a byte and then pushing `symbol` on `buf` results in
            // `buf >= 0x0100_0000`, then do it. If not, then just pushing `symbol`
            // on `buf` is guaranteed not to overflow.
            if buf >= (frequency as u32) << 24 {
                compressed.push((buf & 0xff) as u8);
                buf >>= 8;
                // This is the only time where `buf < 0x0100_0000`. Thus, the decoder,
                // which operates in the opposite order, can use a check for
                // `buf < 0x0100_0000` to see if it has to read the next byte.
            }

            // Push `symbol` on buf.
            // This panics if `frequency` is zero, which may actually be a good thing.
            let prefix = buf / frequency as u32;
            let suffix = (buf % frequency as u32) + cdf as u32;
            buf = (prefix << 8) | suffix;
        }

        for _ in 0..4 {
            compressed.push((buf & 0xff) as u8);
            buf >>= 8;
        }

        compressed.reverse();
        compressed
    }

    /// # Safety
    ///
    /// TODO
    pub unsafe fn decode_u32_8_unchecked(&self, compressed: &[u8], uncompressed: &mut [u8]) {
        let mut buf = (*compressed.get_unchecked(0) as u32) << 24
            | (*compressed.get_unchecked(1) as u32) << 16
            | (*compressed.get_unchecked(2) as u32) << 8
            | *compressed.get_unchecked(3) as u32;

        let mut cursor_compressed = 4;

        for dest in uncompressed.iter_mut() {
            // Pop `symbol` off `buf`.
            let suffix = buf & 0xff;
            let symbol = self.inverse_cdf[suffix as usize];
            *dest = symbol;

            let cdf = self.cdf[symbol as usize];
            let frequency = self
                .cdf
                .get_unchecked(symbol as usize + 1)
                .wrapping_sub(cdf);
            buf = frequency as u32 * (buf >> 8) + suffix - cdf as u32;

            // Refill `buf` if necessary.
            // (This branch could be replaced by bit masks but it seems to hurt performance.)
            if buf < 0x0100_0000 {
                buf = (buf << 8) | *compressed.get_unchecked(cursor_compressed) as u32;
                cursor_compressed += 1;
            }
        }
    }

    pub fn encode_u32_16(&self, uncompressed: &[u8]) -> Vec<u16> {
        let mut compressed = Vec::new();
        let mut buf: u32 = 0x0001_0000;

        for symbol in uncompressed.iter().rev() {
            // Invariant at this point: `buf >= 0x0001_0000`
            let cdf = self.cdf[*symbol as usize];
            let next_cdf = unsafe {
                // This is always safe because `self.cdf` has type `[u8; 257]` and `symbol`
                // has type `u8`, so `symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
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
            // This panics if `frequency` is zero, which may actually be a good thing.
            let prefix = buf / frequency as u32;
            let suffix = (buf % frequency as u32) + cdf as u32;
            buf = (prefix << 8) | suffix;
        }

        for _ in 0..2 {
            compressed.push((buf & 0xffff) as u16);
            buf >>= 16;
        }

        compressed.reverse();
        compressed
    }

    /// # Safety
    ///
    /// TODO
    pub unsafe fn decode_u32_16_unchecked(&self, compressed: &[u16], uncompressed: &mut [u8]) {
        let mut buf =
            (*compressed.get_unchecked(0) as u32) << 16 | *compressed.get_unchecked(1) as u32;

        let mut cursor_compressed = 2;

        for dest in uncompressed.iter_mut() {
            // Pop `symbol` off `buf`.
            let suffix = buf & 0xff;
            let symbol = self.inverse_cdf[suffix as usize];
            *dest = symbol;

            let cdf = self.cdf[symbol as usize];
            let frequency = self
                .cdf
                .get_unchecked(symbol as usize + 1)
                .wrapping_sub(cdf);
            buf = frequency as u32 * (buf >> 8) + suffix - cdf as u32;

            // Refill `buf` if necessary.
            if buf < 0x0001_0000 {
                buf = (buf << 16) | *compressed.get_unchecked(cursor_compressed) as u32;
                cursor_compressed += 1;
            }
        }

        debug_assert_eq!(cursor_compressed, compressed.len());
        debug_assert_eq!(buf, 0x0001_0000);
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

    fn test_single_roundtrip_u32_8(uncompressed_len: usize, seed: u64) {
        let distribution = make_distribution();
        let mut rng = StdRng::seed_from_u64(seed);
        let uncompressed = distribution.generate_samples(uncompressed_len, &mut rng);

        let compressed = distribution.encode_u32_8(&uncompressed);
        dbg!(compressed.len());
        dbg!(uncompressed.len() as f32 * distribution.entropy() / 8.0);

        let mut decompressed = vec![0u8; uncompressed_len];
        unsafe {
            distribution.decode_u32_8_unchecked(&compressed, &mut decompressed);
        }

        assert_eq!(&uncompressed, &decompressed);
    }

    #[test]
    fn roundtrip_u32_8() {
        let mut rng = StdRng::seed_from_u64(1234);
        for uncompressed_len in 0..128 {
            test_single_roundtrip_u32_8(uncompressed_len, rng.next_u64());
        }
        for uncompressed_len in &[1000, 3000, 5000, 10_000, 100_000, 1_000_000] {
            test_single_roundtrip_u32_8(*uncompressed_len, rng.next_u64());
        }
    }

    fn test_single_roundtrip_u32_16(uncompressed_len: usize, seed: u64) {
        let distribution = make_distribution();
        let mut rng = StdRng::seed_from_u64(seed);
        let uncompressed = distribution.generate_samples(uncompressed_len, &mut rng);

        let compressed = distribution.encode_u32_16(&uncompressed);
        dbg!(2 * compressed.len());
        dbg!(uncompressed.len() as f32 * distribution.entropy() / 8.0);

        let mut decompressed = vec![0u8; uncompressed_len];
        unsafe {
            distribution.decode_u32_16_unchecked(&compressed, &mut decompressed);
        }

        assert_eq!(&uncompressed, &decompressed);
    }

    #[test]
    fn roundtrip_u32_16() {
        let mut rng = StdRng::seed_from_u64(1234);
        for uncompressed_len in 0..128 {
            test_single_roundtrip_u32_16(uncompressed_len, rng.next_u64());
        }
        for uncompressed_len in &[1000, 3000, 5000, 10_000, 100_000, 1_000_000] {
            test_single_roundtrip_u32_16(*uncompressed_len, rng.next_u64());
        }
    }
}
