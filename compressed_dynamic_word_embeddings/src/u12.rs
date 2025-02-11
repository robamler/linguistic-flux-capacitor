//! Utilities for reading and writing packed sequences of 12-bit unsigned integers
//!
//! # Example
//!
//! ```
//! # use compressed_dynamic_word_embeddings::u12::{unpack_u12s, pack_u12s};
//! // Pack some 12-bit integers into a compact representation.
//! let unpacked = [0x0123, 0x0456, 0x0789, 0x0abc, 0x0def];
//! let packed = pack_u12s(&unpacked).collect::<Vec<_>>();
//! assert_eq!(packed, [0x0123, 0x4567, 0x89ab, 0xcdef]);
//!
//! // Re-expand the packed representation.
//! let unpacked = unpack_u12s(&packed, 5).collect::<Vec<_>>();
//! assert_eq!(unpacked, unpacked);
//! ```

/// Chop up a packed concatenation of 12-bit integers into
///
/// Returns an iterator that yields `u16`s only the least significant 12 bits
/// of each yielded value carry information (the other bits are zeroed).
///
/// This is the inverse of [`pack_u12s`](fn.pack_u12s.html).
///
/// # Examples
///
/// Chop up a slice of three `u16`s into four numbers with 12-bit accuracy. This is
/// possible because `4 * 12 <= 3 * 16`.
///
/// ```
/// # use compressed_dynamic_word_embeddings::u12::unpack_u12s;
/// let packed = [0x4567, 0x89ab, 0xcdef];
/// let unpacked = unpack_u12s(&packed, 4).collect::<Vec<_>>();
/// assert_eq!(unpacked, [0x0456, 0x0789, 0x0abc, 0x0def]);
/// ```
///
/// The argument `amt` is necessary because the number of 12-bit blocks that fit into
/// a slice of `u16`s can be ambiguous: if we wanted to store only three 12-bit
/// numbers, we would also need to use three `u16`s since two `u16`s would not
/// provide enough room. In such a situation, the highest order bits of the first
/// element of the argument `packed` must be zero:
///
/// ```
/// # use compressed_dynamic_word_embeddings::u12::unpack_u12s;
/// let packed = [0x000a, 0x1234, 0x5678];
/// let unpacked = unpack_u12s(&packed, 3).collect::<Vec<_>>();
/// assert_eq!(unpacked, [0x0a12, 0x0345, 0x0678]);
/// ```
pub fn unpack_u12s(packed: &[u16], amt: u16) -> ExpandedU12Iterator {
    ExpandedU12Iterator::new(packed, amt)
}

/// Pack a sequence of 12-bit integers into a compact representation
///
/// Expects that only the 12 lowest significant of each entry of `unpacked` carries
/// information and the other bits are zeroed. Returns an iterator that yields
/// `u16s` that result from concatenating the 12-bit integers.
///
/// This is the inverse of [`unpack_u12s`](fn.unpack_u12s.html).
///
/// # Example
///
/// Pack six 12-bit integers into a sequence of vive `u16`s. The bits will be
/// "aligned to the right", i.e., leaving any remaining space in the highest order
/// bits of the first entry.
///
/// ```
/// # use compressed_dynamic_word_embeddings::u12::pack_u12s;
/// let unpacked = [0x0123, 0x0456, 0x0789, 0x0abc, 0x0def, 0x0a1b];
/// let packed = pack_u12s(&unpacked).collect::<Vec<_>>();
/// assert_eq!(packed, [0x0012, 0x03456, 0x789a, 0xbcde, 0xfa1b]);
/// ```
pub fn pack_u12s(unpacked: &[u16]) -> CompactifiedU12Iterator {
    CompactifiedU12Iterator::new(unpacked)
}

/// Iterator that yields `u16`s whose 4 highest order bits are zero.
///
/// See [`unpack_u12s`](fn.unpack_u12s.html).
pub struct ExpandedU12Iterator<'a> {
    packed: &'a [u16],
    carry: u32,
    cursor: usize,
    remaining: usize,
}

impl<'a> ExpandedU12Iterator<'a> {
    #[inline]
    fn new(packed: &'a [u16], amt: u16) -> Self {
        // Since `amt` is a `u16` the checked arithmetic gets optimized away
        // unless `usize` is `u16` (only realistic on micro controllers).
        let expected_compact_len = (amt as usize)
            .checked_add(1)
            .unwrap()
            .checked_mul(3)
            .unwrap()
            / 4;
        assert_eq!(packed.len(), expected_compact_len);

        let (carry, cursor) = if amt % 4 == 0 {
            // The initial value of `carry` doesn't matter in this case.
            // The only reason why we don't initialize it to `packed[0]`
            // in all cases is to take into account the edge case
            // `amt == 0`, in which case `packed.len() == 0`.
            (0, usize::MAX)
        } else {
            (packed[0] as u32, 0)
        };

        Self {
            packed,
            carry,
            cursor,
            remaining: amt as usize,
        }
    }
}

