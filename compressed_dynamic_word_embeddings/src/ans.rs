use num::{
    traits::{ToPrimitive, WrappingAdd, WrappingSub},
    CheckedDiv, One, Zero,
};
use rand::RngCore;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::{hash::Hash, mem::size_of};

/// Error type for [`EncoderModel`] and [`Encoder`].
///
/// [`EncoderModel`]: struct.EncoderModel.html
/// [`Encoder`]: struct.Encoder.html
#[derive(Debug)]
pub enum EncoderError {
    /// A symbol that is not part of the [`EntropyModel`] was intended to be put on the
    /// encoder stack.
    ///
    /// [`EntropyModel`]: struct.EntropyModel.html
    UnknownSymbol,

    /// A symbol that has zero frequency under the [`EntropyModel`] was intended to be put
    /// on the encoder stack.
    ///
    /// [`EntropyModel`]: struct.EntropyModel.html
    ProhibitedSymbol,
}

/// Error type for [`DecoderModel`] and [`Decoder`].
///
/// [`DecoderModel`]: struct.DecoderModel.html
/// [`Decoder`]: struct.Decoder.html
#[derive(Debug)]
pub enum DecoderError {
    /// Not enough compressed data left to decode a symbol.
    EndOfFile,

    /// Some compressed data was left when [`Decoder::finish`] was called.
    ///
    /// [`Decoder::finish`]: struct.Decoder.html#method.finish
    DataLeft,

    /// The decoder did not reach the expected terminal state when [`Decoder::finish`]
    /// was called.
    ///
    /// This can happen if
    /// - some part of the compressed data was corrupted; or
    /// - a small amount of data is left (for large amounts of left-over data,
    ///   [`DataLeft`] would be returned instead); or
    /// - decoding used a different [`EntropyModel`] than encoding.
    ///
    /// [`Decoder::finish`]: struct.Decoder.html#method.finish
    /// [`DataLeft`]: #variant.DataLeft
    /// [`EntropyModel`]: struct.EntropyModel.html
    CorruptedData,
}

/// A Builder for an [`EncoderModel`] or a [`DecoderModel`].
///
/// The type parameter `O` with trait bound [`EntropyModelOptions`] defines the
/// data types of compressed words and of scaled quantized frequencies. Two
/// shortcuts for `EntropyModel`s with common data types are provided in
/// [`EntropyModel12_16`] and [`EntropyModel12_32`].
///
/// An `EntropyModel` is a builder that can be used exactly once, either
/// - to construct an `EncoderModel` via the method [`encoder_model`]; or
/// - to construct a `DecoderModel` via the method [`decoder_model`]; or
/// - to construct a table of the probability mass and cumulative distribution
///   functions by [iterating over the `EntropyModel`](#impl-Iterator); or
/// - to calculate the model's entropy via the method [`entropy`].
///
/// # Example
///
/// See [module level documentation](index.html).
///
/// [`EncoderModel`]: struct.EncoderModel.html
/// [`DecoderModel`]: struct.DecoderModel.html
/// [`EntropyModelOptions`]: trait.EntropyModelOptions.html
/// [`EntropyModel12_16`]: type.EntropyModel12_16.html
/// [`EntropyModel12_32`]: type.EntropyModel12_32.html
/// [`encoder_model`]: #method.encoder_model
/// [`decoder_model`]: #method.decoder_model
/// [`entropy`]: #method.entropy
pub struct EntropyModel<O: EntropyModelOptions, SI, FI> {
    symbols: SI,
    frequencies: FI,
    accum: O::Frequency,
    phantom: PhantomData<O>,
}

/// Shortcut for an [`EntropyModel`] with 12 bit frequencies and `u16` compressed words.
///
/// [`EntropyModel`]: struct.EntropyModel.html
pub type EntropyModel12_16<SI, FI> = EntropyModel<EntropyModelOptions12_16, SI, FI>;

/// Shortcut for an [`EntropyModel`] with 12 bit frequencies and `u32` compressed words.
///
/// [`EntropyModel`]: struct.EntropyModel.html
pub type EntropyModel12_32<SI, FI> = EntropyModel<EntropyModelOptions12_32, SI, FI>;

