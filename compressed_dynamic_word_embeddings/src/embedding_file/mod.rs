use std::io::{Read, Write};
use std::ops::Deref;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::u12::expand_u12s;

use super::random_access_reader::RandomAccessReader;
use super::tensors::RankThreeTensorView;

mod builder;

type EntropyModel<SI, FI> = crate::ans::EntropyModel12_16<SI, FI>;
type DecoderModel = crate::ans::DecoderModel<crate::ans::EntropyModelOptions12_16, i16>;
type Decoder<'model, 'data> =
    crate::ans::Decoder<'model, 'data, crate::ans::EntropyModelOptions12_16, i16>;

pub const HEADER_SIZE: u32 = (std::mem::size_of::<FileHeader>() / 4) as u32;

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
    pub meta_size: u32,
    pub num_timesteps: u32,
    pub vocab_size: u32,
    pub embedding_dim: u32,
    pub chunk_size: u32,
    pub scale_factor: f32,
}

// TODO: why does this need to be repr(C)?
#[repr(C)]
pub struct Timestep<'a> {
    decoder_model: DecoderModel,
    chunk_addresses: &'a [u32],
    embedding_data: &'a EmbeddingData,
}

impl EmbeddingFile {
    pub fn new(data: Box<[u32]>) -> Result<Self, ()> {
        if data.len() < HEADER_SIZE as usize {
            return Err(());
        }

        let file = Self { raw_data: data };
        let header = file.header();
        let embeddings_size = header.vocab_size * header.embedding_dim;

        if header.magic != 0x65776400
            || header.major_version != 1
            || header.file_size as usize != file.raw_data.len()
            || header.meta_size > header.file_size
            || header.num_timesteps < 2
            || embeddings_size == 0
            || embeddings_size == 0
            || header.chunk_size == 0
            || header.vocab_size % header.chunk_size != 0
        {
            return Err(());
        }

        // Check only if file is at least large enough to hold the root block.
        if header.file_size < HEADER_SIZE + header.num_timesteps {
            return Err(());
        }

        Ok(file)
    }

    pub fn from_reader(mut reader: impl Read) -> Result<EmbeddingFile, ()> {
        let mut buf = Vec::new();
        buf.resize(HEADER_SIZE as usize, 0);
        reader
            .read_u32_into::<LittleEndian>(&mut buf[..])
            .map_err(|_| ())?;

        let file_size = EmbeddingData::header_from_raw(&buf[..])?.file_size;
        buf.resize(file_size as usize, 0);
        reader
            .read_u32_into::<LittleEndian>(&mut buf[HEADER_SIZE as usize..])
            .map_err(|_| ())?;
        Self::new(buf.into())
    }

    pub fn from_uncompressed_quantized(
        uncompressed: RankThreeTensorView<i16>,
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
    #[inline(always)]
    pub fn header(&self) -> &FileHeader {
        unsafe {
            // This is safe because the constructor checks that `raw_data` is big enough.
            let header_slice = self.raw_data.get_unchecked(0..HEADER_SIZE as usize);

            // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
            // `[u32; HEADER_SIZE]`
            let ptr = header_slice.as_ptr();
            &*(ptr as *const FileHeader)
        }
    }

    #[inline(always)]
    pub fn header_from_raw(header: &[u32]) -> Result<&FileHeader, ()> {
        if header.len() < HEADER_SIZE as usize {
            Err(())
        } else {
            unsafe {
                // This is safe because `FileHeader` is `repr(C)` and has the same alignment as
                // `[u32; HEADER_SIZE]`
                let ptr = header.as_ptr();
                Ok(&*(ptr as *const FileHeader))
            }
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
        if t >= self.header().num_timesteps {
            Err(())
        } else {
            let addr = self.raw_data[(HEADER_SIZE + t) as usize];
            // `vocab_size` is guaranteed to be a multiple of `chunk_size`.
            let num_chunks = header.vocab_size / header.chunk_size;
            Timestep::new(&self, addr, num_chunks)
        }
    }

    pub fn as_slice_u32(&self) -> &[u32] {
        &self.raw_data
    }
}

impl<'a> Timestep<'a> {
    fn new(embedding_data: &'a EmbeddingData, addr: u32, num_chunks: u32) -> Result<Self, ()> {
        let u16_slice = get_u16_slice(embedding_data.raw_data.get(addr as usize..).ok_or(())?);

        let num_symbols = u16_slice[0];
        let symbols = &u16_slice[1..1 + num_symbols as usize];
        let packed_size = (3 * num_symbols as usize) / 4;
        let frequencies = expand_u12s(
            &u16_slice[1 + num_symbols as usize..num_symbols as usize + 1 + packed_size],
            num_symbols - 1,
        );
        let model_description_length = (1 + num_symbols as usize + packed_size + 1) / 2;

        let decoder_model =
            EntropyModel::new(symbols.iter().map(|&x| x as i16), frequencies).decoder_model();

        let start_addr = addr as usize + model_description_length;
        let chunk_addresses = embedding_data
            .raw_data
            .get(start_addr..start_addr + num_chunks as usize)
            .ok_or(())?;

        Ok(Timestep {
            decoder_model,
            chunk_addresses,
            embedding_data,
        })
    }

    pub fn chunk<'s>(&'s self, index: u32) -> Result<Decoder<'s, 'a>, ()> {
        let addr = *self.chunk_addresses.get(index as usize).ok_or(())?;
        let compressed_data = get_u16_slice(
            self.embedding_data
                .raw_data
                .get(addr as usize..)
                .ok_or(())?,
        );
        Ok(self.decoder_model.decoder(compressed_data))
    }

    pub fn reader<'s>(&'s self) -> RawTimestepReader<'s, 'a> {
        RawTimestepReader::new(self)
    }
}

pub trait TimestepReader {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        callback: impl FnMut(i16, I::Item),
    ) -> Result<(), ()>;
}

pub struct RawTimestepReader<'model, 'data> {
    timestep: &'model Timestep<'data>,
    decoder_and_chunk_index: Option<(Decoder<'model, 'data>, u32)>,
    offset: u32,
}

impl<'model, 'data> RawTimestepReader<'model, 'data> {
    fn new(timestep: &'model Timestep<'data>) -> Self {
        Self {
            timestep,
            decoder_and_chunk_index: None,
            offset: 0,
        }
    }
}

impl<'model, 'data> TimestepReader for RawTimestepReader<'model, 'data> {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        mut callback: impl FnMut(i16, I::Item),
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
        decoder
            .skip((offset - self.offset) as usize)
            .map_err(|_| ())?;
        decoder
            .streaming_decode(dest_iter, |&symbol, dest| callback(symbol, dest))
            .map_err(|_| ())?;

        self.offset = offset + header.embedding_dim;
        self.decoder_and_chunk_index = Some((decoder, chunk_index));

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
