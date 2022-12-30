use std::cmp::Ordering::*;
use std::collections::BinaryHeap;

use constriction::{stream::Decode, UnwrapInfallible};

use crate::tensors::RankTwoTensorViewMut;

use super::embedding_file::{EmbeddingFile, TimestepReader};
use super::tensors::{RankThreeTensor, RankTwoTensor, RankTwoTensorView};

pub struct RandomAccessReader {
    file: EmbeddingFile,

    /// The height of the tree. The first and last time step each count as one
    /// toward the tree height.
    tree_height: u32,
}

impl RandomAccessReader {
    pub fn new(embedding_file: EmbeddingFile) -> Self {
        let num_timesteps = embedding_file.header().num_timesteps;
        let tree_height = if num_timesteps <= 2 {
            2
        } else {
            34 - (num_timesteps - 2).leading_zeros()
        };

        Self {
            file: embedding_file,
            tree_height,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pairwise_trajectories(
        &self,
        mut words1: Vec<u32>,
        mut words2: Vec<u32>,
    ) -> RankTwoTensor<f32> {
        fn process_timestep(
            mut reader: impl TimestepReader,
            mut embeddings: RankTwoTensorViewMut<i16>,
            output: &mut [f32],
            unique_words: &[u32],
            words1: &[u32],
            words2: &[u32],
            embedding_dim: u32,
            scale_factor_square: f32,
        ) {
            let mut embeddings_iter_mut = embeddings.as_mut_slice().iter_mut();
            for &word in unique_words {
                reader.jump_to(word).unwrap();
                reader
                    .read_single_embedding_vector(
                        (&mut embeddings_iter_mut).take(embedding_dim as usize),
                        |n, dest| *dest = n,
                    )
                    .unwrap();
            }

            for ((&w1, &w2), dest) in words1.iter().zip(words2).zip(output.iter_mut()) {
                let embedding1 = embeddings.subview(w1 as usize);
                let embedding2 = embeddings.subview(w2 as usize);
                let scalar_product = embedding1
                    .iter()
                    .zip(embedding2)
                    .map(|(&a, &b)| a as i32 * b as i32)
                    .sum::<i32>();
                *dest = scale_factor_square * scalar_product as f32;
            }
        }

        if words1.is_empty() || words1.len() != words2.len() {
            // TODO: handle error if words1.len() != words2.len()
            return RankTwoTensor::new(0, self.file.header().num_timesteps as usize);
        }

        let mut unique_words = words1
            .iter()
            .chain(words2.iter())
            .cloned()
            .collect::<BinaryHeap<u32>>()
            .into_sorted_vec();
        unique_words.dedup();
        let unique_words = unique_words; // Make immutable.

        // Replace entries of `words1` and `words2` with their indices into `unique_words`.
        for word in words1.iter_mut().chain(words2.iter_mut()) {
            *word = unique_words.binary_search(word).unwrap() as u32;
        }

        let header = self.file.header();
        let embedding_dim = header.embedding_dim;
        let scale_factor_square = header.scale_factor * header.scale_factor;

        let mut extracted_embeddings = RankThreeTensor::<i16>::new(
            self.tree_height as usize,
            unique_words.len(),
            header.embedding_dim as usize,
        );
        let mut extracted_embeddings = extracted_embeddings.as_view_mut();

        let mut output = RankTwoTensor::<f32>::new(header.num_timesteps as usize, words1.len());
        let mut output = output.as_view_mut();

        // Extract relevant embeddings and calculate scalar products for first and last
        // time step (levels 0 and 1).
        for &(t, level) in &[(0, 0), (header.num_timesteps - 1, 1)] {
            process_timestep(
                self.file.timestep(t).unwrap(),
                extracted_embeddings.subview_mut(level as usize),
                output.subview_mut(t as usize),
                &unique_words,
                &words1,
                &words2,
                embedding_dim,
                scale_factor_square,
            );
        }

        traverse_subtree(
            2,
            0,
            0,
            header.num_timesteps - 1,
            1,
            &mut |t, level, _left_t, left_level, _right_t, right_level| {
                let (left_parent, right_parent, target) = extracted_embeddings.subviews_rrw(
                    left_level as usize,
                    right_level as usize,
                    level as usize,
                );

                let timestep = self.file.timestep(t).unwrap();
                let reader = AccumulatingReader::new(left_parent, right_parent, timestep);
                process_timestep(
                    reader,
                    target,
                    output.subview_mut(t as usize),
                    &unique_words,
                    &words1,
                    &words2,
                    embedding_dim,
                    scale_factor_square,
                );
                (true, true)
            },
        );

        output.downgrade().to_transposed()
    }

    pub fn most_related_to_at_t(
        &self,
        target_words: Vec<u32>,
        t: u32,
        amt: u32,
    ) -> RankTwoTensor<u32> {
        let embeddings = self.get_embeddings_at(t);
        let embeddings = embeddings.as_view();

        let mut unique_words = target_words
            .iter()
            .cloned()
            .collect::<BinaryHeap<u32>>()
            .into_sorted_vec();
        unique_words.dedup();
        let unique_words = unique_words; // Make immutable;

        let header = self.file.header();
        let embedding_dim = header.embedding_dim;

        let mut target_embeddings = RankTwoTensor::new(unique_words.len(), embedding_dim as usize);
        for (&word, target) in unique_words
            .iter()
            .zip(target_embeddings.as_view_mut().iter_mut_subviews())
        {
            target.copy_from_slice(embeddings.subview(word as usize));
        }
        let target_embeddings = target_embeddings.as_view();

        let mut front_runners =
            RankTwoTensor::<FrontRunnerCandidate<i32>>::new(unique_words.len(), amt as usize);
        let mut front_runners = front_runners.as_view_mut();

        for (word, embedding) in embeddings.iter_subviews().enumerate() {
            for ((&target_word, target_embedding), front_runners) in unique_words
                .iter()
                .zip(target_embeddings.iter_subviews())
                .zip(front_runners.iter_mut_subviews())
            {
                let scalar_product = embedding
                    .iter()
                    .zip(target_embedding)
                    .map(|(&a, &b)| a as i32 * b as i32)
                    .sum::<i32>();

                let (mut last_fr, remaining_fr) = front_runners.split_last_mut().unwrap();

                // Make common case (last_fr.n > scalar_product) quick.
                if last_fr.n < scalar_product && word as u32 != target_word {
                    // Swap sort: overwrite last element, then swap forward. This is optimized
                    // for small `amt`. For large `amt`, a BinaryHeap might be faster.
                    last_fr.word = word as u32;
                    last_fr.n = scalar_product;

                    for fr in remaining_fr.iter_mut().rev() {
                        if fr.n < scalar_product {
                            std::mem::swap(fr, last_fr);
                            last_fr = fr;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        let front_runners = front_runners.downgrade();

        let mut reordered_top_k = RankTwoTensor::new(target_words.len(), amt as usize);
        let mut reordered_top_k_view_mut = reordered_top_k.as_view_mut();

        for (&word, dest) in target_words
            .iter()
            .zip(reordered_top_k_view_mut.iter_mut_subviews())
        {
            let word_index = unique_words.binary_search(&word).unwrap() as u32;

            for (dest_val, fr) in dest
                .iter_mut()
                .zip(front_runners.subview(word_index as usize))
            {
                *dest_val = fr.word
            }
        }

        reordered_top_k
    }

    fn get_embeddings_at(&self, t: u32) -> RankTwoTensor<i16> {
        let header = self.file.header();
        let timestep_size = header.vocab_size * header.embedding_dim;

        let extract_timestep = |t| {
            let (mut decoder, model) = self.file.timestep(t).unwrap().into_inner();
            decoder
                .decode_iid_symbols(timestep_size as usize, model)
                .map(UnwrapInfallible::unwrap_infallible)
                .collect::<Vec<_>>()
        };

        let result = if t == 0 || t == header.num_timesteps - 1 {
            extract_timestep(t)
        } else {
            let mut t_left = 0;
            let mut t_right = header.num_timesteps - 1;
            let mut buf_left = extract_timestep(t_left);
            let mut buf_right = extract_timestep(t_right);
            let mut buf = vec![0; timestep_size as usize]; // TODO: use MaybeUninit

            loop {
                let t_center = (t_left + t_right) / 2;
                let (mut decoder, model) = self.file.timestep(t_center).unwrap().into_inner();
                let symbols = decoder.decode_iid_symbols(timestep_size as usize, model);
                for (((s, target), &l), &r) in
                    symbols.zip(buf.iter_mut()).zip(&buf_left).zip(&buf_right)
                {
                    *target = s
                        .unwrap_infallible()
                        .wrapping_add(((l as i32 + r as i32) / 2) as i16);
                }

                match t_center.cmp(&t) {
                    Equal => break buf,
                    Less => {
                        t_left = t_center;
                        std::mem::swap(&mut buf, &mut buf_left);
                    }
                    Greater => {
                        t_right = t_center;
                        std::mem::swap(&mut buf, &mut buf_right);
                    }
                }
            }
        };

        RankTwoTensor::from_flattened(
            result,
            header.vocab_size as usize,
            header.embedding_dim as usize,
        )
    }

    pub fn largest_changes_wrt(
        &self,
        target_word: u32,
        amt: u32,
        min_increasing: u32,
        min_decreasing: u32,
    ) -> Vec<u32> {
        let header = self.file.header();
        let num_timesteps = header.num_timesteps;
        let vocab_size = header.vocab_size;
        let embedding_dim = header.embedding_dim;

        let extract_single_embedding_vector = |t, i| {
            let mut timestep = self.file.timestep(t).unwrap();
            timestep.jump_to(i).unwrap();
            let mut emb_vector = Vec::with_capacity(embedding_dim as usize);
            timestep
                .read_single_embedding_vector(0..embedding_dim, |s, _| emb_vector.push(s))
                .unwrap();
            timestep.jump_to(0).unwrap();
            (emb_vector, timestep.into_inner())
        };

        let (first_target, (mut first_timestep_decoder, first_timestep_model)) =
            extract_single_embedding_vector(0, target_word);
        let (last_target, (mut last_timestep_decoder, last_timestep_model)) =
            extract_single_embedding_vector(num_timesteps - 1, target_word);

        let mut increasing_front_runners = Vec::<FrontRunnerCandidate<i64>>::new();
        increasing_front_runners.resize_with(amt as usize, Default::default);

        let mut decreasing_front_runners = Vec::<FrontRunnerCandidate<i64>>::new();
        decreasing_front_runners.resize_with(amt as usize, Default::default);

        for word in 0..vocab_size {
            let first_dot_product = first_timestep_decoder
                .decode_iid_symbols(embedding_dim as usize, first_timestep_model)
                .zip(&first_target)
                .map(|(a, &b)| a.unwrap_infallible() as i32 * b as i32)
                .sum::<i32>();
            let last_dot_product = last_timestep_decoder
                .decode_iid_symbols(embedding_dim as usize, last_timestep_model)
                .zip(&last_target)
                .map(|(a, &b)| a.unwrap_infallible() as i32 * b as i32)
                .sum::<i32>();

            if word != target_word {
                let diff = last_dot_product as i64 - first_dot_product as i64;

                let increasing_last_better = increasing_front_runners
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_index, fr)| fr.n >= diff);
                let increasing_first_worse =
                    increasing_last_better.map_or(0, |(index, _)| index + 1);
                let mut insert_fr = FrontRunnerCandidate { word, n: diff };
                for dest in increasing_front_runners[increasing_first_worse..].iter_mut() {
                    std::mem::swap(dest, &mut insert_fr);
                }

                let neg_diff = -diff;
                let decreasing_last_better = decreasing_front_runners
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_index, fr)| fr.n >= neg_diff);
                let decreasing_first_worse =
                    decreasing_last_better.map_or(0, |(index, _)| index + 1);
                let mut insert_fr = FrontRunnerCandidate { word, n: neg_diff };
                for dest in decreasing_front_runners[decreasing_first_worse..].iter_mut() {
                    std::mem::swap(dest, &mut insert_fr);
                }
            }
        }

        // In the very unlikely but possible corner case that fewer than
        // `min_increasing` words have increasing overlap with the target word,
        // `increasing_front_runners[..min_increasing as usize]` holds the
        // `min_increasing` words with *least* decreasing overlap. That's OK since,
        // in this weird situation, these words are arguably the most interesting
        // ones (analogously for `min_decreasing`).

        let mut combined = Vec::with_capacity((2 * amt) as usize);

        // Put the required words (`min_increasing` and `min_decreasing`) first.
        combined.extend_from_slice(&increasing_front_runners[..min_increasing as usize]);
        combined.extend_from_slice(&decreasing_front_runners[..min_decreasing as usize]);

        // Then put the remaining items and sort them.
        combined.extend_from_slice(&increasing_front_runners[min_increasing as usize..]);
        combined.extend_from_slice(&decreasing_front_runners[min_decreasing as usize..]);
        combined[(min_increasing + min_decreasing) as usize..].sort_by_key(|fr| -fr.n);

        // We will keep on only the first half of the list. Sort it as well by magnitude
        // of the change, so that in particular the first result (which a viewer may
        // highlight by default) is the one with the largest change in magnitude.
        combined[..amt as usize].sort_by_key(|fr| -fr.n);

        // Retain only the `word` part of the first half of the list.
        combined
            .into_iter()
            .take(amt as usize)
            .map(|fr| fr.word)
            .collect()
    }
}

#[derive(Copy, Clone)]
struct FrontRunnerCandidate<T> {
    word: u32,
    n: T,
}

impl Default for FrontRunnerCandidate<i32> {
    fn default() -> Self {
        Self {
            word: std::u32::MAX,
            n: std::i32::MIN,
        }
    }
}

impl Default for FrontRunnerCandidate<i64> {
    fn default() -> Self {
        Self {
            word: std::u32::MAX,
            n: std::i64::MIN,
        }
    }
}

struct AccumulatingReader<R: TimestepReader, LI: Iterator<Item = i16>, RI: Iterator<Item = i16>> {
    inner: R,
    left_parent: LI,
    right_parent: RI,
}

impl<'a, R: TimestepReader>
    AccumulatingReader<
        R,
        std::iter::Cloned<std::slice::Iter<'a, i16>>,
        std::iter::Cloned<std::slice::Iter<'a, i16>>,
    >
{
    fn new(
        left_parent: RankTwoTensorView<'a, i16>,
        right_parent: RankTwoTensorView<'a, i16>,
        center: R,
    ) -> Self {
        Self {
            left_parent: left_parent.slice().iter().cloned(),
            right_parent: right_parent.slice().iter().cloned(),
            inner: center,
        }
    }
}

impl<R: TimestepReader, LI: Iterator<Item = i16>, RI: Iterator<Item = i16>> TimestepReader
    for AccumulatingReader<R, LI, RI>
{
    fn read_single_embedding_vector<I: Iterator>(
        &mut self,
        dest_iter: I,
        mut callback: impl FnMut(i16, I::Item),
    ) -> Result<(), ()> {
        self.inner.read_single_embedding_vector(
            dest_iter
                .zip(&mut self.left_parent)
                .zip(&mut self.right_parent),
            |center, ((dest, left), right)| {
                let value = center.wrapping_add(((left as i32 + right as i32) / 2) as i16);
                callback(value, dest);
            },
        )
    }

    #[inline(always)]
    fn jump_to(&mut self, word_index: u32) -> Result<(), ()> {
        self.inner.jump_to(word_index)
    }
}

fn traverse_subtree(
    level: u32,
    left_t: u32,
    left_level: u32,
    right_t: u32,
    right_level: u32,
    callback: &mut impl FnMut(u32, u32, u32, u32, u32, u32) -> (bool, bool),
) {
    let t = (left_t + right_t) / 2;
    if t != left_t {
        let (continue_left, continue_right) =
            callback(t, level, left_t, left_level, right_t, right_level);
        if continue_left {
            traverse_subtree(level + 1, left_t, left_level, t, level, callback);
        }
        if continue_right {
            traverse_subtree(level + 1, t, level, right_t, right_level, callback);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::embedding_file::builder::write_compressed_dwe_file;

    use super::*;

    use std::io::Read;
    use std::{fs::File, io::Cursor};

    #[test]
    fn pairwise_trajectories() {
        let reader = RandomAccessReader::new(create_sample_file());

        let trajectories = reader
            .pairwise_trajectories(vec![3, 50, 1], vec![70, 3, 12])
            .into_inner();

        const EXPECTED: [f32; 3 * 6] = [
            0.474_921,
            -0.534_683_76,
            -1.491_57,
            -1.549_969_4,
            -1.036_873,
            0.257_684_4,
            0.571_041_35,
            0.091_802_91,
            1.607_687_1,
            1.789_475_1,
            0.371_983_56,
            1.156_625_7,
            0.415_612_67,
            -0.102_255_72,
            0.226_553_22,
            0.674_433_23,
            0.452_879_2,
            -0.746_466_76,
        ];

        assert_eq!(trajectories.len(), EXPECTED.len());
        for (found, expected) in trajectories.iter().zip(&EXPECTED) {
            assert!(
                f32::abs(found - expected) < 1e-6,
                "expected {} but found {}",
                expected,
                found
            );
        }
    }

    #[test]
    fn most_related_to_at_t() {
        let reader = RandomAccessReader::new(create_sample_file());

        const EXPECTED: [[u32; 3 * 10]; 6] = [
            [
                42, 64, 47, 32, 22, 79, 39, 24, 2, 85, 85, 5, 35, 72, 57, 36, 21, 44, 83, 65, 48,
                6, 54, 67, 60, 21, 77, 90, 78, 46,
            ],
            [
                24, 47, 2, 20, 79, 34, 99, 42, 64, 85, 5, 72, 36, 24, 3, 25, 83, 41, 85, 64, 90, 5,
                12, 28, 10, 9, 58, 21, 27, 0,
            ],
            [
                11, 96, 13, 47, 50, 24, 32, 99, 18, 65, 72, 90, 13, 5, 96, 80, 19, 3, 43, 71, 55,
                46, 68, 26, 66, 12, 90, 76, 23, 9,
            ],
            [
                18, 71, 98, 50, 13, 32, 62, 42, 11, 96, 80, 90, 79, 76, 72, 13, 42, 96, 68, 43, 12,
                55, 23, 25, 57, 70, 63, 66, 40, 45,
            ],
            [
                93, 32, 11, 42, 31, 62, 87, 96, 67, 29, 13, 90, 63, 2, 85, 43, 22, 76, 67, 19, 9,
                6, 46, 23, 30, 49, 57, 36, 68, 66,
            ],
            [
                29, 67, 34, 69, 42, 50, 96, 41, 93, 31, 42, 2, 67, 29, 22, 49, 43, 3, 93, 69, 9,
                46, 39, 99, 0, 36, 56, 30, 54, 15,
            ],
        ];

        for t in 0..6 {
            let related_words = reader
                .most_related_to_at_t(vec![3, 34, 4], t, 10)
                .into_inner();

            assert_eq!(related_words, EXPECTED[t as usize]);
        }
    }

    fn create_sample_file() -> EmbeddingFile {
        const NUM_TIMESTEPS: u32 = 6;
        const VOCAB_SIZE: u32 = 100;
        const EMBEDDING_DIM: u32 = 16;

        let file_name = format!(
            "{}/tests/fake_data_generation/random_{}_{}_{}",
            env!("CARGO_MANIFEST_DIR"),
            NUM_TIMESTEPS,
            VOCAB_SIZE,
            EMBEDDING_DIM
        );
        let mut input_file = File::open(file_name).unwrap();

        let mut input_buf = Vec::new();
        input_file.read_to_end(&mut input_buf).unwrap();
        assert_eq!(
            input_buf.len(),
            (NUM_TIMESTEPS * VOCAB_SIZE * EMBEDDING_DIM) as usize
        );

        // Convert to i16.
        let input_buf = input_buf
            .iter()
            .map(|&x| x as i8 as i16)
            .collect::<Vec<_>>();

        // Check that negative values are treated correctly.
        assert_eq!(
            input_buf[(3 * VOCAB_SIZE * EMBEDDING_DIM + 5 * EMBEDDING_DIM + 10) as usize],
            -39
        );

        let uncompressed = RankThreeTensor::from_flattened(
            input_buf,
            NUM_TIMESTEPS as usize,
            VOCAB_SIZE as usize,
            EMBEDDING_DIM as usize,
        );

        const JUMP_INTERVAL: u32 = 20;
        const SCALE_FACTOR: f32 = 0.015_074_313;

        let mut compressed = Vec::<u8>::new();

        write_compressed_dwe_file(
            uncompressed.as_view(),
            JUMP_INTERVAL,
            SCALE_FACTOR,
            &mut compressed,
        )
        .unwrap();
        EmbeddingFile::from_reader(Cursor::new(compressed)).unwrap()
    }
}