impl<O, SI, FI> EntropyModel<O, SI, FI>
where
    O: EntropyModelOptions,
    SI: Iterator,
    FI: Iterator<Item = O::Frequency>,
{
    /// Creates a new entropy model, from which one can construct an [`EncoderModel`] or
    /// a [`DecoderModel`].
    ///
    /// The argument `symbols` must yield distinct items, and `frequencies` must yield
    /// nonzero items. Further, either
    /// - `symbols` and `frequencies` must yield the same number of items, in which case
    ///    the frequencies have to add up to
    ///    [`total_frequency::<O>()`](fn.total_frequency.html); or
    /// - `frequencies` may yield exactly one fewer items than `symbols`, in which case
    ///    the frequencies must add up to less than `total_frequency::<O>()` so that some
    ///    nonzero frequency is left for the last symbol.
    ///
    /// Next, you will most likely want to turn the returned `EntropyModel` into either an
    /// `EncoderModel` or a `DecoderModel` by calling either [`encoder_model`] or
    /// [`decoder_model`] on it, respectively.
    ///
    /// # Example
    ///
    /// The generic signature of this method allows constructing an `EntropyModel` from
    /// both an explicit `Vec` or array of frequencies, as well as from a
    /// [`CompactFrequencyReader12bit`]:
    ///
    /// ```
    /// # use compressed_dynamic_word_embeddings::ans::{
    /// #     CompactFrequencyReader12bit,
    /// #     EntropyModel12_32,
    /// # };
    /// let symbols = ['r', 's', 't', 'u', 'v', 'w'];
    ///
    /// // Construct an `EntropyModel` from an explicit list of frequencies.
    /// let explicit_frequencies = [0x0123, 0x0456, 0x0189, 0x02bc, 0x01ef, 0x0453];
    /// let model1 = EntropyModel12_32::new(
    ///     symbols.iter().cloned(),
    ///     explicit_frequencies.iter().cloned()
    /// );
    ///
    /// // Construct an `EntropyModel` from a compact representation of the same
    /// // frequencies. The last frequency will be filled in automatically.
    /// let compact_frequencies = [0x0123, 0x4561, 0x892b, 0xc1ef];
    /// let model2 = EntropyModel12_32::new(
    ///     symbols.iter().cloned(),
    ///     CompactFrequencyReader12bit::new(&compact_frequencies, 5)
    /// );
    ///
    /// assert_eq!(
    ///     model1.collect::<Vec<_>>(),
    ///     model2.collect::<Vec<_>>(),
    /// );
    /// ```
    ///
    /// [`EncoderModel`]: struct.EncoderModel.html
    /// [`DecoderModel`]: struct.DecoderModel.html
    /// [`encoder_model`]: #method.encoder_model
    /// [`decoder_model`]: #method.decoder_model
    /// [`CompactFrequencyReader12bit`]: struct.CompactFrequencyReader12bit.html
    pub fn new(
        symbols: impl IntoIterator<IntoIter = SI, Item = SI::Item>,
        frequencies: impl IntoIterator<Item = O::Frequency, IntoIter = FI>,
    ) -> Self {
        Self {
            symbols: symbols.into_iter(),
            frequencies: frequencies.into_iter(),
            accum: O::Frequency::zero(),
            phantom: PhantomData,
        }
    }

    /// Consumes the model and returns its entropy (expected bits per symbol).
    pub fn entropy(self) -> f32 {
        let f_log2f = self
            .map(|(_, frequency, _)| {
                let frequency: usize = frequency.into();
                let frequency = frequency as f32;
                frequency * frequency.log2()
            })
            .sum::<f32>();

        O::FREQUENCY_BITS as f32 - f_log2f / total_frequency::<O>() as f32
    }

    /// Consumes the model and constructs an [`EncoderModel`]
    ///
    /// The resulting `EncoderModel` may be used for compressing data. In contrast to
    /// `EntropyModel`s, which are builders for single use, `EncoderModel`s can be reused
    /// several times to compress independent chunks of data with a fixed model.
    ///
    /// [`EncoderModel`]: struct.EncoderModel.html
    pub fn encoder_model(self) -> EncoderModel<O, SI::Item>
    where
        SI::Item: Hash + Eq,
    {
        EncoderModel::new(self)
    }

    /// Consumes the model and constructs a [`DecoderModel`].
    ///
    /// The resulting `DecoderModel` may be used for decompressing data. In contrast to
    /// `EntropyModel`s, which are builders for single use, `DecoderModel`s can be reused
    /// several times to decompress independent chunks of data with a fixed model.
    ///
    /// [`DecoderModel`]: struct.DecoderModel.html
    pub fn decoder_model(self) -> DecoderModel<O, SI::Item>
    where
        SI::Item: Default,
    {
        DecoderModel::new(self)
    }
}

/// Iterating over an `EntropyModel` yields tuples `(symbol, frequency, left_sided_cdf)`.
///
/// # Example
///
/// ```
/// # use compressed_dynamic_word_embeddings::ans::EntropyModel12_32;
/// let symbols = ['a', 'b', 'c'];
/// let frequencies = [0x0500, 0x0700];
/// let model = EntropyModel12_32::new(symbols.iter().cloned(), frequencies.iter().cloned());
/// let table = model.collect::<Vec<_>>();
/// assert_eq!(
///     table,
///     [('a', 0x0500, 0), ('b', 0x0700, 0x0500), ('c', 0x0400, 0x0c00)]
/// );
/// ```
impl<O, SI, FI> Iterator for EntropyModel<O, SI, FI>
where
    O: EntropyModelOptions,
    SI: Iterator,
    FI: Iterator<Item = O::Frequency>,
{
    type Item = (SI::Item, O::Frequency, O::Frequency);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.symbols.next(), self.frequencies.next()) {
            (Some(symbol), Some(frequency)) => {
                let old_accum = self.accum;
                self.accum = self.accum + frequency;
                Some((symbol, frequency, old_accum))
            }
            (Some(symbol), None) => {
                // This must be the last entry in `self.symbols`. After this,
                // `self.symbols.next()` must return `None`.
                let old_accum = self.accum;
                self.accum = O::Frequency::one() << O::FREQUENCY_BITS;
                // Use `wrapping sub` because the last entry of the CDF is stored as `0` if
                // `O::FREQUENCY_BITS == 8 * size_of::<O::Frequency>`.
                let frequency = self.accum.wrapping_sub(&old_accum);
                Some((symbol, frequency, old_accum))
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.symbols.size_hint()
    }
}

impl<O, SI, FI> ExactSizeIterator for EntropyModel<O, SI, FI>
where
    O: EntropyModelOptions,
    SI: Iterator + ExactSizeIterator,
    FI: Iterator<Item = O::Frequency>,
{
}

