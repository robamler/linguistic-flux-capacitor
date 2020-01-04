use std::io::Write;
use std::mem::MaybeUninit;

use flate2::write::GzDecoder;
use wasm_bindgen::prelude::*;

/// A Buffer to pass byte streams between JavaScript and Webassembly
/// while avoiding unnecessary copies.
struct SingleCopyWriteBuffer<W: Write> {
    buf: Box<[MaybeUninit<u8>]>,
    bytes_left: usize,
    read_head: usize,
    dest: W,
}

impl<W: Write> SingleCopyWriteBuffer<W> {
    /// Create a new buffer with a fixed default capacity.
    pub fn new(dest: W) -> Self {
        Self::with_capacity(dest, 4 * 1024 * 1024)
    }

    /// Create a new buffer with a given capacity.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is zero.
    pub fn with_capacity(dest: W, capacity: usize) -> Self {
        assert_ne!(capacity, 0);
        Self {
            buf: vec![MaybeUninit::uninit(); capacity].into_boxed_slice(),
            bytes_left: 0,
            read_head: 0,
            dest,
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn get_mut_ptr(&mut self) -> *mut MaybeUninit<u8> {
        unsafe { self.buf.get_unchecked_mut(0) }
    }

    /// Tells the buffer that `amt` new bytes have been written in a contiguous
    /// sequence and are available for consumption.
    ///
    /// This method must always be called from the JavaScript side after writing any
    /// bytes to the buffer. The new bytes must have been written starting at
    /// `write_head` and must not exceed the buffer's capacity (i.e., no wrapping
    /// around at the end of the buffer.)
    ///
    /// On success, the function returns the new write head after as many bytes as
    /// possible have been consumed from the buffer. The new write head is
    /// guaranteed to be smaller than the buffer's capacity, and all bytes from the
    /// write head to the end of the buffer are considered free to be written to.
    pub fn avail(&mut self, amt: usize) -> std::io::Result<usize> {
        self.bytes_left += amt;
        if self.bytes_left == 0 {
            self.read_head = 0;
            return Ok(0);
        }

        let new_write_head = self.read_head + self.bytes_left;
        assert!(new_write_head <= self.capacity());

        let mut first_round = true;

        loop {
            let slice = unsafe {
                std::slice::from_raw_parts(
                    self.buf.get_unchecked(self.read_head).as_ptr(),
                    self.bytes_left,
                )
            };
            let amt_read = self.dest.write(slice)?;
            debug_assert!(amt_read <= self.bytes_left);
            self.bytes_left -= amt_read;

            if self.bytes_left == 0 {
                // No bytes left in buffer. Reset read and write heads to zero.
                self.read_head = 0;
                return Ok(0);
            }

            // Some bytes left in buffer.
            self.read_head += amt_read;

            if new_write_head != self.capacity() {
                return Ok(new_write_head);
            } else if !first_round {
                unsafe {
                    // Copy remaining bytes at the end of the buffer to the beginning after
                    // ensuring that the source and destination subslices don't overlap.
                    if self.bytes_left <= self.capacity() / 2 {
                        std::ptr::copy_nonoverlapping(
                            self.buf.get_unchecked(self.read_head).as_ptr(),
                            self.buf.get_unchecked_mut(0).as_mut_ptr(),
                            self.bytes_left,
                        );

                        self.read_head = 0;
                        return Ok(self.bytes_left);
                    } else if amt_read == 0 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "buffer too small",
                        ));
                    }
                    // (else: loop around, i.e., call `self.dest.read()` again.)
                }
            }

            first_round = false;
        }
    }

    pub fn finish(mut self) -> std::io::Result<W> {
        if self.bytes_left != 0 {
            let slice = unsafe {
                std::slice::from_raw_parts(
                    self.buf.get_unchecked(self.read_head).as_ptr(),
                    self.bytes_left,
                )
            };
            self.dest.write_all(slice)?;
        }

        Ok(self.dest)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct ConsumeByThreesWriter {
        consumed: Vec<u8>,
    }

    impl Write for ConsumeByThreesWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            // Consume buffer in chunks whose sizes are multiples of three.
            let num_consumed = buf.len() - buf.len() % 3;
            self.consumed.extend_from_slice(&buf[..num_consumed]);
            Ok(num_consumed)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn single_copy_write_buffer() {
        let output = ConsumeByThreesWriter {
            consumed: Vec::new(),
        };

        let mut buf = SingleCopyWriteBuffer::with_capacity(output, 10);

        assert_eq!(buf.capacity(), 10);

        // Get a mutable view into the buffer. This corresponds to the `ArrayBuffer`
        // that the JavaScript side would have in a real Wasm application. Both
        // `buf` and `buf2` have mutable access to the same memory, so access has to
        // be coordinated via calls to `buf.avail()`.
        let buf2 = unsafe {
            std::slice::from_raw_parts_mut((*buf.get_mut_ptr()).as_mut_ptr(), buf.capacity())
        };

        buf2[0..3].copy_from_slice(b"abc");
        let write_head = buf.avail(3).unwrap();
        assert_eq!(buf.read_head, 0);
        assert_eq!(write_head, 0);
        assert_eq!(&buf.dest.consumed, b"abc");

        buf2[0..7].copy_from_slice(b"defghij");
        let write_head = buf.avail(7).unwrap();
        assert_eq!(buf.read_head, 6);
        assert_eq!(write_head, 7);
        assert_eq!(&buf.dest.consumed, b"abcdefghi");

        buf2[7] = b'k';
        let write_head = buf.avail(1).unwrap();
        assert_eq!(buf.read_head, 6);
        assert_eq!(write_head, 8);
        assert_eq!(&buf.dest.consumed, b"abcdefghi");

        buf2[8..10].copy_from_slice(b"lm");
        let write_head = buf.avail(2).unwrap();
        assert_eq!(buf.read_head, 0);
        assert_eq!(write_head, 1);
        assert_eq!(&buf.dest.consumed, b"abcdefghijkl");

        buf2[1..6].copy_from_slice(b"nopqr");
        let write_head = buf.avail(5).unwrap();
        assert_eq!(buf.read_head, 0);
        assert_eq!(write_head, 0);
        assert_eq!(&buf.dest.consumed, b"abcdefghijklmnopqr");

        buf2[0..5].copy_from_slice(b"stuvw");
        let write_head = buf.avail(5).unwrap();
        assert_eq!(buf.read_head, 3);
        assert_eq!(write_head, 5);
        assert_eq!(&buf.dest.consumed, b"abcdefghijklmnopqrstu");

        assert!(buf.finish().is_err());
    }
}

