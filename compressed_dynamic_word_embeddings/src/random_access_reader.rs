use std::cmp::Ordering::*;
use std::collections::BinaryHeap;

use super::embedding_file::{EmbeddingData, EmbeddingFile, FileHeader, TimestepReader};
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

    pub fn pairwise_trajectories(&self, words1: Vec<u32>, words2: Vec<u32>) -> RankTwoTensor<f32> {
        if words1.is_empty() || words1.len() != words2.len() {
            // TODO: handle error if words1.len() != words2.len()
            return RankTwoTensor::new(0, self.file.header().num_timesteps as usize);
        }

        let task = PairwiseTrajectories::new(&self.file.header(), words1, words2);
        let traverser = TreeTraverser::new(&self, task);
        traverser.run()
    }

    pub fn most_related_to_2_words(&self, t: u32, word1: u32, word2: u32) -> [[u32; 7]; 2] {
        let (smaller_word, larger_word) = if word1 < word2 {
            (word1, word2)
        } else {
            (word2, word1)
        };
        let header = self.file.header();
        assert!(t < header.num_timesteps);

        if t == 0 || t == header.num_timesteps - 1 {
            todo!()
        } else {
            let start_smaller = smaller_word * header.embedding_dim;
            let end_smaller = start_smaller + header.embedding_dim;
            let start_larger = larger_word * header.embedding_dim;
            let end_larger = start_larger + header.embedding_dim;

            let mut left_embeddings = Vec::new();
            left_embeddings.reserve_exact((2 * header.embedding_dim) as usize);
            let first_timestep_data = self.file.margin_embeddings(0).uncompressed;
            left_embeddings.extend_from_slice(
                &first_timestep_data[start_smaller as usize..end_smaller as usize],
            );
            left_embeddings.extend_from_slice(
                &first_timestep_data[start_larger as usize..end_larger as usize],
            );

            let mut right_embeddings = Vec::new();
            right_embeddings.reserve_exact((2 * header.embedding_dim) as usize);
            let last_timestep_data = self.file.margin_embeddings(1).uncompressed;
            right_embeddings.extend_from_slice(
                &last_timestep_data[start_smaller as usize..end_smaller as usize],
            );
            right_embeddings
                .extend_from_slice(&last_timestep_data[start_larger as usize..end_larger as usize]);

            let mut center_embeddings = Vec::new();
            center_embeddings.resize((2 * header.embedding_dim) as usize, 0);

            let mut path_from_root = Vec::new();

            traverse_subtree(
                2,
                0,
                0,
                header.num_timesteps - 1,
                1,
                &mut |current_t, _level, _left_t, left_level, _right_t, right_level| {
                    let timestep = self.file.timestep(t).unwrap();
                    let diff_reader = timestep.reader();
                    let mut reader = AccumulatingReader::new(
                        diff_reader,
                        &left_embeddings,
                        &right_embeddings,
                        header.embedding_dim,
                    );
                    let mut dest_iter = &mut center_embeddings.iter_mut();

                    reader
                        .next_diff_vector_in_ascending_order(
                            smaller_word,
                            &mut dest_iter,
                            |src, dest| *dest = src,
                        )
                        .unwrap();
                    reader
                        .next_diff_vector_in_ascending_order(
                            larger_word,
                            &mut dest_iter,
                            |src, dest| *dest = src,
                        )
                        .unwrap();

                    path_from_root.push((timestep, current_t));

                    match current_t.cmp(&t) {
                        Less => {
                            // Continue to the right half of the interval.
                            std::mem::swap(&mut center_embeddings, &mut left_embeddings);
                            (false, true)
                        }
                        Greater => {
                            // Continue to the left half of the interval.
                            std::mem::swap(&mut center_embeddings, &mut right_embeddings);
                            (false, true)
                        }
                        Equal => {
                            // Found the node of interest. Stop iteration.
                            (false, false)
                        }
                    }
                },
            );

            let (smaller_embedding, larger_embedding) =
                center_embeddings.split_at(header.embedding_dim as usize);
            let (word1_embedding, word2_embedding) = if word1 < word2 {
                (smaller_embedding, larger_embedding)
            } else {
                (larger_embedding, smaller_embedding)
            };

            let total_chunk_size = (header.chunk_size * header.embedding_dim) as usize;
            let mut left_buf = Vec::new();
            let mut right_buf = Vec::new();
            let mut center_buf = Vec::new();
            center_buf.resize(total_chunk_size, 0);

            // Tuples of (dot_product, word)
            let mut front_runners = [[(std::i32::MIN, std::u32::MAX); 7]; 2];

            for chunk_index in 0..(header.vocab_size / header.chunk_size) {
                // TODO: This would be much cleaner if wed' just compress the first and second
                //       time step as well.
                let chunk_begin = chunk_index as usize * total_chunk_size;
                let chunk_end = chunk_begin + total_chunk_size;
                left_buf.clear();
                left_buf.extend_from_slice(&first_timestep_data[chunk_begin..chunk_end]);
                right_buf.clear();
                right_buf.extend_from_slice(&last_timestep_data[chunk_begin..chunk_end]);

                for (timestep, current_t) in path_from_root.iter() {
                    let mut diff_chunk = timestep.chunk(chunk_index).unwrap();
                    let dest_iter = center_buf
                        .iter_mut()
                        .zip(left_buf.iter())
                        .zip(right_buf.iter());
                    diff_chunk
                        .decode(dest_iter, |diff, ((dest, left), right)| {
                            let prediction = ((*left as i32 + *right as i32) / 2) as i8;
                            *dest = prediction.wrapping_add(diff as i8);
                        })
                        .unwrap();
                    diff_chunk.finish().unwrap();

                    match current_t.cmp(&t) {
                        Less => std::mem::swap(&mut center_buf, &mut left_buf),
                        Greater => std::mem::swap(&mut center_buf, &mut right_buf),
                        Equal => (),
                    }
                }

                let mut buf_iter = center_buf.iter();
                for word in chunk_index * header.chunk_size..(chunk_index + 1) * header.chunk_size {
                    let mut dot_product1 = 0i32;
                    let mut dot_product2 = 0i32;
                    for ((w1_value, w2_value), buf_value) in word1_embedding
                        .iter()
                        .zip(word2_embedding)
                        .zip(&mut buf_iter)
                    {
                        dot_product1 += *buf_value as i32 * *w1_value as i32;
                        dot_product2 += *buf_value as i32 * *w2_value as i32;
                    }

                    for ((front_runners, dot_product), main_word) in front_runners
                        .iter_mut()
                        .zip(&[dot_product1, dot_product2])
                        .zip(&[word1, word2])
                    {
                        if word != *main_word {
                            let last_better = front_runners
                                .iter()
                                .enumerate()
                                .rev()
                                .find(|(_index, (prod, _))| prod >= dot_product);
                            let first_worse = last_better.map_or(0, |(index, _)| index + 1);
                            let mut insert_value = (*dot_product, word);
                            for dest in front_runners[first_worse..].iter_mut() {
                                std::mem::swap(dest, &mut insert_value);
                            }
                        }
                    }
                }
            }
        }
        todo!()
    }
}