/// High level compression API and a reusable builder for low level [`Encoder`]s.
///
/// An `EncoderModel` provides two APIs for data compression:
/// - A high level method [`encode`] that compresses a complete sequence of
///   symbols into a `Vec` of compressed words in one go.
/// - A lower level method [`encoder`](#method.encoder) that constructs an
///   [`Encoder`], which may be used to compress symbols from different sources
///   with a sequence of method calls. Note that an `Encoder` is a stack, so one
///   has to push symbols onto it in reverse order.
///
/// # Example
///
/// ```
/// # use compressed_dynamic_word_embeddings::ans::EntropyModel12_32;
/// let symbols = ['a', 'b', 'c'];
/// let frequencies = [0x0500, 0x0700];
/// let encoder_model =
///     EntropyModel12_32::new(symbols.iter().cloned(), frequencies.iter().cloned())
///         .encoder_model();
///
/// // Encode some sample data in a single go using the high level API.
/// let compressed1 = encoder_model.encode(&['a', 'b', 'b']).unwrap();
///
/// // Encode the same sample data one symbol at a time using the lower level API.
/// // NOTE: We have to put symbols on the encoder in reverse order (it is a stack).
/// let mut encoder = encoder_model.encoder();
/// encoder.put_symbol(&'b').unwrap();
/// encoder.put_symbol(&'b').unwrap();
/// encoder.put_symbol(&'a').unwrap();
/// let compressed2 = encoder.finish();
///
/// assert_eq!(compressed1, compressed2);
/// ```
///
/// [`Encoder`]: struct.Encoder.html
/// [`encode`]: #method.encode
pub struct EncoderModel<O: EntropyModelOptions, Symbol: Hash + Eq> {
    symbol_to_freq_and_cdf: HashMap<Symbol, (O::Frequency, O::Frequency)>,
}

impl<Symbol: Hash + Eq, O: EntropyModelOptions> EncoderModel<O, Symbol> {
    /// Constructs an `EncoderModel`.
    ///
    /// This method is provided only for completeness and discoverability in the auto
    /// generated documentation. Prefer using [`EntropyModel::encoder_model`] instead, as
    /// it follows a more idiomatic builder pattern. See example in the
    /// [struct level documentation](struct.EncoderModel.html).
    ///
    /// [`EntropyModel::encoder_model`]: struct.EntropyModel.html#method.encoder_model
    pub fn new<SI: Iterator<Item = Symbol>, FI: Iterator<Item = O::Frequency>>(
        builder: EntropyModel<O, SI, FI>,
    ) -> Self {
        let symbol_to_freq_and_cdf = builder
            .map(|(symbol, frequency, cdf)| (symbol, (frequency, cdf)))
            .collect::<HashMap<_, _>>();

        Self {
            symbol_to_freq_and_cdf,
        }
    }

    /// Encode (compress) a sequence of symbols.
    ///
    /// This is a convenience high-level wrapper around the more low level [`encoder`]
    /// API. It compresses a full sequence of symbols in a single go.
    ///
    /// This method abstracts away the stack ("first-in-first-out") nature of the
    /// underlying entropy coding algorithm by pushing symbols on an encoder in reverse
    /// order and reversing the compressed data upon completion. Thus, decompressing the
    /// returned data with [`Decoder::decode`] yields the symbols in forward direction.
    /// See [module level documentation](index.html) for end-to-end encoding-decoding
    /// examples.
    ///
    /// # Returns
    ///
    /// A `Vec` of the compressed message or an [`EncoderError`] if `uncompressed`
    /// yields an unknown or prohibited symbol.
    ///
    /// [`encoder`]: #method.encoder
    /// [`Decoder::decode`]: struct.Decoder.html#method.decode
    /// [`EncoderError`]: enum.EncoderError.html
    pub fn encode<'a, I>(&self, uncompressed: I) -> Result<Vec<O::CompressedWord>, EncoderError>
    where
        I: IntoIterator<Item = &'a Symbol>,
        I::IntoIter: DoubleEndedIterator,
        Symbol: 'a,
    {
        let mut encoder = self.encoder();
        for symbol in uncompressed.into_iter().rev() {
            encoder.put_symbol(symbol)?;
        }

        Ok(encoder.finish())
    }

    /// Low-level API for compressing data symbol by symbol.
    ///
    /// Constructs an [`Encoder`], which can be used to compress data with fine control.
    /// If you just want to encode a sequence of symbols in one go, then the method
    /// [`encode`] provides a more convenient higher level API.
    ///
    /// This method may be called several times to instantiate independent `Encoder`s
    /// that all use the same entropy model.
    ///
    /// # Example
    ///
    /// See [struct level documentation](struct.EncoderModel.html).
    ///
    /// [`Encoder`]: struct.Encoder.html
    /// [`encode`]: #method.encode
    pub fn encoder(&self) -> Encoder<O, Symbol> {
        Encoder::new(self)
    }
}

/// Low level API for encoding (i.e., compressing) data.
///
/// Allows encoding data one symbol at a time. If you just want to encode a
/// sequence of symbols in one go, then the higher level [`EncoderModel::encode`]
/// API is more convenient.
///
/// This is the only struct that exposes the stack ("first-in-first-out") nature of
/// the underlying entropy coding algorithm to the user: symbols put on the stack
/// (via the method [`put_symbol`]) can be decoded from the compressed data in
/// *reverse* order.
///
/// # Example
///
/// See documentation of [`EncoderModel`].
///
/// [`put_symbol`]: #method.put_symbol
/// [`EncoderModel::encode`]: struct.EncoderModel.html#method.encode
/// [`EncoderModel`]: struct.EncoderModel.html
pub struct Encoder<'model, O: EntropyModelOptions, Symbol>
where
    Symbol: Hash + Eq,
{
    model: &'model EncoderModel<O, Symbol>,
    state: O::State,
    compressed: Vec<O::CompressedWord>,
}

