use super::tensors::{RankThreeTensor, RankTwoTensor, RankTwoTensorView, RankTwoTensorViewMut};

use super::compression::Decoder;
use super::embedding_file::{
    CompressedTimestep, EmbeddingData, EmbeddingFile, FileHeader, TimestepReader,
};

use wasm_bindgen::prelude::*;

use std::collections::BinaryHeap;
use std::iter::once;

#[wasm_bindgen]
pub struct RandomAccessReader {
    file: EmbeddingFile,

    /// The height of the tree. The first and last time step each count as one
    /// toward the tree height.
    tree_height: u32,
}

// #[wasm_bindgen]
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

    pub fn pairwise_trajectories(&self, words1: Vec<u32>, words2: Vec<u32>) -> Vec<f32> {
        if words1.is_empty() || words1.len() != words2.len() {
            // TODO: handle error if words1.len() != words2.len()
            return Vec::new();
        }

        let task = PairwiseTrajectories::new(&self.file.header(), words1, words2);
        let traverser = TreeTraverser::new(&self, task);
        traverser.run().into_inner()
    }
}

trait TraversalTask {
    type Output: Default;

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
}

impl<'a, R: TimestepReader> AccumulatingReader<'a, R> {
    fn new(inner: R, left_parent_buf: &'a [i8], right_parent_buf: &'a [i8]) -> Self {
        Self {
            inner,
            left_parent_iter: left_parent_buf.iter(),
            right_parent_iter: right_parent_buf.iter(),
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
                .zip(&mut self.right_parent_iter),
            |diff, ((dest, left), right)| {
                callback((((*left as i32 + *right as i32) / 2) as i8) + diff, dest)
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

        let mut buf = RankThreeTensor::<i8>::new(
            (embeddings.tree_height + 2) as usize,
            task.output_size(),
            header.embedding_dim as usize,
        );

        let mut output = RankTwoTensor::<T::Output>::new(
            header.num_timesteps as usize,
            header.embedding_dim as usize,
        );

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

        for (t, level) in &[(0, 0), (1, header.num_timesteps - 1)] {
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

        self.traverse_subtree(2, 0, 0, header.num_timesteps - 1, 1);

        self.output
    }

    #[allow(dead_code)] // TODO: remove
    fn traverse_subtree(
        &mut self,
        level: u32,
        left_t: u32,
        left_level: u32,
        right_t: u32,
        right_level: u32,
    ) {
        let t = (left_t + right_t) / 2;
        if t != left_t {
            let mut buf_view = self.buf.as_view_mut();
            let (left_parent_view, right_parent_view, mut target_view) =
                buf_view.subviews_rrw(left_level as usize, right_level as usize, level as usize);
            let mut buf_iter_mut = target_view.as_mut_slice().iter_mut();

            let timestep = self.data.timestep(t).unwrap();
            let mut reader = AccumulatingReader::new(
                timestep.reader(),
                left_parent_view.as_slice(),
                right_parent_view.as_slice(),
            );

            let mut output_view = self.output.as_view_mut();
            let output = output_view.subview_mut(t as usize);

            self.task.iter_words(|word| {
                reader
                    .next_diff_vector_in_ascending_order(word, &mut buf_iter_mut, |value, dest| {
                        *dest = value
                    })
                    .unwrap();
            });
            self.task
                .finalize_timestep(t, target_view.downgrade(), output);

            self.traverse_subtree(level + 1, left_t, left_level, t, level);
            self.traverse_subtree(level + 1, t, level, right_t, right_level);
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

        // At this point `unique_words` is sorted but may have duplicates. Remove them.
        // Initialize `last_word` with an invalid value to ensure that it's different
        // from `unique_words[0]`. Since `vocab_size` is a `u32`, `std::u32::MAX` is an
        // an invalid word index.
        let mut last_word = std::u32::MAX;
        unique_words.retain(|current_word| {
            let is_unique = *current_word != last_word;
            last_word = *current_word;
            is_unique
        });

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

    fn output_size(&self) -> usize {
        self.unique_words.len()
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
