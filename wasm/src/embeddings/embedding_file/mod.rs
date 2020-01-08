mod builder;

use super::compression::{Decoder, DistributionU8};
use super::random_access_reader::RandomAccessReader;
use super::tensors::RankThreeTensorView;

use wasm_bindgen::prelude::*;

use std::ops::Deref;

#[wasm_bindgen]
pub struct EmbeddingFile {
    raw_data: Box<[u32]>,
}

#[repr(transparent)]
pub struct EmbeddingData {
    raw_data: [u32],
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct FileHeader {
    pub magic: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub file_size: u32,
    pub num_timesteps: u32,
    pub vocab_size: u32,
    pub embedding_dim: u32,
    pub chunk_size: u32,
    pub scale_factor: f32,
}

#[repr(C)]
pub struct CompressedTimestep<'a> {
    distribution: DistributionU8,
    chunk_addresses: &'a [u32],
    embedding_data: &'a EmbeddingData,
}

#[repr(C)]
pub struct UncompressedTimestep<'a> {
    uncompressed: &'a [i8],
    embedding_data: &'a EmbeddingData,
}

impl EmbeddingFile {
    pub fn new(data: Box<[u32]>) -> Result<Self, ()> {
        if data.len() < EmbeddingData::HEADER_SIZE {
            return Err(());
        }

        let file = Self { raw_data: data };
        let header = file.header();
        let embeddings_size = header.vocab_size * header.embedding_dim;

        if header.major_version != 0
            || header.file_size as usize != file.raw_data.len()
            || header.num_timesteps < 2
            || embeddings_size == 0
            || embeddings_size % 4 != 0
            || header.chunk_size == 0
            || header.vocab_size % header.chunk_size != 0
        {
            return Err(());
        }

        let first_embeddings_offset = EmbeddingData::HEADER_SIZE as u32 + header.num_timesteps - 2;
        let last_embeddings_offset = first_embeddings_offset + embeddings_size / 4;
        let payload_offset = last_embeddings_offset + embeddings_size / 4;

        if header.file_size < payload_offset {
            return Err(());
        }

        Ok(file)
    }

    pub fn from_uncompressed_quantized(
        uncompressed: RankThreeTensorView<i8>,
        chunk_size: u32,
        scale_factor: f32,
    ) -> Result<Self, ()> {
        Self::new(builder::build_file(uncompressed, chunk_size, scale_factor).into())
    }

    pub fn into_random_access_reader(self) -> RandomAccessReader {
        RandomAccessReader::new(self)
    }

    pub fn into_inner(self) -> Box<[u32]> {
        self.raw_data
    }
}

impl Deref for EmbeddingFile {
    type Target = EmbeddingData;

    fn deref(&self) -> &EmbeddingData {
        let raw_data_slice: &[u32] = &self.raw_data;
        unsafe {
            // As far as I understand, this should be safe because `EmbeddingData` is
            // declared as `#[repr(transparent)]`.
            &*(raw_data_slice as *const [u32] as *const EmbeddingData)
        }
    }
}

impl EmbeddingData {
    const HEADER_SIZE: usize = std::mem::size_of::<FileHeader>() / 4;

    pub fn header(&self) -> &FileHeader {
        unsafe {
            // This is safe because the constructor checks that `raw_data` is big enough.
            let header_slice = self.raw_data.get_unchecked(0..Self::HEADER_SIZE);

            // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
            // `[u32; HEADER_SIZE]`
            let ptr = header_slice.as_ptr();
            &*(ptr as *const FileHeader)
        }
    }

    pub fn margin_embeddings(&self, level: u32) -> UncompressedTimestep {
        assert!(level < 2);
        let header = self.header();
        let embedding_size = header.vocab_size * header.embedding_dim / 4;
        let begin = Self::HEADER_SIZE as u32 + header.num_timesteps - 2 + level * embedding_size;
        let end = begin + embedding_size;

        UncompressedTimestep {
            uncompressed: get_i8_slice(&self.raw_data[begin as usize..end as usize]),
            embedding_data: self,
        }
    }

    pub fn timestep(&self, t: u32) -> Result<CompressedTimestep, ()> {
        if t == 0 || t > self.header().num_timesteps {
            Err(())
        } else {
            let addr = self.raw_data[Self::HEADER_SIZE + (t - 1) as usize];
            let header = self.header();
            // `vocab_size` is guaranteed to be a multiple of `chunk_size`.
            let num_chunks = header.vocab_size / header.chunk_size;

            CompressedTimestep::new(&self, addr, num_chunks)
        }
    }
}