impl<'model, Symbol: Hash + Eq, O: EntropyModelOptions> Encoder<'model, O, Symbol> {
    /// Constructs an `Encoder`.
    ///
    /// This method is provided only for completeness and discoverability in the auto
    /// generated documentation. Prefer using [`EncoderModel::encoder`] instead, as it
    /// follows a more idiomatic builder pattern. See example in the struct-level
    /// documentation of [`EncoderModel`].
    ///
    /// [`EncoderModel::encoder`]: struct.EncoderModel.html#method.encoder
    /// [`EncoderModel`]: struct.EncoderModel.html
    pub fn new(model: &'model EncoderModel<O, Symbol>) -> Self {
        Self {
            model,
            state: min_state::<O>(),
            compressed: Vec::new(),
        }
    }

    /// Put a symbol on the compressed data stack.
    ///
    /// This method is deliberately named *`put`*`_symbol` to remind callers that the
    /// underlying entropy coding algorithm is a stack, so symbols have to be put on the
    /// stack in reverse order.
    pub fn put_symbol(&mut self, symbol: &Symbol) -> Result<(), EncoderError> {
        let word_bits = 8 * size_of::<O::CompressedWord>();

        // Invariant at this point: `state >= min_state::<O>()`.
        let (frequency, cdf) = *self
            .model
            .symbol_to_freq_and_cdf
            .get(symbol)
            .ok_or(EncoderError::UnknownSymbol)?;
        let frequency: O::State = From::<O::Frequency>::from(frequency);

        // If emitting a compressed word and then pushing `symbol` on `state` results
        // in `state >= min_state::<O>()`, then do it. If not, then just pushing
        // `symbol` on `state` is guaranteed not to overflow.
        if self.state >= threshold_encoder_state::<O>() * frequency {
            // The following line verifiably gets optimized to just a single "movl"
            // or "movq" instruction on x86.
            let compressed_word = num::cast(self.state % (O::State::one() << word_bits)).unwrap();
            self.state = self.state >> word_bits;

            self.compressed.push(compressed_word);
            // This is the only time where `state < min_state::<O>()`. Thus,
            // the decoder, which operates in the reverse order, can use a check for
            // `state < min_state::<O>()` to see if it has to refill `state`.
        }

        // Push `symbol` on `state`.
        let prefix = self
            .state
            .checked_div(&frequency)
            .ok_or(EncoderError::ProhibitedSymbol)?;
        let suffix = self.state % frequency + From::<O::Frequency>::from(cdf);
        self.state = (prefix << O::FREQUENCY_BITS) | suffix;

        Ok(())
    }

    /// Flush the encoder, reverse the compressed data, and return it.
    ///
    /// The compressed data is reversed before being returned so that it can be
    /// decompressed in forward direction. Note that this only affects the direction
    /// in which the returned compressed data can be read for decompression. Decompression
    /// will still yield the symbols in reverse order compared to the order in which they
    /// have been put on the stack.
    pub fn finish(mut self) -> Vec<O::CompressedWord> {
        let word_bits = 8 * size_of::<O::CompressedWord>();
        let compressed_word1 = num::cast(self.state % (O::State::one() << word_bits)).unwrap();
        let compressed_word2 = num::cast(self.state >> word_bits).unwrap();
        self.compressed.push(compressed_word1);
        self.compressed.push(compressed_word2);

        self.compressed.reverse();
        self.compressed
    }
}

/// High level decompression API and a reusable builder for low level [`Decoder`]s.
///
/// A `DecoderModel` provides two APIs for decompressing a compressed data slice:
/// - A high level method [`decode`] that completely decompresses a compressed data
///   slice into a `Vec` of Symbols.
/// - A lower level method [`decoder`](#method.decoder) that constructs a
///   [`Decoder`], which may be used to decompress the data in batches and/or
///   to process the decompressed symbols eagerly in a streaming manner.
///
/// # Example
///
/// ```
/// # use compressed_dynamic_word_embeddings::ans::EntropyModel12_32;
/// let symbols = ['a', 'b', 'b'];
/// let frequencies = [0x0500, 0x0700];
/// let compressed = [0x0000_0010, 0xb7e7_1100];  // the compressed sequence ['a', 'b', 'b']
/// let expected = ['a', 'b', 'b'];
///
/// let decoder_model =
///     EntropyModel12_32::new(symbols.iter().cloned(), frequencies.iter().cloned())
///         .decoder_model();
///
/// // Decode the sample data into a `Vec<char>` and then verify it.
/// let decompressed_full = decoder_model.decode(&compressed, 3).unwrap();
/// assert_eq!(decompressed_full, expected);
///
/// // Decode the same sample data and verify it on the fly.
/// let mut decoder = decoder_model.decoder(&compressed);
/// decoder
///     .decode(expected.iter(), |decompressed_symbol, expected_symbol| {
///         assert_eq!(decompressed_symbol, expected_symbol)
///     })
///     .unwrap();
/// decoder.finish().unwrap();
/// ```
///
/// [`Decoder`]: struct.Decoder.html
/// [`decode`]: #method.decode
pub struct DecoderModel<O: EntropyModelOptions, Symbol> {
    /// Boxed slice with one more element than the number of distinct symbols
    /// allowed in the entropy model. The first entries of the tuples comprise the
    /// left sided cumulative distribution function, i.e.,
    /// `cdf_and_symbols[i].0 = sum_{j=0}^{i-1} frequency[i]`. The second entries of
    /// the tuples are the associated symbols. The last entry is the tuple
    /// `(O::frequency_from_usize(total_frequency::<O>()), x)` where `x` is arbitrary.
    cdf_and_symbols: Box<[(O::Frequency, Symbol)]>,