fn search_timestep(t: u32, callback: &mut impl FnMut()) {}

trait TraversalTask {
    type Output: Default + Clone;

    fn scratch_size(&self) -> usize;

    fn output_size(&self) -> usize;

    fn iter_words(&mut self, callback: impl FnMut(u32));

    fn finalize_timestep(
        &mut self,
        t: u32,
        embeddings: RankTwoTensorView<i8>,
        output: &mut [Self::Output],
    );
}

struct AccumulatingReader<'a, R: TimestepReader> {
    inner: R,
    left_parent_iter: std::slice::Iter<'a, i8>,
    right_parent_iter: std::slice::Iter<'a, i8>,
    embedding_dim: u32,
}

impl<'a, R: TimestepReader> AccumulatingReader<'a, R> {
    fn new(
        inner: R,
        left_parent_buf: &'a [i8],
        right_parent_buf: &'a [i8],
        embedding_dim: u32,
    ) -> Self {
        Self {
            inner,
            left_parent_iter: left_parent_buf.iter(),
            right_parent_iter: right_parent_buf.iter(),
            embedding_dim,
        }
    }
}

impl<'a, R: TimestepReader> TimestepReader for AccumulatingReader<'a, R> {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        mut callback: impl FnMut(i8, I::Item),
    ) -> Result<(), ()> {
        self.inner.next_diff_vector_in_ascending_order(
            index,
            dest_iter
                .zip(&mut self.left_parent_iter)
                .zip(&mut self.right_parent_iter)
                .take(self.embedding_dim as usize),
            |diff, ((dest, left), right)| {
                let prediction = ((*left as i32 + *right as i32) / 2) as i8;
                callback(prediction.wrapping_add(diff), dest)
            },
        )
    }
}

