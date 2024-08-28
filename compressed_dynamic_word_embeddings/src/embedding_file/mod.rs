use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use constriction::{stream::Decode, Seek, UnwrapInfallible};

use super::random_access_reader::RandomAccessReader;
use crate::u12::unpack_u12s;

pub mod builder;

type Cursor<'data> = constriction::backends::Cursor<u16, &'data [u16]>;
type ReversedCursor<'data> = constriction::backends::Reverse<Cursor<'data>>;
type DecoderModel = constriction::stream::model::SmallNonContiguousLookupDecoderModel<i16>;
type DecoderModelView<'a> = constriction::stream::model::SmallNonContiguousLookupDecoderModel<
    i16,
    &'a [(u16, i16)],
    &'a [u16],
>;
type Decoder<'data> = constriction::stream::stack::SmallAnsCoder<ReversedCursor<'data>>;

pub const HEADER_SIZE: u32 = (std::mem::size_of::<FileHeader>() / 4) as u32;

pub struct EmbeddingFile {
    raw_data: Box<[u32]>,
    decoder_models: Box<[DecoderModel]>,
    jump_points_per_timestep: usize,
    compressed_data_start: usize,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct FileHeader {
    pub magic: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub file_size: u32,
    pub jump_table_address: u32,
    pub num_timesteps: u32,
    pub vocab_size: u32,
    pub embedding_dim: u32,
    pub jump_interval: u32,
    pub scale_factor: f32,
}

impl FileHeader {
    /// # Safety
    ///
    /// `data.len()` must be at least `HEADER_SIZE`
    #[inline(always)]
    pub unsafe fn memory_map_unsafe(data: &[u32]) -> &Self {
        #[allow(unused_unsafe)] // See Rust RFC #2585.
        unsafe {
            // SAFETY: safe according to contract of this method
            let header_slice = data.get_unchecked(0..HEADER_SIZE as usize);

            // SAFETY: `FileHeader` is `repr(C)` and has the same size and alignment as
            // `[u32; HEADER_SIZE]`
            let ptr = header_slice.as_ptr();
            &*(ptr as *const FileHeader)
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
struct JumpPointer {
    offset: u32,
    state: u32,
}

pub struct Timestep<'data, 'model> {
    decoder: Decoder<'data>,
    model: DecoderModelView<'model>,
    jump_table: &'data [JumpPointer],
    word_index: u32,
    embedding_dim: u32,
    jump_interval: u32,
}

impl EmbeddingFile {
    pub fn new(data: Box<[u32]>) -> Result<Self, ()> {
        if data.len() < HEADER_SIZE as usize {
            return Err(());
        }

        let header = unsafe {
            // SAFETY: We checked above that data.len() >= HEADER_SIZE
            FileHeader::memory_map_unsafe(&data)
        };

        let embeddings_size = header.vocab_size * header.embedding_dim;

        if header.magic != 0x6577_6400
            || header.major_version != 1
            || header.file_size as usize != data.len()
            || header.jump_table_address <= HEADER_SIZE
            || header.jump_table_address > header.file_size
            || header.num_timesteps < 2
            || embeddings_size == 0
            || header.jump_interval == 0
        {
            return Err(());
        }

        let entropy_models_section =
            get_u16_slice(&data[HEADER_SIZE as usize..header.jump_table_address as usize]);

        let mut remainder = entropy_models_section;
        let mut decoder_models = Vec::with_capacity(header.num_timesteps as usize);
        for _ in 0..header.num_timesteps {
            let (model, r) = deserialize_decoder_model(remainder)?;
            remainder = r;
            decoder_models.push(model);
        }
        if remainder.len() > 1 {
            // At most one padding entry allowed.
            Err(())?
        }

        let jump_points_per_timestep =
            ((header.vocab_size + header.jump_interval - 1) / header.jump_interval) as usize;
        let compressed_data_start = header.jump_table_address as usize
            + 2 * header.num_timesteps as usize * jump_points_per_timestep;

        Ok(EmbeddingFile {
            raw_data: data,
            decoder_models: decoder_models.into(),
            jump_points_per_timestep,
            compressed_data_start,
        })
    }

    pub fn from_reader(mut reader: impl Read) -> Result<EmbeddingFile, ()> {
        let mut buf = vec![0; HEADER_SIZE as usize];
        reader
            .read_u32_into::<LittleEndian>(&mut buf[..])
            .map_err(|_| ())?;

        let header = unsafe {
            // SAFETY: We made sure that buf.len() == HEADER_SIZE
            FileHeader::memory_map_unsafe(&buf)
        };
        let file_size = header.file_size;

        buf.reserve_exact((file_size - HEADER_SIZE) as usize);
        for _ in HEADER_SIZE..file_size {
            buf.push(reader.read_u32::<LittleEndian>().map_err(|_| ())?);
        }

        Self::new(buf.into())
    }

    pub fn into_random_access_reader(self) -> RandomAccessReader {
        RandomAccessReader::new(self)
    }

    pub fn into_inner(self) -> Box<[u32]> {
        self.raw_data
    }

    #[inline(always)]
    pub fn header(&self) -> &FileHeader {
        unsafe {
            // SAFETY: the ensures checks that `self.raw_data` is big enough.
            FileHeader::memory_map_unsafe(&self.raw_data)
        }
    }

    /// Writes the compressed data to a writer and flushes it.
    ///
    /// If the goal is to write the data to a file then a `std::io::BufWriter`
    /// should be used as this function writes the data in lots of tiny chunks of
    /// just four bytes.
    pub fn write_to(&self, mut writer: impl Write) -> std::io::Result<()> {
        for i in self.as_slice_u32() {
            writer.write_u32::<LittleEndian>(*i)?;
        }
        writer.flush()
    }

    pub fn timestep(&self, t: u32) -> Result<Timestep, ()> {
        let header = self.header();
        if t as usize >= self.decoder_models.len() {
            Err(())
        } else {
            let jump_table_start =
                header.jump_table_address as usize + 2 * self.jump_points_per_timestep * t as usize;

            let jump_table = unsafe {
                // SAFETY: Transmuting from `&[u32]` of even length to `&[JumpPointer]` is safe,
                // because `JumpPointer` is `repr(C)` and contains exactly two `u32`s.
                // See also https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
                let jump_table_data = &self.raw_data
                    [jump_table_start..jump_table_start + 2 * self.jump_points_per_timestep];
                let ptr = jump_table_data.as_ptr();
                std::slice::from_raw_parts(ptr as *const JumpPointer, self.jump_points_per_timestep)
            };

            let compressed = get_u16_slice(&self.raw_data[self.compressed_data_start..]);

            Ok(Timestep::new(
                &self.decoder_models[t as usize],
                jump_table,
                compressed,
                header.embedding_dim,
                header.jump_interval,
            ))
        }
    }

    pub fn as_slice_u32(&self) -> &[u32] {
        &self.raw_data
    }
}

fn deserialize_decoder_model(serialized: &[u16]) -> Result<(DecoderModel, &[u16]), ()> {
    let num_symbols = serialized[0];
    let packed_size = 3 * num_symbols as usize / 4;

    // Extract remainder first to check most constrained bounds.
    let remainder = serialized
        .get(1 + num_symbols as usize + packed_size..)
        .ok_or(())?;
    let symbols = &serialized[1..1 + num_symbols as usize];
    let packed_frequencies =
        &serialized[1 + num_symbols as usize..1 + num_symbols as usize + packed_size];

    let model = DecoderModel::from_symbols_and_nonzero_fixed_point_probabilities(
        symbols.iter().map(|&s| s as i16),
        unpack_u12s(packed_frequencies, num_symbols - 1),
        true,
    )
    .expect("Invalid entropy model.");

    Ok((model, remainder))
}

impl<'data, 'model> Timestep<'data, 'model> {
    fn new(
        decoder_model: &'model DecoderModel,
        jump_table: &'data [JumpPointer],
        compressed: &'data [u16],
        embedding_dim: u32,
        jump_interval: u32,
    ) -> Self {
        let JumpPointer { offset, state } = jump_table[0];
        let cursor =
            Cursor::new_at_pos(compressed, offset as usize).expect("Jump position out of bounds");

        let decoder = Decoder::from_raw_parts(constriction::backends::Reverse(cursor), state);

        Timestep {
            decoder,
            model: decoder_model.as_view(),
            jump_table,
            word_index: 0,
            embedding_dim,
            jump_interval,
        }
    }

    pub fn into_inner(self) -> (Decoder<'data>, DecoderModelView<'model>) {
        (self.decoder, self.model)
    }
}

pub trait TimestepReader {
    fn read_single_embedding_vector<I: Iterator>(
        &mut self,
        dest_iter: I,
        callback: impl FnMut(i16, I::Item),
    ) -> Result<(), ()>;

    fn jump_to(&mut self, word_index: u32) -> Result<(), ()>;
}

impl<'data, 'model> TimestepReader for Timestep<'data, 'model> {
    fn read_single_embedding_vector<I: Iterator>(
        &mut self,
        dest_iter: I,
        mut callback: impl FnMut(i16, I::Item),
    ) -> Result<(), ()> {
        let decoder = &mut self.decoder;
        let model = self.model;
        for dest in dest_iter {
            let symbol = decoder.decode_symbol(model).unwrap_infallible();
            callback(symbol, dest);
        }
        self.word_index += 1;
        Ok(())
    }

    fn jump_to(&mut self, word_index: u32) -> Result<(), ()> {
        let jump_point = word_index / self.jump_interval;
        if word_index < self.word_index || jump_point != self.word_index / self.jump_interval {
            let JumpPointer { offset, state } = self.jump_table[jump_point as usize];
            self.decoder.seek((offset as usize, state))?;
            self.word_index = jump_point * self.jump_interval;
        }

        // Note that just calling `decode_iid_symbols` won't do anything because it's lazy.
        // We actually actually have to drain the iterator.
        for symbol in self.decoder.decode_iid_symbols(
            self.embedding_dim as usize * (word_index - self.word_index) as usize,
            self.model,
        ) {
            symbol.unwrap_infallible();
        }
        self.word_index = word_index;

        Ok(())
    }
}

fn get_u16_slice(data: &[u32]) -> &[u16] {
    unsafe {
        // Transmuting from `&[u32]` to `&[u16]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = data.as_ptr();
        std::slice::from_raw_parts(ptr as *const u16, 2 * data.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_file() {
        let mut header_section = vec![
            0x65776400u32, // magic
            1,             // major_version
            0,             // minor_version
            35,            // file_size
            19,            // jump_table_address
            3,             // num_timesteps
            4,             // vocab_size
            5,             // embedding_dim
            3,             // jump_interval
            1,             // scale_factor (is logically an f32)
        ];
        for i in header_section.iter_mut() {
            *i = i.to_le();
        }

        let mut entropy_models_definition_section = [
            2u16, // num_symbols (time step 1)
            1, 2,      // symbols
            0x0abc, // frequencies
            4u16,   // num_symbols (time step 2)
            1, 2, 3, 4, // symbols
            0x0008, 0xbc12, 0x3456, // frequencies
            3u16,   // num_symbols (time step 3)
            1, 2, 3, // symbols
            0x0012, 0x3456, // frequencies
        ];
        for i in entropy_models_definition_section.iter_mut() {
            *i = i.to_le();
        }

        let mut jump_table_section = [
            0u32,
            0x1234_5678, // t=0, i=0
            2,
            0x1234_5678, // t=0, i=3
            5,
            0x1234_5678, // t=1, i=0
            7,
            0x1234_5678, // t=1, i=3
            3,
            0x1234_5678, // t=2, i=0
            4,
            0x1234_5678, // t=2, i=3
        ];
        for i in jump_table_section.iter_mut() {
            *i = i.to_le();
        }

        let compressed_data_section = [
            1u16, 2, 3, 4, 5, 6, 7, 8, // Dummy content (we won't actually decode it).
        ];

        let mut data = header_section;
        for chunk in entropy_models_definition_section.chunks_exact(2) {
            data.push(chunk[0] as u32 | ((chunk[1] as u32) << 16));
        }
        data.extend_from_slice(&jump_table_section[..]);
        for chunk in compressed_data_section.chunks_exact(2) {
            data.push(chunk[0] as u32 | ((chunk[1] as u32) << 16));
        }

        let file = EmbeddingFile::new(data.into()).unwrap();
        assert_eq!(file.header().file_size, 35);

        let mut data = file.into_inner();
        data[4] -= 1; // Invalidate the jump_table address.
        assert!(EmbeddingFile::new(data).is_err());
    }
}