    /// Boxed slice of length `total_frequency::<O>()` representing the inverse of the
    /// cumulative distribution function, also called the quantile function. Maps
    /// cumulative frequencies to indices into `cdf_and_symbols`.
    ///
    /// Satisfies the following invariants for all `i`:
    /// - `inverse_cdf[i] < cdf_and_symbols.len() - 1`
    /// - `cdf_and_symbols[inverse_cdf[i]].0 <= inverse_cdf[i]`
    /// - `inverse_cdf[i] < cdf_and_symbols[inverse_cdf[i] + 1].0`
    ///
    /// TODO: Turn this into an unboxed array of fixed size once const generics
    /// allow this.
    inverse_cdf: Box<[O::Frequency]>,
}

impl<O: EntropyModelOptions, Symbol> DecoderModel<O, Symbol> {
    /// Constructs a `DecoderModel`.
    ///
    /// This method is provided only for completeness and discoverability in the auto
    /// generated documentation. Prefer using [`EntropyModel::decoder_model`] instead, as
    /// it follows a more idiomatic builder pattern. See example in the
    /// [struct level documentation](struct.DecoderModel.html).
    ///
    /// [`EntropyModel::decoder_model`]: struct.EntropyModel.html#method.decoder_model
    pub fn new<SI: Iterator<Item = Symbol>, FI: Iterator<Item = O::Frequency>>(
        builder: EntropyModel<O, SI, FI>,
    ) -> Self
    where
        Symbol: Default,
    {
        let mut inverse_cdf = Vec::with_capacity(total_frequency::<O>());
        inverse_cdf.resize(total_frequency::<O>(), O::Frequency::zero());
        let mut inverse_cdf = inverse_cdf.into_boxed_slice();
        let mut index = O::Frequency::zero();

        let cdf_and_symbols = builder
            .map(|(symbol, frequency, accum)| {
                let accum_usize: usize = accum.into();
                let freq_usize: usize = frequency.into();
                for dest_inverse_cdf in &mut inverse_cdf[accum_usize..accum_usize + freq_usize] {
                    *dest_inverse_cdf = index;
                }
                // Increment `index` with `wrapping_add` because otherwise the last addition could
                // undefined behavior (for distributions with an alphabet size of
                // `1 << (8 * size_of::<O::Frequency>()))`).
                index = index.wrapping_add(&O::Frequency::one());
                (accum, symbol)
            })
            .chain(std::iter::once((
                O::Frequency::one() << O::FREQUENCY_BITS,
                Default::default(),
            )))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            cdf_and_symbols,
            inverse_cdf,
        }
    }

    /// Decode (decompress) a compressed data slice into a `Vec` of symbols.
    ///
    /// This is a convenience high-level wrapper around the more low level [`decoder`]
    /// API. It decompresses `amt` symbols from the compressed data `compressed`, checks
    /// if the decoder has reached the expected terminal state, and returns the
    /// decompressed symbols in a `Vec`.
    ///
    /// See [struct level documentation](struct.DecoderModel.html) for an example.
    ///
    /// [`decoder`]: #method.decoder
    /// [`Decoder::decode`]: struct.Decoder.html#method.decode
    /// [`EncoderError`]: enum.EncoderError.html
    pub fn decode(
        &self,
        compressed: &[O::CompressedWord],
        amt: usize,
    ) -> Result<Vec<Symbol>, DecoderError>
    where
        Symbol: Clone,
    {
        let mut decoder = self.decoder(compressed);
        let mut result = Vec::with_capacity(amt);
        decoder.decode(0..amt, |symbol, _| result.push(symbol.clone()))?;
        Ok(result)
    }

    /// Low-level API for decompressing data in a streaming fashion.
    ///
    /// Constructs a [`Decoder`], which can be used to decompress the compressed data
    /// `compressed` in a streaming fashion. If you just want to decode the entire data
    /// into a `Vec` of symbols, then the method [`decode`] provides a more convenient
    /// higher level API.
    ///
    /// This method may be called several times to instantiate independent `Encoder`s
    /// that all use the same entropy model.
    ///
    /// # Example
    ///
    /// See [struct level documentation](struct.DecoderModel.html).
    ///
    /// [`Decoder`]: struct.Decoder.html
    /// [`decode`]: #method.decode
    pub fn decoder<'model, 'data>(
        &'model self,
        compressed: &'data [O::CompressedWord],
    ) -> Decoder<'model, 'data, O, Symbol> {
        Decoder::new(self, compressed)
    }

    /// Generates a random sample from the entropy model.
    ///
    /// This is implemented as a method on `DecoderModel` since drawing random samples
    /// requires the same lookup tables that decompressing requires (essentially,
    /// drawing random samples is similar to decoding a random bitstring).
    pub fn draw_sample(&self, rng: &mut impl RngCore) -> &Symbol {
        let total_freq = Into::<usize>::into(total_frequency::<O>());
        unsafe {
            // SAFETY:
            // - `inverse_cdf` has `total_frequency::<O>()` entries, so indexing
            //   with some value `% total_frequency::<O>()` is always within bounds.
            // - The entries of `inverse_cdf` are guaranteed to be within bounds
            //   for indexing into `cdf_and_symbols`.
            let index = *self
                .inverse_cdf
                .get_unchecked(rng.next_u32() as usize % total_freq);
            &self
                .cdf_and_symbols
                .get_unchecked(Into::<usize>::into(index))
                .1
        }
    }
}

/// Low level API for decoding compressed data.
///
/// Allows decoding compressed data in a streaming fashion. If you just want to decode a
/// compressed data slice into a `Vec` of symbols, then the higher level
/// [`DecoderModel::decode`] API is more convenient.
///
/// # Example
///
/// See documentation of [`DecoderModel`].
///
/// [`DecoderModel::decode`]: struct.DecoderModel.html#method.decode
/// [`DecoderModel`]: struct.DecoderModel.html
pub struct Decoder<'model, 'data, O: EntropyModelOptions, Symbol> {
    model: &'model DecoderModel<O, Symbol>,
    state: O::State,
    cursor: usize,
    compressed: &'data [O::CompressedWord],
}

