use num::{
    traits::{WrappingAdd, WrappingSub},
    CheckedDiv, One, Zero,
};
use rand::RngCore;
use std::collections::HashMap;
use std::mem::{size_of, MaybeUninit};
use std::ops::{BitAnd, Div, Mul, Shl, Shr};
use std::{cmp::Ord, marker::PhantomData};

pub struct EntropyModel<O: EntropyModelOptions> {
    cdf_and_symbols: Box<[(O::Frequency, O::Symbol)]>,

    /// Invariant: `inverse_cdf[i] < cdf_and_symbols.len() - 1` for all `i`.
    inverse_cdf: Box<[O::Frequency]>,
}

impl<O: EntropyModelOptions> std::fmt::Debug for EntropyModel<O>
where
    O::Symbol: std::fmt::Display,
    O::Frequency: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntropyModel")
            .field(
                "inverse_cdf",
                &self
                    .inverse_cdf
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .field(
                "cdf_and_symbols",
                &self
                    .cdf_and_symbols
                    .iter()
                    .map(|(accum, symbol)| format!("{} -> {}", accum, symbol))
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .finish()
    }
}
pub unsafe trait EntropyModelOptions {
    const FREQUENCY_BITS: usize;
    type Frequency: num::PrimInt + Into<usize> + From<u8> + WrappingAdd + WrappingSub;
    type Symbol: Clone;
    type CompressedWord: Copy;

    /// Must hold two `CompressedWord`s.
    type State: num::PrimInt + From<Self::Frequency> + From<Self::CompressedWord>;

    #[inline(always)]
    fn min_state() -> Self::State {
        Self::State::one() << (8 * size_of::<Self::CompressedWord>())
    }

    #[inline(always)]
    fn total_frequency() -> usize {
        1 << Self::FREQUENCY_BITS
    }

    #[inline(always)]
    fn threshold_encoder_state() -> Self::State {
        Self::State::one() << (16 * size_of::<Self::CompressedWord>() - Self::FREQUENCY_BITS)
    }

    #[inline(always)]
    fn pop_compressed_word_off_state(state: &mut Self::State) -> Self::CompressedWord {
        let word_bits = 8 * size_of::<Self::CompressedWord>();
        let result =
            Self::truncate_state_to_compressed_word(*state % (Self::State::one() << word_bits));
        *state = *state >> word_bits;
        result
    }

    fn truncate_state_to_compressed_word(state: Self::State) -> Self::CompressedWord;

    fn state_as_frequency(state: Self::State) -> Self::Frequency;

    fn frequency_from_usize(n: usize) -> Self::Frequency;
}

pub struct EntropyModelOptions12_16;
pub struct EntropyModelOptions12_32;

unsafe impl EntropyModelOptions for EntropyModelOptions12_16 {
    const FREQUENCY_BITS: usize = 12;
    type Frequency = u16;
    type Symbol = i16;
    type CompressedWord = u16;
    type State = u32;

    #[inline(always)]
    fn frequency_from_usize(n: usize) -> Self::Frequency {
        n as Self::Frequency
    }

    #[inline(always)]
    fn truncate_state_to_compressed_word(state: Self::State) -> Self::CompressedWord {
        state as Self::CompressedWord
    }

    #[inline(always)]
    fn state_as_frequency(state: Self::State) -> Self::Frequency {
        state as Self::Frequency
    }
}

unsafe impl EntropyModelOptions for EntropyModelOptions12_32 {
    const FREQUENCY_BITS: usize = 12;
    type Frequency = u16;
    type Symbol = i16;
    type CompressedWord = u32;
    type State = u64;

    #[inline(always)]
    fn frequency_from_usize(n: usize) -> Self::Frequency {
        n as Self::Frequency
    }

    #[inline(always)]
    fn truncate_state_to_compressed_word(state: Self::State) -> Self::CompressedWord {
        state as Self::CompressedWord
    }

