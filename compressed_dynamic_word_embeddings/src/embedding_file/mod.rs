use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{ans::DecoderModel12_16, u12::unpack_u12s};

use super::random_access_reader::RandomAccessReader;
use super::tensors::RankThreeTensorView;

pub mod builder;

type EntropyModel<SI, FI> = crate::ans::EntropyModel12_16<SI, FI>;
type DecoderModel = crate::ans::DecoderModel12_16<i16>;
type Decoder<'model, 'data> = crate::ans::Decoder12_16<'model, 'data, i16>;

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
    /// SAFETY: `data.len()` must be at least `HEADER_SIZE`
    #[inline(always)]
    pub unsafe fn memory_map_unsafe(data: &[u32]) -> &Self {
        unsafe {
            // SAFETY: safe according to contract of this method
            let header_slice = data.get_unchecked(0..HEADER_SIZE as usize);

            // SAFETY: `FileHeader` is `repr(C)` and has the same alignment as
            // `[u32; HEADER_SIZE]`
            let ptr = header_slice.as_ptr();
            &*(ptr as *const FileHeader)
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct JumpPointer {
    offset: u32,
    state: u32,
}

pub struct Timestep<'a> {
    decoder: Decoder<'a, 'a>,
    jump_table: &'a [JumpPointer],
    word_index: usize,
    embedding_dim: usize,
    jump_interval: usize,
}

impl EmbeddingFile {
    pub fn new(data: Box<[u32]>) -> Result<Self, ()> {
        if data.len() < HEADER_SIZE as usize {
            return Err(());
        }

        let header = unsafe {
            // SAFETY: We checked above that data.len() >= HEADER_SIZE
            FileHeader::memory_map_unsafe(&*data)
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
        let decoder_models = (0..header.num_timesteps)
            .map(|_| {
                let (model, r) = deserialize_decoder_model(remainder);
                remainder = r;
                model
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        assert!(remainder.len() <= 1); // At most one padding entry allowed.

        let jump_points_per_timestep =
            ((header.vocab_size + header.jump_interval - 1) / header.jump_interval) as usize;
        let compressed_data_start = header.jump_table_address as usize
            + 2 * header.num_timesteps as usize * jump_points_per_timestep;

        Ok(EmbeddingFile {
            raw_data: data,
            decoder_models,
            jump_points_per_timestep,
            compressed_data_start,
        })
    }

    pub fn from_reader(mut reader: impl Read) -> Result<EmbeddingFile, ()> {
        let mut buf = Vec::new();
        buf.resize(HEADER_SIZE as usize, 0);
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
            FileHeader::memory_map_unsafe(&*self.raw_data)
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
                header.embedding_dim as usize,
                header.jump_interval as usize,
            ))
        }
    }

    pub fn as_slice_u32(&self) -> &[u32] {
        &self.raw_data
    }
}

fn deserialize_decoder_model(serialized: &[u16]) -> (DecoderModel12_16<i16>, &[u16]) {
    let num_symbols = serialized[0];
    let packed_size = 3 * num_symbols as usize / 4;
    assert!(serialized.len() >= 1 + num_symbols as usize + packed_size);

    let symbols = &serialized[1..1 + num_symbols as usize];
    let packed_frequencies =
        &serialized[1 + num_symbols as usize..1 + num_symbols as usize + packed_size];
    let remainder = &serialized[1 + num_symbols as usize + packed_size..];

    let model = EntropyModel::new(
        symbols.iter().map(|&s| s as i16),
        unpack_u12s(packed_frequencies, num_symbols - 1),
    )
    .decoder_model();

    (model, remainder)
}

impl<'a> Timestep<'a> {
    fn new(
        decoder_model: &'a DecoderModel,
        jump_table: &'a [JumpPointer],
        compressed: &'a [u16],
        embedding_dim: usize,
        jump_interval: usize,
    ) -> Self {
        let JumpPointer { offset, state } = jump_table[0];

        Timestep {
            decoder: decoder_model.decoder_with_history(compressed, offset as usize, state),
            jump_table,
            word_index: 0,
            embedding_dim,
            jump_interval,
        }
    }
    // fn new(embedding_data: &'a EmbeddingData, addr: u32, num_chunks: u32) -> Result<Self, ()> {
    //     todo!()
    // let u16_slice = get_u16_slice(embedding_data.raw_data.get(addr as usize..).ok_or(())?);

    // let num_symbols = u16_slice[0];
    // let symbols = &u16_slice[1..1 + num_symbols as usize];
    // let packed_size = (3 * num_symbols as usize) / 4;
    // let frequencies = expand_u12s(
    //     &u16_slice[1 + num_symbols as usize..num_symbols as usize + 1 + packed_size],
    //     num_symbols - 1,
    // );
    // let model_description_length = (1 + num_symbols as usize + packed_size + 1) / 2;

    // let decoder_model =
    //     EntropyModel::new(symbols.iter().map(|&x| x as i16), frequencies).decoder_model();

    // let start_addr = addr as usize + model_description_length;
    // let chunk_addresses = embedding_data
    //     .raw_data
    //     .get(start_addr..start_addr + num_chunks as usize)
    //     .ok_or(())?;

    // Ok(Timestep {
    //     decoder_model,
    //     chunk_addresses,
    //     embedding_data,
    // })
    // }
}

pub trait TimestepReader {
    fn read_single_embedding_vector<I: Iterator>(
        &mut self,
        dest_iter: I,
        callback: impl FnMut(i16, I::Item),
    ) -> Result<(), ()>;

    fn jump_to(&mut self, word_index: usize) -> Result<(), ()>;
}

impl<'a> TimestepReader for Timestep<'a> {
    fn read_single_embedding_vector<I: Iterator>(
        &mut self,
        dest_iter: I,
        mut callback: impl FnMut(i16, I::Item),
    ) -> Result<(), ()> {
        self.decoder
            .streaming_decode(dest_iter, |&symbol, dest| callback(symbol, dest))
            .map_err(|_| ())?;
        self.word_index += 1;
        Ok(())
    }

    fn jump_to(&mut self, word_index: usize) -> Result<(), ()> {
        let jump_point = word_index / self.jump_interval;
        if word_index < self.word_index || jump_point != self.word_index / self.jump_interval {
            let JumpPointer { offset, state } = self.jump_table[jump_point];
            self.decoder.jump_to(offset as usize, state);
            self.word_index = jump_point * self.jump_interval;
        }

        self.decoder
            .skip(self.embedding_dim * (word_index - self.word_index))
            .map_err(|_| ())?;
        self.word_index = word_index;

        Ok(())
    }
}

#[cfg(target_endian = "little")]
fn get_i8_slice(data: &[u32]) -> &[i8] {
    unsafe {
        // Transmuting from `&[u32]` to `&[i8]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = data.as_ptr();
        std::slice::from_raw_parts(ptr as *const i8, 4 * data.len())
    }
}

// TODO: should no longer be needed.
#[cfg(target_endian = "little")]
fn get_u8_slice(data: &[u32]) -> &[u8] {
    unsafe {
        // Transmuting from `&[u32]` to `&[u8]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        let ptr = data.as_ptr();
        std::slice::from_raw_parts(ptr as *const u8, 4 * data.len())
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
        let data = vec![
            255, // magic
            0,   // major_version
            0,   // minor_version
            16,  // file_size
            3,   // num_timesteps
            4,   // vocab_size
            3,   // embedding_dim
            2,   // chunk_size
            1,   // scale_factor (is logically an f32)
            10,  // pointer to the other time step
            1, 2, 3, // embedding vectors of first time step (4 at a time given as `u32`s)
            4, 5, 6, // embedding vectors of last time step (4 at a time given as `u32`s)
        ];

        let file = EmbeddingFile::new(data.into_boxed_slice()).unwrap();
        assert_eq!(file.header().file_size, 16);

        let mut data = file.into_inner();
        data[3] = 11; // Invalidate the file size field.
        assert!(EmbeddingFile::new(data).is_err());
    }
}