impl<'model, 'data, O: EntropyModelOptions, Symbol> Decoder<'model, 'data, O, Symbol> {
    /// Constructs an `Decoder`.
    ///
    /// This method is provided only for completeness and discoverability in the auto
    /// generated documentation. Prefer using [`DecoderModel::decoder`] instead, as it
    /// follows a more idiomatic builder pattern. See example in the struct-level
    /// documentation of [`DecoderModel`].
    ///
    /// [`DecoderModel::decoder`]: struct.DecoderModel.html#method.decoder
    /// [`DecoderModel`]: struct.DecoderModel.html
    pub fn new(
        model: &'model DecoderModel<O, Symbol>,
        compressed: &'data [O::CompressedWord],
    ) -> Self {
        let word_size = 8 * size_of::<O::CompressedWord>();
        let state = (O::State::from(compressed[0]) << word_size) | O::State::from(compressed[1]);
        Self {
            model,
            state,
            cursor: 2,
            compressed,
        }
    }

    /// Low-level decoding API.
    ///
    /// Decodes some symbols and calls a callback on each symbol. This is implemented as
    /// an "interior iterator" for maximum efficiency. The provided iterator `driver`
    /// "drives" the iteration. For each item yielded by `driver`, one symbol is decoded
    /// from the encapsulated buffer and `callback` is called with the decoded symbol and
    /// the item yielded by `driver` as arguments.
    ///
    /// # Example
    ///
    /// ```
    /// # use compressed_dynamic_word_embeddings::ans::EntropyModel12_32;
    /// let symbols = ['a', 'b', 'c'];
    /// let frequencies = [0x0500, 0x0700];
    /// let compressed = [0x0000_0010, 0xb7e7_1100];  // the compressed sequence ['a', 'b', 'b']
    ///
    /// let decoder_model =
    ///     EntropyModel12_32::new(symbols.iter().cloned(), frequencies.iter().cloned())
    ///         .decoder_model();
    /// let mut decoder = decoder_model.decoder(&compressed);
    ///
    /// // Decode the first two symbols, turn them into upper case, and write them to a buffer.
    /// let mut buffer = [' '; 2];
    /// decoder
    ///     .decode(buffer.iter_mut(), |symbol, destination| {
    ///         *destination = symbol.to_ascii_uppercase();
    ///     })
    ///     .unwrap();
    /// assert_eq!(buffer, ['A', 'B']);
    ///
    /// // No need to decode to the end if we're just interested in the first part of the data.
    /// ```
    pub fn decode<I: Iterator>(
        &mut self,
        driver: I,
        mut callback: impl FnMut(&Symbol, I::Item),
    ) -> Result<(), DecoderError> {
        // Dereference all fields just once before we enter the hot loop, and then
        // never dereference them in the loop. This turns out to improve
        // performance.
        let mut cursor = self.cursor;
        let mut state = self.state;
        let compressed = self.compressed;
        let cdf_and_symbols = &*self.model.cdf_and_symbols;
        let inverse_cdf = &*self.model.inverse_cdf;

        for dest in driver {
            // Pop `symbol` off `state`.
            let suffix = state % (O::State::one() << O::FREQUENCY_BITS);
            let index = unsafe {
                // SAFETY: `inverse_cdf` has length `total_frequency::<O>()`, and
                // `O::state_as_frequency` returns a value smaller than
                // `total_frequency::<O>()`.
                *inverse_cdf.get_unchecked(suffix.to_usize().unwrap())
            };
            let index = Into::<usize>::into(index);
            let ((cdf, symbol), next_cdf) = unsafe {
                // SAFETY: `index` comes from `inverse_cdf`, which only takes values
                // `< cdf_and_symbols.len() -1`.
                (
                    cdf_and_symbols.get_unchecked(index).clone(),
                    cdf_and_symbols.get_unchecked(index + 1).0,
                )
            };
            // We use `wrapping sub` because the last entry of the CDF is stored as `0` if
            // `O::FREQUENCY_BITS == 8 * size_of::<O::Frequency>`.
            let frequency = next_cdf.wrapping_sub(cdf);

            callback(symbol, dest);

            // Update `state`.
            let word_size = 8 * size_of::<O::CompressedWord>();
            state = (state >> O::FREQUENCY_BITS) * From::<O::Frequency>::from(frequency) + suffix
                - From::<O::Frequency>::from(*cdf);

            // Refill `state` from compressed data if necessary.
            if state < min_state::<O>() {
                let next_word = *compressed.get(cursor).ok_or(DecoderError::EndOfFile)?;
                state = (state << word_size) | O::State::from(next_word);
                cursor += 1;
            }
        }

        self.cursor = cursor;
        self.state = state;

        Ok(())
    }

    /// Decodes `amt` symbols and discards them.
    ///
    /// This method is deliberately called *skip* rather than *seek* to remind the caller
    /// that skipping ahead requires decompression, so it requires `O(amt)` time.
    pub fn skip(&mut self, amt: usize) -> Result<(), DecoderError> {
        self.decode(0..amt, |_, _| ())
    }