impl Iterator for ExpandedU12Iterator<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            self.cursor = self.cursor.wrapping_add((self.remaining % 4 != 0) as usize);

            let shift_l = (self.remaining % 4) * 4;
            let shift_r = 16 - shift_l;

            let current = unsafe {
                // SAFETY: The constructor ensures that `self.packed` is long enough.
                *self.packed.get_unchecked(self.cursor) as u32
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

impl ExactSizeIterator for ExpandedU12Iterator<'_> {}

/// Iterator that yields `u16`s containing overlapping parts of 12-bit numbers.
///
/// See [`pack_u12s`](fn.pack_u12s.html).
pub struct CompactifiedU12Iterator<'a> {
    unpacked: &'a [u16],
    left_cursor: usize,
    right_cursor: usize,
    shift_l: usize,
}

impl<'a> CompactifiedU12Iterator<'a> {
    #[inline]
    fn new(unpacked: &'a [u16]) -> Self {
        let (right_cursor, shift_l) = if unpacked.len() % 4 == 0 {
            (1, 4)
        } else {
            (0, 16)
        };

        Self {
            unpacked,
            left_cursor: 0,
            right_cursor,
            shift_l,
        }
    }
}

impl Iterator for CompactifiedU12Iterator<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left_cursor == self.unpacked.len() {
            None
        } else {
            let left_part = self.unpacked[self.left_cursor];
            let right_part = self.unpacked[self.right_cursor];
            let shift_r = ((self.unpacked.len() - self.right_cursor - 1) % 4) * 4;
            let new = ((left_part as u32) << self.shift_l) as u16 | (right_part >> shift_r);

            self.left_cursor = self.right_cursor + (shift_r == 0) as usize;
            self.right_cursor = self.left_cursor + 1;
            self.shift_l = (self.left_cursor.wrapping_sub(self.unpacked.len() - 1) % 4) * 4;

            Some(new)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_packed = (self.unpacked.len() + 1 - self.right_cursor) * 3 / 4;
        (remaining_packed, Some(remaining_packed))
    }
}

impl ExactSizeIterator for CompactifiedU12Iterator<'_> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expand_u12s() {
        let packed = [0x0123, 0x4567, 0x89ab, 0xcdef];
        let frequencies = unpack_u12s(&packed, 5).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0123, 0x0456, 0x0789, 0x0abc, 0x0def]);

        let packed = [0x4567, 0x89ab, 0xcdef];
        let frequencies = unpack_u12s(&packed, 4).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0456, 0x0789, 0x0abc, 0x0def]);

        let packed = [0x000a, 0x1234, 0x5678];
        let frequencies = unpack_u12s(&packed, 3).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0a12, 0x0345, 0x0678]);

        let packed = [0x00ab, 0xcdef];
        let frequencies = unpack_u12s(&packed, 2).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0abc, 0x0def]);

        let packed = [0x0bcd];
        let frequencies = unpack_u12s(&packed, 1).collect::<Vec<_>>();
        assert_eq!(frequencies, [0x0bcd]);

        let packed = [];
        #[allow(clippy::needless_collect)]
        let frequencies = unpack_u12s(&packed, 0).collect::<Vec<_>>();
        assert!(frequencies.is_empty());
    }

    #[test]
    fn test_pack_u12s() {
        let frequencies = [0x0123, 0x0456, 0x0789, 0x0abc, 0x0def];
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert_eq!(packed, [0x0123, 0x4567, 0x89ab, 0xcdef]);

        let frequencies = [0x0456, 0x0789, 0x0abc, 0x0def];
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert_eq!(packed, [0x4567, 0x89ab, 0xcdef]);

        let frequencies = [0x0a12, 0x0345, 0x0678];
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert_eq!(packed, [0x000a, 0x1234, 0x5678]);

        let frequencies = [0x0abc, 0x0def];
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert_eq!(packed, [0x00ab, 0xcdef]);

        let frequencies = [0x0bcd];
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert_eq!(packed, [0x0bcd]);

        let frequencies = [];
        #[allow(clippy::needless_collect)]
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert!(packed.is_empty());

        // Directly taken from the example in the file format specification:
        let frequencies = [0x167, 0x289, 0x3ab, 0x0cd, 0x5ef];
        let packed = pack_u12s(&frequencies).collect::<Vec<_>>();
        assert_eq!(packed, [0x0167, 0x2893, 0xab0c, 0xd5ef]);
    }
}