#[wasm_bindgen]
pub struct SimpleSingleCopyWriteBuffer(SingleCopyWriteBuffer<Vec<u8>>);

#[wasm_bindgen]
impl SimpleSingleCopyWriteBuffer {
    pub fn new() -> Self {
        Self(SingleCopyWriteBuffer::new(Vec::<u8>::new()))
    }

    pub fn get_mut_ptr(&mut self) -> *mut MaybeUninit<u8> {
        self.0.get_mut_ptr()
    }

    pub fn capacity(&mut self) -> usize {
        self.0.capacity()
    }

    pub fn avail(&mut self, amt: usize) -> usize {
        self.0.avail(amt).unwrap()
    }

    pub fn into_string(self) -> String {
        String::from_utf8(self.0.finish().unwrap()).unwrap()
    }
}

#[wasm_bindgen]
pub struct GzCompressedBuffer(SingleCopyWriteBuffer<GzDecoder<Vec<u8>>>);

#[wasm_bindgen]
impl GzCompressedBuffer {
    pub fn new() -> Self {
        Self(SingleCopyWriteBuffer::new(GzDecoder::new(Vec::<u8>::new())))
    }

    pub fn get_mut_ptr(&mut self) -> *mut MaybeUninit<u8> {
        self.0.get_mut_ptr()
    }

    pub fn capacity(&mut self) -> usize {
        self.0.capacity()
    }

    pub fn avail(&mut self, amt: usize) -> usize {
        self.0.avail(amt).unwrap()
    }

    pub fn finish_and_peek(self) -> String {
        let mut bytes = self.0.finish().unwrap().finish().unwrap();
        bytes.resize(100, b' ');
        String::from_utf8(bytes).unwrap()
    }
}

impl Default for GzCompressedBuffer {
    fn default() -> Self {
        Self::new()
    }
}