    /// Checks if the `Decoder` is in a valid "end" state and then drops it.
    ///
    /// This provides a cheap and effective check for data integrity. A random (i.e.,
    /// non-adversarial) corruption of the compressed data will likely cause this method
    /// to return an error when called after decoding the expected number of symbols.
    ///
    /// If you don't want to read a compressed stream to the end and want to stop
    /// early instead, then don't call this method and just drop the decoder
    /// normally instead. It is not possible to verify data integrity without
    /// reading to the end of the compressed stream.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a `Err(e)` where `e` is either [`DecoderError::DataLeft`]
    /// or [`DecoderError::CorruptedData`].
    ///
    /// [`DecoderError::DataLeft`]: enum.DecoderError.html#variant.DataLeft
    /// [`DecoderError::CorruptedData`]: enum.DecoderError.html#variant.CorruptedData
    pub fn finish(self) -> Result<(), DecoderError> {
        if self.cursor != self.compressed.len() {
            Err(DecoderError::DataLeft)
        } else if self.state != min_state::<O>() {
            Err(DecoderError::CorruptedData)
        } else {
            Ok(())
        }
    }
}

impl<O: EntropyModelOptions, Symbol> std::fmt::Debug for DecoderModel<O, Symbol>
where
    Symbol: std::fmt::Display,
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

/// A trait that defines the data types of compressed words, and the type and
/// accuracy and of scaled quantized frequencies used in an [`EntropyModel`],
/// [`EncoderModel`], [`DecoderModel`], [`Encoder`], or [`Decoder`].
///
/// Two example implementations are defined for [`EntropyModelOptions12_16`]
/// and [`EntropyModelOptions12_32`].
///
/// # Safety
///
/// This trait is declared unsafe because the following requirements are not
/// statically enforced:
/// - [`State`] must be big enough to hold at least two [`CompressedWord`s], i.e.,
///   `std::mem::size_of::<State>() >= 2 * std::mem::size_of<CompressedWord>()`.
///   (Note: in practice, there is no reason to go beyond equality in this bound.)
/// - [`Frequency`] must not be larger than [`CompressedWord`], i.e.,
///   `std::mem::size_of::<Frequency>() <= std::mem::size_of<CompressedWord>()`.
/// - [`FREQUENCY_BITS`] must be positive and at most the number of bits in
///   [`Frequency`], i.e., `FREQUENCY_BITS <= 8 * std::mem::size_of::<Frequency>()`.
///
/// [`EntropyModel`]: struct.EntropyModel.html
/// [`EncoderModel`]: struct.EncoderModel.html
/// [`DecoderModel`]: struct.DecoderModel.html
/// [`Encoder`]: struct.Encoder.html
/// [`Decoder`]: struct.Decoder.html
/// [`EntropyModelOptions12_16`]: struct.EntropyModelOptions12_16.html
/// [`EntropyModelOptions12_32`]: struct.EntropyModelOptions12_32.html
/// [`State`]: #associatedtype.State
/// [`CompressedWord`]: #associatedtype.CompressedWord
/// [`CompressedWord`s]: #associatedtype.CompressedWord
/// [`FREQUENCY_BITS`]: #associatedconstant.FREQUENCY_BITS
/// [`Frequency`]: #associatedtype.Frequency
pub unsafe trait EntropyModelOptions {
    /// The type of compressed data words that get emitted by the encoder and read
    /// in by the decoder.
    type CompressedWord: num::PrimInt + num::Unsigned;

    /// Type of some internal state of the encoder and decoder.
    ///
    /// Must be able hold two `CompressedWord`s.
    type State: num::PrimInt + num::Unsigned + From<Self::Frequency> + From<Self::CompressedWord>;

    /// The type used to describe quantized scaled frequencies.
    ///
    /// Must not be larger than [`CompressedWord`]. For any values, only the least
    /// significant [`FREQUENCY_BITS`] bits will ever be nonzero, except when
    /// representing the total frequency (last entry of the cumulative distribution
    /// function), which is `1 << FREQUENCY_BITS` if representable by this type or zero
    /// otherwise.
    ///
    /// [`CompressedWord`s]: #associatedtype.CompressedWord
    /// [`FREQUENCY_BITS`]: #associatedconstant.FREQUENCY_BITS
    type Frequency: num::PrimInt + num::Unsigned + Into<usize> + WrappingAdd + WrappingSub;

    /// Number of bits used for quantized scaled frequencies.
    ///
    /// Must be positive and no larger than `8 * std::mem::size_of_<Frequency>()`.
    const FREQUENCY_BITS: usize;
}

/// Convenience function for the total scaled quantized frequency.
///
/// This is defined as `1 << O::FREQUENCY_BITS`. It's the number to which all
/// scaled quantized frequencies in an entropy model have to add up. The result
/// is of type `usize` rather than [`O::Frequency`] since the latter is only
/// guaranteed to hold values up to `(1 << O::FREQUENCY_BITS) - 1`.
///
/// [`O::Frequency`]: trait.EntropyModelOptions.html#associatedtype.Frequency
#[inline(always)]
pub fn total_frequency<O: EntropyModelOptions>() -> usize {
    1 << O::FREQUENCY_BITS
}

/// If the decoder state falls this value, then the state has to be refilled with
/// a word from the compressed data. This is also the initial state of the encoder
/// (and therefore also the expected terminal state of the decoder).
#[inline(always)]
fn min_state<O: EntropyModelOptions>() -> O::State {
    O::State::one() << (8 * size_of::<O::CompressedWord>())
}

/// If the encoder state reaches this threshold times the scaled quantized frequency
/// of the next symbol, then a compressed word should be popped off the encoder
/// state and flushed to the main stack of compressed words.
#[inline(always)]
fn threshold_encoder_state<O: EntropyModelOptions>() -> O::State {
    O::State::one() << (16 * size_of::<O::CompressedWord>() - O::FREQUENCY_BITS)
}

