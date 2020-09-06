use num::{
    traits::{WrappingAdd, WrappingSub},
    CheckedDiv, One, Zero,
};
use rand::RngCore;
use std::collections::HashMap;
use std::mem::size_of;

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
        // This verifiably gets optimized to the same as the equivalent hand crafted
        // implementation in the case of u16 and u32 compressed words both on x86 and wasm.
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
    pub fn new(
        frequencies_except_last: impl IntoIterator<Item = O::Frequency>,
        symbols: &[O::Symbol],
    ) -> Self {
        debug_assert!(symbols.len() >= 2);
        debug_assert!(symbols.len() <= O::total_frequency());
        // debug_assert_eq!(symbols.len(), frequencies_except_last.len() + 1);

        let mut inverse_cdf = Vec::with_capacity(O::total_frequency());
        inverse_cdf.resize(O::total_frequency(), O::Frequency::zero());
        let mut inverse_cdf = inverse_cdf.into_boxed_slice();

        let mut accum = O::Frequency::zero();
        let mut index = O::Frequency::zero();

        let mut cdf_and_symbols = frequencies_except_last
            .into_iter()
            .zip(symbols)
            .chain(
                // Append entries with wrong frequencies for now, will fix up below.
                [
                    (O::Frequency::zero(), symbols.last().unwrap()),
                    (O::Frequency::zero(), symbols.first().unwrap()),
                ]
                .iter()
                .cloned(),
            )
            .map(|(frequency, symbol)| {
                let accum_usize: usize = accum.into();
                let freq_usize: usize = frequency.into();
                for dest_inverse_cdf in &mut inverse_cdf[accum_usize..accum_usize + freq_usize] {
                    *dest_inverse_cdf = index;
                }
                let old_accum = accum;
                accum = accum + frequency;
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
        let mut state = (O::State::from(*compressed_iter.next().unwrap()) << word_size)
            | O::State::from(*compressed_iter.next().unwrap());

        for _ in 0..amt {
            // Pop `symbol` off `state`.
            let suffix = state % (O::State::one() << O::FREQUENCY_BITS);
            let index = unsafe {
                // SAFETY: TODO
                self.inverse_cdf
                    .get_unchecked(O::state_as_frequency(suffix).into())
            };
            let index = Into::<usize>::into(*index);
            let (cdf, symbol) = unsafe {
                // SAFETY: TODO
                self.cdf_and_symbols.get_unchecked(index)
            };
            let next_cdf = unsafe {
                // SAFETY: TODO
                // This is always safe because `self.cdf` has type `[u8; 257]` and `symbol`
                // has type `u8`, so `symbol as usize + 1` is guaranteed to be within bounds.
                // Unfortunately, the compiler doesn't realize this automatically.
                // Note: We could instead make `cdf` of length only `256` and wrap around
                //       at the end but this turns out to hurt performance.
                self.cdf_and_symbols.get_unchecked(index + 1).0
            };
            uncompressed.push(symbol.clone());

            // Update `state`.
            let frequency = next_cdf.wrapping_sub(cdf);
            state = (state >> O::FREQUENCY_BITS) * From::<O::Frequency>::from(frequency) + suffix
                - From::<O::Frequency>::from(*cdf);

            // Refill `state` from compressed data if necessary.
            if state < O::min_state() {
                state = (state << word_size) | O::State::from(*compressed_iter.next().ok_or(())?);
            }
        }

        assert!(state == O::min_state() && compressed_iter.next().is_none());
        Ok(uncompressed)
    }
}

struct CompactFrequencyReader12bit<'a> {
    compact: &'a [u16],
    carry: u32,
    cursor: usize,
    remaining: usize,
}

impl<'a> CompactFrequencyReader12bit<'a> {
    fn new(compact: &'a [u16], amt: u16) -> Self {
        // Since `amt` is a `u16` the checked arithmetic gets optimized away
        // unless `usize` is `u16` (only realistic on micro controllers).
        let expected_compact_len = (amt as usize)
            .checked_add(1)
            .unwrap()
            .checked_mul(3)
            .unwrap()
            / 4;
        assert_eq!(compact.len(), expected_compact_len);

        let (carry, cursor) = if amt % 4 == 0 {
            // The initial value of `carry` doesn't matter in this case.
            // The only reason why we don't initialize it to `compact[0]`
            // in all cases is to take into account the edge case
            // `amt == 0`, in which case `compact.len() == 0`.
            (0, usize::max_value())
        } else {
            (compact[0] as u32, 0)
        };

        Self {
            compact,
            carry,
            cursor,
            remaining: amt as usize,
        }
    }
}

impl<'a> Iterator for CompactFrequencyReader12bit<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            self.cursor = self.cursor.wrapping_add((self.remaining % 4 != 0) as usize);

            let shift_l = (self.remaining * 4) % 16;
            let shift_r = 16 - shift_l;

            let current = unsafe {
                // SAFETY: The constructor ensures that `self.compact` is long enough.
                *self.compact.get_unchecked(self.cursor) as u32
            };
            let new = (((self.carry << shift_l) & 0x0fff) | (current >> shift_r)) as u16;
            self.carry = current;

            Some(new)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> ExactSizeIterator for CompactFrequencyReader12bit<'a> {}