impl<'a> CompressedTimestep<'a> {
    fn new(embedding_data: &'a EmbeddingData, addr: u32, num_chunks: u32) -> Result<Self, ()> {
        let byte_slice = get_u8_slice(embedding_data.raw_data.get(addr as usize..).ok_or(())?);

        let smallest_symbol = byte_slice[0] as i8;
        let largest_symbol = byte_slice[1] as i8;
        let frequencies_end = 3 + largest_symbol.wrapping_sub(smallest_symbol) as u8 as usize;
        let frequencies = &byte_slice[2..frequencies_end];

        // The compression module operates on unsigned rather than on signed bytes so
        // that it is not unnecessarily coupled to this specific application. We convert
        // symbols between `u8` and `i8` as we pass them to or from the compression
        // module. On the machine code level, this conversion is a no-op.
        let distribution = DistributionU8::new(smallest_symbol as u8, frequencies);

        let start_addr = addr as usize + (frequencies_end + 3) / 4;
        let chunk_addresses = embedding_data
            .raw_data
            .get(start_addr..start_addr + num_chunks as usize)
            .ok_or(())?;

        Ok(CompressedTimestep {
            distribution,
            chunk_addresses,
            embedding_data,
        })
    }

    fn chunk<'s>(&'s self, index: u32) -> Result<Decoder<'s, 'a>, ()> {
        let addr = *self.chunk_addresses.get(index as usize).ok_or(())?;
        let compressed_data = get_u16_slice(
            self.embedding_data
                .raw_data
                .get(addr as usize..)
                .ok_or(())?,
        );
        self.distribution.decoder(compressed_data)
    }

    pub fn reader<'s>(&'s self) -> CompressedTimestepReader<'s, 'a> {
        CompressedTimestepReader::new(self)
    }
}

pub trait TimestepReader {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        callback: impl FnMut(i8, I::Item),
    ) -> Result<(), ()>;
}

pub struct CompressedTimestepReader<'a, 'b> {
    timestep: &'a CompressedTimestep<'b>,
    decoder_and_chunk_index: Option<(Decoder<'a, 'b>, u32)>,
    offset: u32,
}

impl<'a, 'b> CompressedTimestepReader<'a, 'b> {
    fn new(timestep: &'a CompressedTimestep<'b>) -> Self {
        Self {
            timestep,
            decoder_and_chunk_index: None,
            offset: 0,
        }
    }
}

impl TimestepReader for CompressedTimestepReader<'_, '_> {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        mut callback: impl FnMut(i8, I::Item),
    ) -> Result<(), ()> {
        let header = self.timestep.embedding_data.header();
        let chunk_index = index / header.chunk_size;
        let offset = header.embedding_dim * (index % header.chunk_size);

        // TODO: find out if the taking and resetting has an impact on performance
        //       or whether it's just semantics.
        let mut decoder = match self.decoder_and_chunk_index.take() {
            Some((decoder, old_chunk_index)) if old_chunk_index == chunk_index => decoder,
            _ => {
                self.offset = 0;
                self.timestep.chunk(chunk_index)?
            }
        };

        assert!(offset >= self.offset);
        decoder.skip((offset - self.offset) as usize)?;
        decoder.decode(dest_iter, |byte, dest_item| callback(byte as i8, dest_item))?;

        self.offset = offset + header.embedding_dim;
        self.decoder_and_chunk_index = Some((decoder, chunk_index));

        Ok(())
    }
}

impl TimestepReader for UncompressedTimestep<'_> {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        mut callback: impl FnMut(i8, I::Item),
    ) -> Result<(), ()> {
        let header = self.embedding_data.header();
        let start = header.embedding_dim * index;
        let end = start + header.embedding_dim;

        // The order in which we zip the iterators is important here since `zip` is
        // short-circuiting. We want to allow callers to continue to use `dest_iter`
        // after this method terminates.
        for (source, dest) in self
            .uncompressed
            .get(start as usize..end as usize)
            .ok_or(())?
            .iter()
            .zip(dest_iter)
        {
            callback(*source, dest);
        }

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