/// Zero sized marker struct that implements [`EntropyModelOptions`] for
/// [`EntropyModel`s] with 12 bit frequency quantization and `u16` compressed words.
///
/// See also [`EntropyModelOptions12_32`].
///
/// [`EntropyModelOptions`]: trait.EntropyModelOptions.html
/// [`EntropyModel`s]: struct.EntropyModel.html
/// [`EntropyModelOptions12_32`]: struct.EntropyModelOptions12_32.html
pub struct EntropyModelOptions12_16;

unsafe impl EntropyModelOptions for EntropyModelOptions12_16 {
    const FREQUENCY_BITS: usize = 12;
    type Frequency = u16;
    type CompressedWord = u16;
    type State = u32;
}

/// Zero sized marker struct that implements [`EntropyModelOptions`] for
/// [`EntropyModel`s] with 12 bit frequency quantization and `u32` compressed words.
///
/// See also [`EntropyModelOptions12_16`].
///
/// [`EntropyModelOptions`]: trait.EntropyModelOptions.html
/// [`EntropyModel`s]: struct.EntropyModel.html
/// [`EntropyModelOptions12_16`]: struct.EntropyModelOptions12_16.html
pub struct EntropyModelOptions12_32;

unsafe impl EntropyModelOptions for EntropyModelOptions12_32 {
    const FREQUENCY_BITS: usize = 12;
    type Frequency = u16;
    type CompressedWord = u32;
    type State = u64;
}

/// TODO: move to a separate module
pub struct CompactFrequencyReader12bit<'a> {
    compact: &'a [u16],
    carry: u32,
    cursor: usize,
    remaining: usize,
}

impl<'a> CompactFrequencyReader12bit<'a> {
    pub fn new(compact: &'a [u16], amt: u16) -> Self {
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
    fn decoder_model_from_compact_frequencies() {
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
        expected_table.push((0x1000, 0));

        let dec = EntropyModel::<EntropyModelOptions12_32, _, _>::new(
            symbols.iter().cloned(),
            CompactFrequencyReader12bit::new(&frequencies_compact, 6),
        )
        .decoder_model();

        assert_eq!(&*dec.cdf_and_symbols, &expected_table[..]);
    }

    #[test]
    fn entropy_model_12_16() {
        test_run::<EntropyModelOptions12_16, _>(
            &[4i16, -5, 6],
            &[500, 2000, 1596],
            &[(0, 4), (500, -5), (2500, 6), (4096, 0)],
        );
        test_run::<EntropyModelOptions12_32, _>(
            &[4i16, -5, 6],
            &[500, 2000, 1596],
            &[(0, 4), (500, -5), (2500, 6), (4096, 0)],
        );
    }

    fn test_run<O: EntropyModelOptions, Symbol: Default + Clone>(
        symbols: &[Symbol],
        frequencies: &[O::Frequency],
        expected_cdf_and_symbols: &[(O::Frequency, Symbol)],
    ) where
        O::Frequency: Eq + std::fmt::Debug,
        Symbol: Eq + std::fmt::Debug + std::hash::Hash,
    {
        let encoder_model =
            EntropyModel::<O, _, _>::new(symbols.iter().cloned(), frequencies.iter().cloned())
                .encoder_model();
        let decoder_model =
            EntropyModel::<O, _, _>::new(symbols.iter().cloned(), frequencies.iter().cloned())
                .decoder_model();

        assert_eq!(decoder_model.cdf_and_symbols.len(), symbols.len() + 1);
        assert_eq!(&*decoder_model.cdf_and_symbols, expected_cdf_and_symbols);

        for (i, freq_pair) in decoder_model.cdf_and_symbols.windows(2).enumerate() {
            for index in &decoder_model.inverse_cdf[freq_pair[0].0.into()..freq_pair[1].0.into()] {
                assert_eq!(Into::<usize>::into(*index), i);
            }
        }

        let entropy =
            EntropyModel::<O, _, _>::new(symbols.iter().cloned(), frequencies.iter().cloned())
                .entropy();
        let true_entropy = -(500.0 / 4096.0) * (500.0f32 / 4096.0).log2()
            - (2000.0 / 4096.0) * (2000.0f32 / 4096.0).log2()
            - (1596.0 / 4096.0) * (1596.0f32 / 4096.0).log2();
        assert!((entropy - true_entropy).abs() < 1e-6);

        let mut rng = StdRng::seed_from_u64(123);
        let samples = (0..100000)
            .map(|_| decoder_model.draw_sample(&mut rng).clone())
            .collect::<Vec<_>>();

        let mut counts = HashMap::new();
        for sample in &samples {
            counts
                .entry(sample)
                .and_modify(|c| *c += 1)
                .or_insert(1usize);
        }

        assert_eq!(counts.len(), 3);
        // Check that observed frequencies are within 1% of expected frequencies.
        for (symbol, freq) in symbols.iter().zip(frequencies) {
            let expected = Into::<usize>::into(*freq) * samples.len() / 4096;
            let observed = counts[symbol];
            assert!(observed > 99 * expected / 100);
            assert!(observed < 101 * expected / 100);
        }

        let compressed = encoder_model.encode(&samples[..]).unwrap();
        let expected_bitlength = entropy * samples.len() as f32;
        let observed_bitlength = 8 * size_of::<O::CompressedWord>() * compressed.len();
        dbg!(expected_bitlength, observed_bitlength);
        assert!(observed_bitlength as f32 > 0.99 * expected_bitlength);
        assert!((observed_bitlength as f32) < 1.01 * expected_bitlength);

        let mut decoded = Vec::with_capacity(samples.len());
        let mut decoder = decoder_model.decoder(&*compressed);
        decoder
            .decode(0..samples.len(), |symbol, _| decoded.push(symbol.clone()))
            .unwrap();
        decoder.finish().unwrap();
        assert!(decoded == samples);
    }
}
