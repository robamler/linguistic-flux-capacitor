use crate::ans::{Decoder, DistributionU8};

use std::convert::TryInto;

#[repr(C)]
struct EmbeddingFile<'a> {
    pub header: &'a FileHeader,
    pub timestep_addrs: &'a [u32],
    pub first_embeddings: &'a [i8],
    pub last_embeddings: &'a [i8],
    raw_data: &'a [u32],
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

impl<'a> EmbeddingFile<'a> {
    pub fn new(data: &'a [u32]) -> Result<Self, ()> {
        const HEADER_SIZE: usize = std::mem::size_of::<FileHeader>() / 4;
        if data.len() < HEADER_SIZE {
            return Err(());
        }

        let header_array: [u32; HEADER_SIZE] = data[0..HEADER_SIZE].try_into().unwrap();
        let header = unsafe {
            // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
            // `[u32; HEADER_SIZE]
            &*(&header_array as *const [u32; HEADER_SIZE] as *const FileHeader)
        };

        let embeddings_size = header.vocab_size * header.embedding_dim;

        if header.major_version != 0
            || header.file_size as usize != data.len()
            || header.num_timesteps < 2
            || embeddings_size == 0
            || embeddings_size % 4 != 0
            || header.chunk_size == 0
            || header.vocab_size % header.chunk_size != 0
        {
            return Err(());
        }

        let first_embeddings_offset = HEADER_SIZE as u32 + header.num_timesteps - 2;
        let last_embeddings_offset = first_embeddings_offset + embeddings_size / 4;
        let payload_offset = last_embeddings_offset + embeddings_size / 4;

        if header.file_size < payload_offset {
            return Err(());
        }

        // The below `unsafe` blocks are save because they just transmute
        // from `&[u32]` to `&[i8]`.
        Ok(EmbeddingFile {
            header,
            timestep_addrs: &data[HEADER_SIZE..first_embeddings_offset as usize],
            first_embeddings: get_i8_slice(
                &data[first_embeddings_offset as usize..last_embeddings_offset as usize],
            ),
            last_embeddings: get_i8_slice(
                &data[last_embeddings_offset as usize..payload_offset as usize],
            ),
            raw_data: &data[payload_offset as usize..],
        })
    }

    fn timestep(&self, t: u32) -> Result<Timestep<'a>, ()> {
        if t == 0 || t > self.header.num_timesteps {
            Err(())
        } else {
            Timestep::new(self.raw_data, self.timestep_addrs[t as usize - 1])
        }
    }
}

impl<'a> Timestep<'a> {
    fn new(raw_data: &'a [u32], addr: u32) -> Result<Self, ()> {
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

        Ok(Timestep {
            distribution: DistributionU8::new(smallest_symbol as u8, frequencies),
            chunk_addresses: raw_data
                .get((addr as usize + (frequencies_end + 3) / 4)..)
                .ok_or(())?,
            raw_data,
        })
    }

    fn chunk<'s>(&'s self, index: u32) -> Result<Decoder<'s, 'a>, ()> {
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
        let mut data = vec![
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

        let file = EmbeddingFile::new(&data).unwrap();
        assert_eq!(file.header.file_size, 15);
        assert_eq!(file.timestep_addrs, [10]);

        data[3] = 11; // Invalidate the file size field.
        assert!(EmbeddingFile::new(&data).is_err());
    }
}