    #[inline(always)]
    fn state_as_frequency(state: Self::State) -> Self::Frequency {
        state as Self::Frequency
    }
}

type EntropyModel12_16 = EntropyModel<EntropyModelOptions12_16>;
type EntropyModel12_32 = EntropyModel<EntropyModelOptions12_32>;

impl<O: EntropyModelOptions> EntropyModel<O> {
    pub fn new(frequencies_except_last: &[O::Frequency], symbols: &[O::Symbol]) -> Self {
        debug_assert!(symbols.len() >= 2);
        debug_assert!(symbols.len() <= O::total_frequency());
        debug_assert_eq!(symbols.len(), frequencies_except_last.len() + 1);

        let mut inverse_cdf = Vec::with_capacity(O::total_frequency());
        inverse_cdf.resize(O::total_frequency(), O::Frequency::zero());
        let mut inverse_cdf = inverse_cdf.into_boxed_slice();

        let mut accum = O::Frequency::zero();
        let mut index = O::Frequency::zero();

        let mut cdf_and_symbols = frequencies_except_last
            .iter()
            .zip(symbols)
            .chain(
                // Append entries with wrong frequencies for now, will fix up below.
                [
                    (&O::Frequency::zero(), symbols.last().unwrap()),
                    (&O::Frequency::zero(), symbols.first().unwrap()),
                ]
                .iter()
                .cloned(),
            )
            .map(|(frequency, symbol)| {
                let accum_usize: usize = accum.into();
                let freq_usize: usize = (*frequency).into();
                for dest_inverse_cdf in &mut inverse_cdf[accum_usize..accum_usize + freq_usize] {
                    *dest_inverse_cdf = index;
                }
                let old_accum = accum;
                accum = accum + *frequency;
                index = index.wrapping_add(&O::Frequency::one());
                (old_accum, symbol.clone())
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        // Fix up last two entries now that we know final `accum` and `index`.
        cdf_and_symbols[cdf_and_symbols.len() - 2].0 = accum;
        cdf_and_symbols[cdf_and_symbols.len() - 1].0 =
            O::frequency_from_usize(O::total_frequency());

        index = index.wrapping_sub(&O::Frequency::from(2));
        for dest_inverse_cdf in &mut inverse_cdf[accum.into()..] {
            *dest_inverse_cdf = index;
        }

        Self {
            cdf_and_symbols,
            inverse_cdf,
        }
    }

    pub fn entropy(&self) -> f32 {
        let mut previous_accum = self.cdf_and_symbols[0].0;
        let f_log2f = self.cdf_and_symbols[1..]
            .iter()
            .map(|(accum, _)| {
                let freq = accum.wrapping_sub(&previous_accum);
                previous_accum = *accum;
                debug_assert!(freq != O::Frequency::zero());
                let freq: usize = freq.into();
                let freq = freq as f32;
                freq * freq.log2()
            })
            .sum::<f32>();

        O::FREQUENCY_BITS as f32 - f_log2f / O::total_frequency() as f32
    }

    pub fn generate_samples(&self, amt: usize, rng: &mut impl RngCore) -> Vec<O::Symbol> {
        let total_freq = Into::<usize>::into(O::total_frequency());
        (0..amt)
            .map(|_| unsafe {
                // SAFETY:
                // - `inverse_cdf` has `O::total_frequency()` entries, so indexing
                //   with some value `% O::total_frequency()` is always within bounds.
                // - The entries of `inverse_cdf` are guaranteed to be within bounds
                //   for indexing into `cdf_and_symbols`.
                let index = *self
                    .inverse_cdf
                    .get_unchecked(rng.next_u32() as usize % total_freq);
                self.cdf_and_symbols
                    .get_unchecked(Into::<usize>::into(index))
                    .1
                    .clone()
            })
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
    /// symbol that should have zero frequency according to the entropy model.
    pub fn encode(&self, uncompressed: &[O::Symbol]) -> Result<Vec<O::CompressedWord>, ()>
    where
        O::Symbol: Eq + std::hash::Hash,
    {
        // TODO: this should be cached --> Create an `Encoder` class that encapsulates this table and the distribution
        let symbol_to_freq_and_cdf = self
            .cdf_and_symbols
            .windows(2)
            .map(|cdfs| {
                let (cdf, symbol) = &cdfs[0];
                let (next_cdf, _) = cdfs[1];
                (symbol, (next_cdf.wrapping_sub(cdf), *cdf))
            })
            .collect::<HashMap<_, _>>();

        let mut compressed = Vec::new();
        let mut state = O::min_state();

        for symbol in uncompressed.iter().rev() {
            // Invariant at this point: `state >= MIN_ENCODER_STATE`.
            let (frequency, cdf) = *symbol_to_freq_and_cdf.get(&symbol).ok_or(())?;
            let frequency: O::State = From::<O::Frequency>::from(frequency);

            // If emitting a compressed word and then pushing `symbol` on `state` results
            // in `state >= O::min_state()`, then do it. If not, then just pushing
            // `symbol` on `state` is guaranteed not to overflow.
            if state >= O::threshold_encoder_state() * frequency {
                compressed.push(O::pop_compressed_word_off_state(&mut state));
                // This is the only time where `state < O::min_state()`. Thus,
                // the decoder, which operates in the reverse order, can use a check for
                // `state < O::min_state()` to see if it has to refill `state`.
            }

            // Push `symbol` on `state`.
            let prefix = state.checked_div(&frequency).ok_or(())?;
            let suffix = state % frequency + From::<O::Frequency>::from(cdf);
            state = (prefix << O::FREQUENCY_BITS) | suffix;
        }

        // Flush last two words.
        compressed.push(O::pop_compressed_word_off_state(&mut state));
        compressed.push(O::pop_compressed_word_off_state(&mut state));

        compressed.reverse();
        Ok(compressed)
    }

    pub fn decode(
        &self,
        compressed: &[O::CompressedWord],
        amt: usize,
    ) -> Result<Vec<O::Symbol>, ()> {
        let word_size = 8 * size_of::<O::CompressedWord>();
        let mut uncompressed = Vec::with_capacity(amt);

        let mut compressed_iter = compressed.iter();
        let mut state = O::State::from(*compressed_iter.next().unwrap()) << word_size;
        let mut state = state | O::State::from(*compressed_iter.next().unwrap());

        for _ in 0..amt {
            //     // Invariant at this point: `state >= MIN_ENCODER_STATE`.
            //     let (frequency, cdf) = *symbol_to_freq_and_cdf.get(&symbol).ok_or(())?;
            //     let frequency = O::State::from(frequency);

            //     // If emitting a compressed word then pushing `symbol` on `state` results in
            //     // `state >= O::min_state()`, then do it. If not, then just pushing
            //     // `symbol` on `state` is guaranteed not to overflow.
            //     if state >= O::threshold_encoder_state() * frequency {
            //         compressed.push(O::pop_compressed_word_off_state(&mut state));
            //         // This is the only time where `state < O::min_state()`. Thus,
            //         // the decoder, which operates in the reverse order, can use a check for
            //         // `state < O::min_state()` to see if it has to refill `state`.
            //     }

            //     // Push `symbol` on `state`.
            //     let prefix = state.checked_div(&frequency).ok_or(())?;
            //     let suffix = state % frequency + O::State::from(cdf);
            // state = (prefix << O::FREQUENCY_BITS) | suffix;
            // prefix = state >> O::FREQUENCY_BITS;

            // Pop `symbol` off `state`.
            let suffix = state % (O::State::one() << O::FREQUENCY_BITS);
            let index = unsafe {
                self.inverse_cdf
                    .get_unchecked(O::state_as_frequency(suffix).into())
            };
            let index = Into::<usize>::into(*index);
            let (cdf, symbol) = unsafe { self.cdf_and_symbols.get_unchecked(index) };
            uncompressed.push(symbol.clone());

            // Update `state`.
            let next_cdf = unsafe {
                // This is always safe because `self.cdf` has type `[u8; 257]` and `symbol`
                // has type `u8`, so `symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
                // Note: We could instead make `cdf` of length only `256` and wrap around
                //       at the end but this turns out to hurt performance.
                self.cdf_and_symbols.get_unchecked(index + 1).0
            };
            let frequency = next_cdf.wrapping_sub(cdf);
            let fs: O::State = From::<O::Frequency>::from(frequency);
            state = fs * (state >> O::FREQUENCY_BITS) + suffix - From::<O::Frequency>::from(*cdf);

            // Refill `state` from data source if necessary.
            if state < O::min_state() {
                state = (state << word_size) | O::State::from(*compressed_iter.next().ok_or(())?);
            }

        }
        
        assert!(state == O::min_state() && compressed_iter.next().is_none());
        Ok(uncompressed)

        // // Flush last two words.
        // compressed.push(O::pop_compressed_word_off_state(&mut state));
        // compressed.push(O::pop_compressed_word_off_state(&mut state));

        // compressed.reverse();
        // Ok(compressed)
    }
}

#[cfg(test)]
mod test {
    //! TODO: Test with real data: overhead for small compressed words will probably
    //! only be visible when there are lots of symbols with small frequencies.

    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn construct_entropy_model_12_16() {
        test_run::<EntropyModelOptions12_16>(
            &[4, -5, 6],
            &[500, 2000, 1596],
            &[(0, 4), (500, -5), (2500, 6), (4096, 4)],
        );
        test_run::<EntropyModelOptions12_32>(
            &[4, -5, 6],
            &[500, 2000, 1596],
            &[(0, 4), (500, -5), (2500, 6), (4096, 4)],
        );
    }

    fn test_run<O: EntropyModelOptions>(
        symbols: &[O::Symbol],
        frequencies: &[O::Frequency],
        expected_cdf_and_symbols: &[(O::Frequency, O::Symbol)],
    ) where
        O::Frequency: Eq + std::fmt::Debug,
        O::Symbol: Eq + std::fmt::Debug + std::hash::Hash,
    {
        let ent = EntropyModel::<O>::new(&frequencies[..2], &symbols);

        assert_eq!(ent.cdf_and_symbols.len(), symbols.len() + 1);

        assert_eq!(&*(ent.cdf_and_symbols), expected_cdf_and_symbols);

        for (i, freq_pair) in ent.cdf_and_symbols.windows(2).enumerate() {
            for index in &ent.inverse_cdf[freq_pair[0].0.into()..freq_pair[1].0.into()] {
                assert_eq!(Into::<usize>::into(*index), i);
            }
        }

        let entropy = -(500.0 / 4096.0) * (500.0f32 / 4096.0).log2()
            - (2000.0 / 4096.0) * (2000.0f32 / 4096.0).log2()
            - (1596.0 / 4096.0) * (1596.0f32 / 4096.0).log2();
        assert!((ent.entropy() - entropy).abs() < 1e-6);

        let mut rng = StdRng::seed_from_u64(123);
        let samples = ent.generate_samples(100000, &mut rng);

        let mut counts = HashMap::new();
        for sample in &samples {
            counts
                .entry(sample)
                .and_modify(|c| *c += 1)
                .or_insert(1usize);
        }

        assert_eq!(counts.len(), 3);
        // Check that observed frequencies are within 10% of expected frequencies.
        for (symbol, freq) in symbols.iter().zip(frequencies) {
            let expected = Into::<usize>::into(*freq) * samples.len() / 4096;
            let observed = counts[symbol];
            assert!(observed > 9 * expected / 10);
            assert!(observed < 11 * expected / 10);
        }

        let compressed = ent.encode(&samples).unwrap();
        let expected_bitlength = entropy * samples.len() as f32;
        let observed_bitlength = (8 * size_of::<O::CompressedWord>() * compressed.len()) as f32;
        dbg!(expected_bitlength, observed_bitlength);
        assert!(observed_bitlength > 0.9 * expected_bitlength);
        assert!(observed_bitlength < 1.1 * expected_bitlength);

        let decoded = ent.decode(&compressed, samples.len()).unwrap();
        assert_eq!(decoded, samples)
    }
}

// trait CompressedWord: std::marker::Sized {
//     type State:From<u16> + num::PrimInt + std::ops::Rem<Output=Self::State>;
//     // type State:From<u16> + Shl<usize, Output=Self::State> + Shr<usize, Output=Self::State> + BitAnd<Output=Self::State>+ Mul<Output=Self::State>+Div<Output=Self::State>+Ord;

//     // TODO: turn into a `const fn`.
//     #[inline(always)]
//      fn min_state() -> Self::State{
//         Self::State::from(1) << (8*std::mem::size_of::<Self>())
//     }

//     // TODO: turn into a `const fn`.
//     #[inline(always)]
//     fn threshold_encoder_state(frequency_quantization:u8)->Self::State{
//         Self::State::from(1) << (16*std::mem::size_of::<Self>() - frequency_quantization as usize)
//     }

//     fn pop_state(state: &mut Self::State) -> Self;
//     // THRESHOLD_ENCODER_STATE * FREQUENCY_QUANTIZATION == (1 << 64) for a u64 state
//     // THRESHOLD_ENCODER_STATE / MIN_ENCODER_STATE == (1 << 32) for u32 compressed words
//     // const MIN_ENCODER_STATE: u64 = 1 << 20;
//     // const THRESHOLD_ENCODER_STATE: u64 = 1 << 52;
//     // const FREQUENCY_QUANTIZATION: u16 = 1 << 12;
//     // const INITIAL_ENCODER_STATE: u64 = MIN_ENCODER_STATE;
// }
// impl CompressedWord for u16{
//     type State=u32;

//     #[inline(always)]
//     fn pop_state(state:&mut  Self::State) -> Self{
//         let result = (*state & 0xffff) as u16;
//         *state >>= 16;
//         result
//     }
// }

// impl CompressedWord for u32{
//     type State=u64;

//     #[inline(always)]
//     fn pop_state(state:&mut  Self::State) -> Self{
//         let result = (*state & 0xffff_ffff) as u32;
//         *state >>= 32;
//         result
//     }
// }

// impl Distribution12bit {
//     pub fn new(frequencies_except_last: &[u16], symbols: &[i16]) -> Self {
//         debug_assert!(symbols.len() >= 2);
//         debug_assert!(symbols.len() <= 4096);
//         debug_assert_eq!(symbols.len(), frequencies_except_last.len() + 1);

//         let mut cdf_and_symbols = Vec::with_capacity(symbols.len() + 1);
//         cdf_and_symbols.resize(symbols.len() + 1, (0u16, 0i16));
//         let mut cdf_and_symbols = cdf_and_symbols.into_boxed_slice();
//         let mut inverse_cdf = [0u16; 0x1000];

//         let mut accum = 0u32;
//         for (i, ((frequency, symbol), (dest_accum, dest_symbol))) in frequencies_except_last
//             .iter()
//             .zip(symbols)
//             .zip(cdf_and_symbols.iter_mut())
//             .enumerate()
//         {
//             *dest_accum = accum as u16;
//             *dest_symbol = *symbol;

//             let new_accum = accum + *frequency as u32;
//             let i_u16 = i as u16;
//             for dest_inverse_cdf in &mut inverse_cdf[accum as usize..new_accum as usize] {
//                 *dest_inverse_cdf = i_u16;
//             }
//             accum = new_accum;
//         }

//         // Last entry:
//         cdf_and_symbols[symbols.len() - 1] = (accum as u16, *symbols.last().unwrap());
//         debug_assert!(accum < 0x1000);
//         let i_u16 = frequencies_except_last.len() as u16;
//         for dest_inverse_cdf in &mut inverse_cdf[accum as usize..] {
//             *dest_inverse_cdf = i_u16;
//         }

//         // One after last entry: set cdf to upper bound (symbol is arbitrary here).
//         // (Note: we could instead wrap around bu this seems to hurt performance)
//         (*cdf_and_symbols.last_mut().unwrap()).0 = 0x1000;

//         Self {
//             cdf_and_symbols,
//             inverse_cdf,
//         }
//     }

//     pub fn entropy(&self) -> f32 {
//         let mut last_accum = self.cdf_and_symbols[0].0;
//         let f_log2f = self.cdf_and_symbols[1..]
//             .iter()
//             .map(|(accum, _)| {
//                 let freq = accum - last_accum;
//                 last_accum = *accum;
//                 debug_assert!(freq != 0);
//                 (freq as f32) * (freq as f32).log2()
//             })
//             .sum::<f32>();

//         12.0 - f_log2f / 0x1000 as f32
//     }

//     pub fn generate_samples(&self, amt: usize, rng: &mut impl RngCore) -> Vec<i16> {
//         (0..amt)
//             .map(|_| unsafe {
//                 // SAFETY:
//                 // - `inverse_cdf` is of type `[_, 0x1000]`, so masking the index
//                 //   with 0x0fff results in an index that's always within bounds.
//                 // - The entries of `inverse_cdf` are guaranteed to be within bounds
//                 //   for indexing into `cdf_and_symbols`.
//                 let index = *self
//                     .inverse_cdf
//                     .get_unchecked((rng.next_u32() & 0x0fff) as usize);
//                 self.cdf_and_symbols.get_unchecked(index as usize).1
//             })
//             .collect()
//     }

//     /// Encode (compress) a sequence of symbols using ANS.
//     ///
//     /// In contrast to decoding, encoding cannot be done in a streaming fashion
//     /// because the encoder has to process the data in reverse direction.
//     ///
//     /// # Returns
//     ///
//     /// A vector of the compressed message or an error if `uncompressed` contains a
//     /// symbol that should have zero frequency according to the distribution `self`.
//     pub fn encode<CW: CompressedWord>(
//         &self,
//         uncompressed: &[i16],
//         symbol_to_index: HashMap<i16, u16>,
//     ) -> Result<Vec<CW>, ()> {
//         let mut compressed = Vec::new();
//         let mut state = CW::min_state();

//         for symbol in uncompressed.iter().rev() {
//             // Invariant at this point: `state >= MIN_ENCODER_STATE`.
//             let index = *symbol_to_index.get(symbol).ok_or(())?;

//             assert!(index as usize + 1 < self.cdf_and_symbols.len());
//             let ((cdf, sym), next_cdf) = unsafe {
//                 // SAFETY: Safe due to above assertion.
//                 (
//                     *self.cdf_and_symbols.get_unchecked(index as usize),
//                     self.cdf_and_symbols
//                         .get_unchecked(index as usize + 1)
//                         .0,
//                 )
//             };
//             debug_assert_eq!(sym, *symbol);
//             let frequency = CW::State::from(next_cdf-cdf);

//             // If emitting a compressed word then pushing `symbol` on `state` results in
//             // `state >= MIN_ENCODER_STATE`, then do it. If not, then just pushing
//             // `symbol` on `state` is guaranteed not to overflow.
//             if state >= CW::threshold_encoder_state(12) * frequency {
//                 compressed.push(CW::pop_state(&mut state));
//                 // This is the only time where `state < MIN_ENCODER_STATE`. Thus, the
//                 // decoder, which operates in the reverse order, can use a check for
//                 // `state < MIN_ENCODER_STATE` to see if it has to read the next byte.
//             }

//             // Push `symbol` on `state`.
//             let prefix = state.checked_div(&frequency).ok_or(())?;
//             let suffix = state %frequency + CW::State::from(cdf);
//             state = (prefix << 12) | suffix;
//         }

//         // Flush last two words.
//             compressed.push(CW::pop_state(&mut state));
//             compressed.push(CW::pop_state(&mut state));

//         compressed.reverse();
//         Ok(compressed)
//     }

//     pub fn decoder<'a, 'b,CW:CompressedWord>(&'a self, compressed: &'b [u16]) -> Result<Decoder<'a, 'b,CW>, ()> {
//         Decoder::new(self, compressed)
//     }

//     pub fn decode_all_to<CW:CompressedWord>(&self, compressed: &[CW], uncompressed: &mut [i16]) -> Result<(), ()> {
//         let mut decoder = self.decoder(compressed)?;
//         decoder.decode_to(uncompressed)?;
//         decoder.finish()
//     }
// }

// pub struct Decoder<'a, 'b,CW:CompressedWord> {
//     distribution: &'a Distribution12bit,
//     state: CW::State,
//     cursor: usize,
//     compressed: &'b [CW],
// }

// impl<'a, 'b,CW:CompressedWord> Decoder<'a, 'b,CW> {
//     fn new(distribution: &'a Distribution12bit, compressed: &'b [CW]) -> Result<Self, ()> {
//         let state =
//             CW::State::from(*compressed.get(0).ok_or(())?) << (8*std::mem::size_of::<CW>()) | CW::State::from(*compressed.get(1).ok_or(())?);
//         Ok(Self {
//             distribution,
//             compressed,
//             state,
//             cursor: 2,
//         })
//     }

//     pub fn decode<I: Iterator>(
//         &mut self,
//         dest_iter: I,
//         mut callback: impl FnMut(i16, I::Item),
//     ) -> Result<(), ()> {
//         // Dereference all fields just once before we enter the hot loop, and then
//         // never dereference them in the loop. This turns out to improve
//         // performance.
//         let mut cursor = self.cursor;
//         let mut state = self.state;
//         let compressed = self.compressed;
//         let cdf = &self.distribution.cdf;
//         let inverse_cdf = &self.distribution.inverse_cdf;

//         for dest in dest_iter {
//             // Pop `symbol` off `state` and call `callback`.
//             let suffix = state & 0x0fff;
//             let symbol = inverse_cdf[suffix as usize];
//             callback(symbol, dest);

//             // Update `state`.
//             let cdf_value = cdf[symbol as usize];
//             let next_cdf_value = unsafe {
//                 // This is always safe because `self.cdf` has type `[u8; 257]` and `symbol`
//                 // has type `u8`, so `symbol as usize + 1` is guaranteed to be within bounds.
//                 // Unfortunately, the compiler doesn't realize this automatically.
//                 // Note: We could instead make `cdf` of length only `256` and wrap around
//                 //       at the end but this turns out to hurt performance.
//                 cdf.get_unchecked(symbol as usize + 1)
//             };
//             let frequency = next_cdf_value.wrapping_sub(cdf_value);
//             state = frequency as u32 * (state >> 8) + suffix - cdf_value as u32;

//             // Refill `state` from data source if necessary.
//             if state < MIN_ENCODER_STATE {
//                 state = (state << 16) | *compressed.get(cursor).ok_or(())? as u32;
//                 cursor += 1;
//             }
//         }

//         self.cursor = cursor;
//         self.state = state;

//         Ok(())
//     }

//     pub fn decode_to(&mut self, dest: &mut [u8]) -> Result<(), ()> {
//         self.decode(dest.iter_mut(), |symbol, dest| *dest = symbol)
//     }

//     pub fn decode_wrapping_add(&mut self, dest: &mut [i8]) -> Result<(), ()> {
//         self.decode(dest.iter_mut(), |symbol, dest| {
//             *dest = dest.wrapping_add(symbol as i8)
//         })
//     }

//     pub fn skip(&mut self, amt: usize) -> Result<(), ()> {
//         self.decode(0..amt, |_, _| ())
//     }

//     /// Check if encoder is in a valid "end" state and then drop it.
//     ///
//     /// If you don't want to read a compressed stream to the end and want to stop
//     /// early instead, you can just drop the decoder using `std::mem::drop`. There
//     /// is no way to verify data integrity without reading to the end of the stream.
//     ///
//     /// # Returns
//     ///
//     /// `Ok(())` on success, `Err(())` if there is either data left or if the
//     /// decoder is not in the expected final state (indicating data corruption).
//     pub fn finish(self) -> Result<(), ()> {
//         if self.cursor == self.compressed.len() && self.state == MIN_ENCODER_STATE {
//             Ok(())
//         } else {
//             Err(())
//         }
//     }

//     /// Check if decoder state is consistent with EOF, regardless of whether there
//     /// is more uncompressed data available.
//     ///
//     /// This function is useful to verify data integrity after decoding in a
//     /// scenario where the byte slice that was used to create the encoder may
//     /// contain further unnecessary data at the end (e.g., if only the size of the
//     /// decoded data is known, or if the size of the compressed data is encoded in
//     /// the data itself). If the compressed data size is known at the time the
//     /// decoder is constructed, then use [`finish`](#method.finish) instead.
//     ///
//     /// A return value of `true` from this method is a necessary but not a
//     /// sufficient condition that the decoder has reached the end of a compressed
//     /// stream.
//     pub fn could_be_end(&self) -> bool {
//         self.state == INITIAL_ENCODER_STATE
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use rand::{rngs::StdRng, SeedableRng};

//     #[test]
//     fn distribution() {
//         let min_symbol = 250;
//         let frequencies = [10, 1, 15, 0, 0, 7, 100, 110, 13];
//         let distribution = DistributionU8::new(min_symbol, &frequencies);

//         let mut counts = [0u8; 256];
//         let mut last_symbol = min_symbol;
//         let mut num_decreases = 0;
//         for symbol in distribution.inverse_cdf.iter() {
//             if *symbol < last_symbol {
//                 num_decreases += 1
//             }
//             last_symbol = *symbol;
//             counts[*symbol as usize] += 1;
//         }

//         assert!(num_decreases <= 1);
//         assert_eq!(&counts[250..], &frequencies[..6]);
//         assert_eq!(&counts[..3], &frequencies[6..]);
//         assert!((distribution.entropy() - 1.867_519_4).abs() < 1e-6);
//     }

//     fn make_distribution() -> DistributionU8 {
//         DistributionU8::new(250, &[10, 1, 15, 0, 0, 7, 100, 110, 13])
//     }

//     fn test_single_roundtrip(uncompressed_len: usize, seed: u64) {
//         let distribution = make_distribution();
//         let mut rng = StdRng::seed_from_u64(seed);
//         let uncompressed = distribution.generate_samples(uncompressed_len, &mut rng);

//         let compressed = distribution.encode(&uncompressed).unwrap();
//         dbg!(2 * compressed.len());
//         dbg!(uncompressed.len() as f32 * distribution.entropy() / 8.0);

//         let mut decompressed = vec![0u8; uncompressed_len];
//         distribution
//             .decode_all_to(&compressed, &mut decompressed)
//             .unwrap();

//         assert_eq!(&uncompressed, &decompressed);
//     }

//     #[test]
//     fn roundtrip() {
//         let mut rng = StdRng::seed_from_u64(1234);
//         for uncompressed_len in 0..128 {
//             test_single_roundtrip(uncompressed_len, rng.next_u64());
//         }
//         for uncompressed_len in &[1000, 3000, 5000, 10_000, 100_000, 1_000_000] {
//             test_single_roundtrip(*uncompressed_len, rng.next_u64());
//         }
//     }
// }