struct TreeTraverser<'a, T: TraversalTask> {
    buf: RankThreeTensor<i8>,
    output: RankTwoTensor<T::Output>,
    task: T,
    data: &'a EmbeddingData,
}

impl<'a, T: TraversalTask> TreeTraverser<'a, T> {
    fn new(embeddings: &'a RandomAccessReader, task: T) -> Self {
        let header = &embeddings.file.header();

        let buf = RankThreeTensor::<i8>::new(
            embeddings.tree_height as usize,
            task.scratch_size(),
            header.embedding_dim as usize,
        );

        let output =
            RankTwoTensor::<T::Output>::new(header.num_timesteps as usize, task.output_size());

        Self {
            buf,
            output,
            task,
            data: &embeddings.file,
        }
    }

    fn run(mut self) -> RankTwoTensor<T::Output> {
        let header = self.data.header();
        let mut buf_view = self.buf.as_view_mut();
        let mut output_view = self.output.as_view_mut();

        for (t, level) in &[(0, 0), (header.num_timesteps - 1, 1)] {
            let mut buf_subview = buf_view.subview_mut(*level as usize);
            let mut reader = self.data.margin_embeddings(*level);
            let mut buf_iter_mut = buf_subview.as_mut_slice().iter_mut();
            let output = output_view.subview_mut(*t as usize);

            self.task.iter_words(|word| {
                reader
                    .next_diff_vector_in_ascending_order(word, &mut buf_iter_mut, |value, dest| {
                        *dest = value
                    })
                    .unwrap();
            });

            self.task
                .finalize_timestep(*t, buf_subview.downgrade(), output);
        }

        traverse_subtree(
            2,
            0,
            0,
            header.num_timesteps - 1,
            1,
            &mut |t, level, _left_t, left_level, _right_t, right_level| {
                let mut buf_view = self.buf.as_view_mut();
                let (left_parent_view, right_parent_view, mut target_view) = buf_view.subviews_rrw(
                    left_level as usize,
                    right_level as usize,
                    level as usize,
                );
                let mut buf_iter_mut = target_view.as_mut_slice().iter_mut();

                let timestep = self.data.timestep(t).unwrap();
                let mut reader = AccumulatingReader::new(
                    timestep.reader(),
                    left_parent_view.slice(),
                    right_parent_view.slice(),
                    self.data.header().embedding_dim,
                );

                let mut output_view = self.output.as_view_mut();
                let output = output_view.subview_mut(t as usize);

                self.task.iter_words(|word| {
                    reader
                        .next_diff_vector_in_ascending_order(
                            word,
                            &mut buf_iter_mut,
                            |value, dest| *dest = value,
                        )
                        .unwrap();
                });
                self.task
                    .finalize_timestep(t, target_view.downgrade(), output);

                (true, true)
            },
        );

        self.output.as_view().to_transposed()
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

struct PairwiseTrajectories {
    unique_words: Vec<u32>,
    words1: Vec<u32>,
    words2: Vec<u32>,
    scale_factor: f32,
}

impl PairwiseTrajectories {
    pub fn new(header: &FileHeader, mut words1: Vec<u32>, mut words2: Vec<u32>) -> Self {
        let mut unique_words = words1
            .iter()
            .chain(words2.iter())
            .cloned()
            .collect::<BinaryHeap<u32>>()
            .into_sorted_vec();
        unique_words.dedup();

        // Replace entries of `words1` and `words2` with their indices into `unique_words`.
        for word in words1.iter_mut().chain(words2.iter_mut()) {
            *word = unique_words.binary_search(word).unwrap() as u32;
        }

        Self {
            words1,
            words2,
            unique_words,
            scale_factor: header.scale_factor,
        }
    }
}

impl TraversalTask for PairwiseTrajectories {
    type Output = f32;

    fn scratch_size(&self) -> usize {
        self.unique_words.len()
    }

    fn output_size(&self) -> usize {
        self.words1.len()
    }

    fn iter_words(&mut self, mut callback: impl FnMut(u32)) {
        for word in &self.unique_words {
            callback(*word);
        }
    }

    fn finalize_timestep(
        &mut self,
        _t: u32,
        embeddings: RankTwoTensorView<i8>,
        output: &mut [Self::Output],
    ) {
        for ((w1, w2), dest) in self
            .words1
            .iter()
            .zip(self.words2.iter())
            .zip(output.iter_mut())
        {
            let embedding1 = embeddings.subview(*w1 as usize);
            let embedding2 = embeddings.subview(*w2 as usize);
            let dot_product: i32 = embedding1
                .iter()
                .zip(embedding2)
                .map(|(x1, x2)| *x1 as i32 * *x2 as i32)
                .sum();
            *dest = self.scale_factor * dot_product as f32;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fs::File;
    use std::io::Read;

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

    /// TODO: replace this by a function that loads a precompressed file from disk.
    fn create_sample_file() -> EmbeddingFile {
        const NUM_TIMESTEPS: u32 = 6;
        const VOCAB_SIZE: u32 = 100;
        const EMBEDDING_DIM: u32 = 16;

        let file_name = format!(
            "tests/fake_data_generation/random_{}_{}_{}",
            NUM_TIMESTEPS, VOCAB_SIZE, EMBEDDING_DIM
        );
        let mut input_file = File::open(file_name).unwrap();

        let mut input_buf = Vec::new();
        input_file.read_to_end(&mut input_buf).unwrap();
        assert_eq!(
            input_buf.len(),
            (NUM_TIMESTEPS * VOCAB_SIZE * EMBEDDING_DIM) as usize
        );

        // Check that negative values are treated correctly.
        assert_eq!(
            input_buf[(3 * VOCAB_SIZE * EMBEDDING_DIM + 5 * EMBEDDING_DIM + 10) as usize] as i8,
            -39
        );

        let uncompressed = RankThreeTensor::from_flattened(
            u8_slice_to_i8_slice(&input_buf).to_vec(),
            NUM_TIMESTEPS as usize,
            VOCAB_SIZE as usize,
            EMBEDDING_DIM as usize,
        );

        let chunk_size = 20;
        let scale_factor = 0.000_227_234_92;

        EmbeddingFile::from_uncompressed_quantized(uncompressed.as_view(), chunk_size, scale_factor)
            .unwrap()
    }

    fn u8_slice_to_i8_slice(data: &[u8]) -> &[i8] {
        unsafe {
            let ptr = data.as_ptr();
            std::slice::from_raw_parts_mut(ptr as *mut i8, data.len())
        }
    }
}
