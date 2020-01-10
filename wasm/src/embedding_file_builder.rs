use std::mem::MaybeUninit;

use wasm_bindgen::prelude::*;

use compressed_dynamic_word_embeddings::{
    embedding_file::{EmbeddingData, EmbeddingFile},
    random_access_reader::RandomAccessReader,
};

#[wasm_bindgen]
#[derive(Default)]
pub struct EmbeddingFileBuilder {
    buf: Vec<MaybeUninit<u32>>,
    bytes_initialized: usize,
}

#[wasm_bindgen]
impl EmbeddingFileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Returns
    ///
    /// A pointer to the *start* of the buffer (which may have changed since the
    /// buffer may have been reallocated at a new position in memory.
    ///
    /// # Panics
    ///
    /// If `additional_bytes` is zero and no bytes have yet been written to the
    /// builder.
    pub fn reserve(&mut self, additional_bytes: usize) -> *mut u8 {
        self.buf.resize_with(
            usize::max(
                (self.bytes_initialized + additional_bytes + 3) / 4,
                EmbeddingFile::HEADER_SIZE,
            ),
            MaybeUninit::uninit,
        );

        assert!(!self.buf.is_empty());
        self.buf.as_mut_ptr() as *mut u8
    }

    /// Tells the buffer that `amt` new bytes have been written in a contiguous
    /// sequence and are available for consumption.
    ///
    /// # Returns
    ///
    /// `Some(file_size)` if the written data completed the file header. In this
    /// case, the buffer has been resized to the exact file size, which is returned
    /// inside the `Option`. The builder will report a `Some` value only once.
    /// After it reported a `Some` value, `reserve` should not be called any more
    /// and, in total, exactly `file_size` bytes have to be written to the builder
    /// (including the ones already written).
    ///
    /// # Safety
    ///
    /// The builder will trust the caller that `amt` bytes really have been
    /// initialized before this method is called.
    pub fn avail(&mut self, amt: usize) -> Option<PointerAndLen> {
        unsafe {
            const HEADER_BYTES: usize = EmbeddingFile::HEADER_SIZE * 4;
            if self.bytes_initialized + amt >= HEADER_BYTES {
                let ptr = self.buf.as_ptr();
                let header_u32s =
                    std::slice::from_raw_parts(ptr as *const u32, EmbeddingFile::HEADER_SIZE);
                let file_size = EmbeddingData::header_from_raw(header_u32s)
                    .unwrap()
                    .file_size;
                self.buf.reserve_exact(file_size as usize - self.buf.len());
                self.buf
                    .resize_with(file_size as usize, MaybeUninit::uninit);

                Some(PointerAndLen {
                    pointer: self.buf.as_mut_ptr() as *mut u8,
                    len: file_size as usize * 4,
                })
            } else {
                self.bytes_initialized += amt;
                None
            }
        }
    }

    /// Parse the fully filled buffer as an `EmbeddingFile`
    ///
    /// # Safety
    ///
    /// Before calling this method, the caller must have:
    /// * called `reserve` with a nonzero value, then written the announced number
    ///   of bytes and called `avail` with the same number;
    /// * repeated the last step until `avail` returned a `Some`, enclosing the file
    ///   size read out of the file header; then
    /// * filled in the rest of the buffer with exactly the right amount of bytes,
    ///   without any more calls to `reserve` or `avail`.
    /// * filled some bytes into the buffer and called avail `avail`, and repeated
    ///   this process until `avail` returned a `Some` variant; then
    /// * filled exactly as many bytes as returned inside the `Some` from the last
    ///   call to `avail`.
    ///
    /// After calling this method, the caller may no longer write to the buffer.
    pub fn finish(self) -> EmbeddingHandle {
        unsafe {
            // This is really not safe, so the method should be declared as unsafe
            // but wasm-bindgen doesn't allow exporting unsafe function (isn't that
            // the whole point of an FFI interface?)
            let len = self.buf.len();
            let ptr = std::boxed::Box::into_raw(self.buf.into_boxed_slice());
            let u32_vec = Vec::from_raw_parts(ptr as *mut u32, len, len);
            let embedding_file = EmbeddingFile::new(u32_vec.into()).unwrap();
            EmbeddingHandle::new(embedding_file.into_random_access_reader())
        }
    }
}

#[wasm_bindgen]
pub struct PointerAndLen {
    pub pointer: *mut u8,
    pub len: usize,
}

#[wasm_bindgen]
pub struct EmbeddingHandle {
    reader: RandomAccessReader,
}

impl EmbeddingHandle {
    fn new(reader: RandomAccessReader) -> Self {
        Self { reader }
    }
}

#[wasm_bindgen]
impl EmbeddingHandle {
    pub fn pairwise_trajectories(&self, words1: Vec<u32>, words2: Vec<u32>) -> Vec<f32> {
        self.reader
            .pairwise_trajectories(words1, words2)
            .into_inner()
    }

    pub fn most_related_to_at_t(&self, words: Vec<u32>, t: u32, amt: u32) -> Vec<u32> {
        self.reader.most_related_to_at_t(words, t, amt).into_inner()
    }

    pub fn largest_changes_wrt(
        &self,
        target_word: u32,
        amt: u32,
        min_increasing: u32,
        min_decreasing: u32,
    ) -> Vec<u32> {
        self.reader
            .largest_changes_wrt(target_word, amt, min_increasing, min_decreasing)
    }
}