#[cfg(test)]
mod test {
    //! TODO: Test with real data: overhead for small compressed words will probably
    //! only be visible when there are lots of symbols with small frequencies.
    //! (also: benchmark both variants on real data)

    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn compact_frequency_reader_12bit() {
        let compact = [0x0123, 0x4567, 0x89ab, 0xcdef];
        let frequencies = CompactFrequencyReader12bit::new(&compact, 5).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0123, 0x0456, 0x0789, 0x0abc, 0x0def]);

        let compact = [0x4567, 0x89ab, 0xcdef];
        let frequencies = CompactFrequencyReader12bit::new(&compact, 4).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0456, 0x0789, 0x0abc, 0x0def]);

        let compact = [0x000a, 0x1234, 0x5678];
        let frequencies = CompactFrequencyReader12bit::new(&compact, 3).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0a12, 0x0345, 0x0678]);

        let compact = [0x00ab, 0xcdef];
        let frequencies = CompactFrequencyReader12bit::new(&compact, 2).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0abc, 0x0def]);

        let compact = [0x0bcd];
        let frequencies = CompactFrequencyReader12bit::new(&compact, 1).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0bcd]);

        let compact = [];
        let frequencies = CompactFrequencyReader12bit::new(&compact, 0).collect::<Vec<_>>();
        assert_eq!(frequencies, []);
    }

    #[test]
    fn entropy_model_from_compact_frequencies() {
        let symbols = [4, -5, 6, 100, 101, 102, 103];
        let frequencies = [0x0123, 0x0342, 0x0054, 0x0500, 0x0001, 0x0109, 0x063d];
        let frequencies_compact = [0x0012, 0x3342, 0x0545, 0x0000, 0x1109];

        let mut expected_table = frequencies
            .iter()
            .scan(0, |accum, f| {
                let old_accum = *accum;
                *accum += f;
                Some(old_accum)
            })
            .zip(symbols.iter().cloned())
            .collect::<Vec<_>>();
        expected_table.push((0x1000, 4));

        let ent = EntropyModel12_16::new(
            CompactFrequencyReader12bit::new(&frequencies_compact, 6),
            &symbols,
        );

        assert_eq!(&*ent.cdf_and_symbols, &expected_table[..]);
    }

    #[test]
    fn entropy_model_12_16() {
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
        let ent = EntropyModel::<O>::new(frequencies[..2].iter().cloned(), &symbols);

        assert_eq!(ent.cdf_and_symbols.len(), symbols.len() + 1);

        assert_eq!(&*ent.cdf_and_symbols, expected_cdf_and_symbols);

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

    fn read_frequencies(compact: &[u16], amt: u16) {
        todo!()
    }
}

// start with remaining == 4, cursor == -1, carry == arbitrary
// start with remaining == 3, cursor == 0, carry == compact[0]
// start with remaining == 2, cursor == 0, carry == compact[0]
// start with remaining == 1, cursor == 0, carry == compact[0]

// _  _  _ |_  _  _ |_  _  _ |_  _  _ |
//            |           |           |

// start with remaining == 4, cursor == -1, carry == arbitrary

// remaining = 3
// cursor = 0
// shift_l = 12
// shift_r = 4
// current = compact[0]
// new = ((carry << 12) & 0x0fff) | (current >> 4) = compact[0] >> 4
// carry = compact[0]
// yield new

// remaining = 2
// cursor = 1
// shift_l = 8
// shift_r = 8
// current = compact[1]
// new = ((carry << 8) & 0x0fff) | (current >> 8) = ((compact[0] << 8) & 0x0f00) | (compact[1] >> 8)
// carry = compact[1]
// yield new

// remaining = 1
// cursor = 2
// shift_l = 4
// shift_r = 12
// current = compact[2]
// new = ((carry << 4) & 0x0fff) | (current >> 12) = ((compact[1] << 4) & 0x0ff0) | (compact[2] >> 12)
// carry = compact[2]
// yield new

// remaining = 0
// cursor = 2
// shift_l = 0
// shift_r = 16
// current = compact[2]
// new = ((carry << 0) & 0x0fff) | (current >> 16) = compact[2] & 0x0fff
// carry = compact[2]
// yield new

// Done.

//         |_  _  _ |_  _  _ |_  _  _ |
//            |           |           |

// start with remaining == 3, cursor == 0, carry == compact[0]

// remaining = 2
// cursor = 1
// shift_l = 8
// shift_r = 8
// current = compact[1]
// new = ((carry << 8) & 0x0fff) | (current >> 8) = ((compact[0] << 8) & 0x0f00) | (compact[1] >> 8)
// carry = compact[1]
// yield new

// remaining = 1
// cursor = 2
// shift_l = 4
// shift_r = 12
// current = compact[2]
// new = ((carry << 4) & 0x0fff) | (current >> 12) = ((compact[1] << 4) & 0x0ff0) | (compact[2] >> 12)
// carry = compact[2]
// yield new

// remaining = 0
// cursor = 2
// shift_l = 0
// shift_r = 16
// current = compact[2]
// new = ((carry << 0) & 0x0fff) | (current >> 16) = compact[2] & 0x0fff
// carry = compact[2]
// yield new

// Done.

//      |_  _  _ |_  _  _ |
//            |           |

// start with remaining == 2, cursor == 0, carry == compact[0]

// remaining = 1
// cursor = 1
// shift_l = 4
// shift_r = 12
// current = compact[1]
// new = ((carry << 4) & 0x0fff) | (current >> 12) = ((compact[0] << 4) & 0x0ff0) | (compact[1] >> 12)
// carry = compact[2]
// yield new

// remaining = 0
// cursor = 1
// shift_l = 0
// shift_r = 16
// current = compact[1]
// new = ((carry << 0) & 0x0fff) | (current >> 16) = compact[1] & 0x0fff
// carry = compact[1]
// yield new

// Done.

//   |_  _  _ |
//            |

// start with remaining == 1, cursor == 0, carry == compact[0]

// remaining = 0
// cursor = 0
// shift_l = 0
// shift_r = 16
// current = compact[0]
// new = ((carry << 0) & 0x0fff) | (current >> 16) = compact[0] & 0x0fff
// carry = compact[0]
// yield new

// Done.
