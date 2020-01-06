use crate::ans::{Decoder, DistributionU8};

use wasm_bindgen::prelude::*;

use std::convert::TryInto;

#[wasm_bindgen]
struct EmbeddingFile {
    raw_data: Box<[u32]>,
}

#[repr(C)]
struct FileHeader {
    pub magic: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub file_size: u32,
    pub num_timesteps: u32,
    pub vocab_size: u32,
    pub embedding_dim: u32,
    pub chunk_size: u32,
}

#[repr(C)]
struct Timestep<'a> {
    distribution: DistributionU8,
    chunk_addresses: &'a [u32],
    raw_data: &'a [u32],
}

impl EmbeddingFile {
    const HEADER_SIZE: usize = std::mem::size_of::<FileHeader>() / 4;

    pub fn new(data: Box<[u32]>) -> Result<Self, ()> {
        if data.len() < Self::HEADER_SIZE {
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

        let first_embeddings_offset = Self::HEADER_SIZE as u32 + header.num_timesteps - 2;
        let last_embeddings_offset = first_embeddings_offset + embeddings_size / 4;
        let payload_offset = last_embeddings_offset + embeddings_size / 4;

        if header.file_size < payload_offset {
            return Err(());
        }

        Ok(file)
    }

    pub fn header(&self) -> &FileHeader {
        let header_slice = unsafe {
            // This is safe because the constructor checks that `raw_data` is big enough.
            self.raw_data.get_unchecked(0..Self::HEADER_SIZE)
        };
        let header_array: [u32; Self::HEADER_SIZE] = header_slice.try_into().unwrap();
        unsafe {
            // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
            // `[u32; HEADER_SIZE]
            &*(&header_array as *const [u32; Self::HEADER_SIZE] as *const FileHeader)
        }
    }

    pub fn first_embeddings(&self) -> &[i8] {
        let header = self.header();
        let begin = Self::HEADER_SIZE as u32 + header.num_timesteps - 2;
        let end = begin + header.vocab_size * header.embedding_dim / 4;

        get_i8_slice(unsafe {
            // This is safe because the constructor checks that `raw_data` is big enough.
            &self.raw_data.get_unchecked(begin as usize..end as usize)
        })
    }

    pub fn last_embeddings(&self) -> &[i8] {
        let header = self.header();
        let embedding_size = header.vocab_size * header.embedding_dim / 4;
        let begin = Self::HEADER_SIZE as u32 + header.num_timesteps - 2 + embedding_size;
        let end = begin + embedding_size;

        get_i8_slice(unsafe {
            // This is safe because the constructor checks that `raw_data` is big enough.
            &self.raw_data.get_unchecked(begin as usize..end as usize)
        })
    }

    pub fn timestep(&self, t: u32) -> Result<Timestep, ()> {
        if t == 0 || t > self.header().num_timesteps {
            Err(())
        } else {
            let addr = unsafe {
                // This is safe because the constructor checks that `raw_data` is big enough.
                *self
                    .raw_data
                    .get_unchecked(Self::HEADER_SIZE + (t - 1) as usize)
            };

            let header = self.header();
            // `vocab_size` is guaranteed to be a multiple of `chunk_size`.
            let num_chunks = header.vocab_size / header.chunk_size;

            Timestep::new(&self.raw_data, addr, num_chunks)
        }
    }

    pub fn into_inner(self) -> Box<[u32]> {
        self.raw_data
    }
}

impl<'a> Timestep<'a> {
    fn new(raw_data: &'a [u32], addr: u32, num_chunks: u32) -> Result<Self, ()> {
        let byteslice = get_u8_slice(raw_data.get(addr as usize..).ok_or(())?);

        let smallest_symbol = byteslice[0] as i8;
        let largest_symbol = byteslice[1] as i8;
        if largest_symbol <= smallest_symbol {
            return Err(());
        }
        let frequencies_end = 3 + (largest_symbol - smallest_symbol) as usize;
        let frequencies = &byteslice[2..frequencies_end];

        // The compression module operates on unsigned rather than on signed bytes so
        // that it is not unnecessarily coupled to this specific application. We convert
        // symbols between `u8` and `i8` as we pass them to or from the compression
        // module. On the machine code level, this conversion is a no-op.
        let distribution = DistributionU8::new(smallest_symbol as u8, frequencies);

        let start_addr = addr as usize + (frequencies_end + 3) / 4;
        let chunk_addresses = raw_data
            .get(start_addr..start_addr + num_chunks as usize)
            .ok_or(())?;

        Ok(Timestep {
            distribution,
            chunk_addresses,
            raw_data,
        })
    }

    pub fn chunk<'s>(&'s self, index: u32) -> Result<Decoder<'s, 'a>, ()> {
        let addr = *self.chunk_addresses.get(index as usize).ok_or(())?;
        let compressed_data = get_u16_slice(self.raw_data.get(addr as usize..).ok_or(())?);
        self.distribution.decoder(compressed_data)
    }
}

fn get_i8_slice(data: &[u32]) -> &[i8] {
    unsafe {
        // Transmuting from `&[u32]` to `&[i8]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        data.align_to().1
    }
}

fn get_u8_slice(data: &[u32]) -> &[u8] {
    unsafe {
        // Transmuting from `&[u32]` to `&[u8]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        data.align_to().1
    }
}

fn get_u16_slice(data: &[u32]) -> &[u16] {
    unsafe {
        // Transmuting from `&[u32]` to `&[u16]` is always safe, see, e.g.:
        // https://internals.rust-lang.org/t/pre-rfc-v2-safe-transmute/11431
        data.align_to().1
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
            15,  // file_size
            3,   // num_timesteps
            4,   // vocab_size
            3,   // embedding_dim
            2,   // chunk_size
            10,  // pointer to the other time step
            1, 2, 3, // embedding vectors of first time step (4 at a time given as `u32`s)
            4, 5, 6, // embedding vectors of last time step (4 at a time given as `u32`s)
        ];

        let file = EmbeddingFile::new(data.into_boxed_slice()).unwrap();
        assert_eq!(file.header().file_size, 15);

        let mut data = file.into_inner();
        data[3] = 11; // Invalidate the file size field.
        assert!(EmbeddingFile::new(data).is_err());
    }
}
