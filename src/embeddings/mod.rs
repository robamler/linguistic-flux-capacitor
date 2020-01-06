mod tensors;

use tensors::{RankThreeTensor, RankTwoTensor, RankTwoTensorView, RankTwoTensorViewMut};

use crate::ans::Decoder;
use crate::embedding_file::{CompressedTimestep, EmbeddingFile, TimestepReader};

use wasm_bindgen::prelude::*;

use std::collections::BinaryHeap;
use std::iter::once;

#[wasm_bindgen]
struct DynamicEmbeddings {
    file: EmbeddingFile,

    /// The height of the tree. The uncompressed representations of the first
    /// and last time step each count one to the tree height.
    tree_height: u32,
}

// #[wasm_bindgen]
impl DynamicEmbeddings {
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

    pub fn pairwise_trajectories(&self, mut words1: Vec<u32>, mut words2: Vec<u32>) -> Vec<f32> {
        if words1.is_empty() || words1.len() != words2.len() {
            return Vec::new();
        }
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

        // Replace entries of `words1` and `words2` by their indices into `unique_words`.
        for word in words1.iter_mut().chain(words2.iter_mut()) {
            *word = unique_words.binary_search(word).unwrap() as u32;
        }
        let header = self.file.header();

        let mut trajectories =
            RankTwoTensor::<f32>::new(header.num_timesteps as usize, header.embedding_dim as usize);
        let mut trajectories_view = trajectories.as_view_mut();

        let mut vectors = RankThreeTensor::<i8>::new(
            (self.tree_height + 2) as usize,
            unique_words.len(),
            header.vocab_size as usize,
        );
        let mut vectors_view = vectors.as_view_mut();

        // TODO: factor out this three-fold code duplication.
        let mut source = self.file.margin_embeddings(0);
        let mut buffer = vectors_view.subview_mut(0);
        for word in unique_words.iter() {
            source
                .next_diff_vector_in_ascending_order(
                    *word,
                    buffer.subview_mut(*word as usize).iter_mut(),
                    |value, dest| *dest = value,
                )
                .unwrap();
        }

        for ((w1, w2), dest) in words1
            .iter()
            .zip(words2.iter())
            .zip(trajectories_view.subview_mut(0).iter_mut())
        {
            let embedding1 = buffer.subview(*w1 as usize);
            let embedding2 = buffer.subview(*w2 as usize);
            let dot_product: i32 = embedding1
                .iter()
                .zip(embedding2)
                .map(|(x1, x2)| *x1 as i32 * *x2 as i32)
                .sum();
            *dest = header.scale_factor * dot_product as f32;
        }

        let mut source = self.file.margin_embeddings(1);
        let mut buffer = vectors_view.subview_mut(1);
        for word in unique_words.iter() {
            source
                .next_diff_vector_in_ascending_order(
                    *word,
                    buffer.subview_mut(*word as usize).iter_mut(),
                    |value, dest| *dest = value,
                )
                .unwrap();
        }

        self.traverse_inner_timesteps(|t, _| {
            let timestep = self.file.timestep(t);
            true
        });

        todo!();
        Vec::new()
    }

    fn traverse_inner_timesteps(&self, mut callback: impl FnMut(u32, u32) -> bool) {
        preorder_subtree_traversal(0, self.file.header().num_timesteps - 1, 0, &mut callback);
    }
}

fn preorder_subtree_traversal(
    left: u32,
    right: u32,
    level: u32,
    callback: &mut impl FnMut(u32, u32) -> bool,
) {
    let next_index = (left + right) / 2;
    if next_index != left && callback(next_index, level) {
        preorder_subtree_traversal(left, next_index, level + 1, callback);
        preorder_subtree_traversal(next_index, right, level + 1, callback);
    }
}

trait TraversalTask {
    type Output;

    fn process_timestep(
        &mut self,
        t: u32,
        embeddings: impl TimestepReader,
        output: &mut [Self::Output],
    );
}

struct TeeReader<'a, R: TimestepReader> {
    inner: R,
    target_iter: std::slice::IterMut<'a, i8>,
}

impl<'a, R: TimestepReader> TeeReader<'a, R> {
    fn new(inner: R, buf: &'a mut [i8]) -> Self {
        Self {
            inner,
            target_iter: buf.iter_mut(),
        }
    }
}

impl<'a, R: TimestepReader> TimestepReader for TeeReader<'a, R> {
    fn next_diff_vector_in_ascending_order<I: Iterator>(
        &mut self,
        index: u32,
        dest_iter: I,
        mut callback: impl FnMut(i8, I::Item),
    ) -> Result<(), ()> {
        self.inner.next_diff_vector_in_ascending_order(
            index,
            dest_iter.zip(&mut self.target_iter),
            |src, (external_dest, my_dest)| {
                *my_dest = src;
                callback(src, external_dest)
            },
        )
    }
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
            |diff, ((dest, left), right)| callback((left + right) / 2 + diff, dest),
        )
    }
}

struct TreeTraverser<T: TraversalTask> {
    buf: RankThreeTensor<i8>,
    output: RankTwoTensor<T::Output>,
    task: T,
    file: EmbeddingFile,
}

impl<T: TraversalTask> TreeTraverser<T> {
    fn run(mut self) -> RankTwoTensor<T::Output> {
        let header = self.file.header();
        let mut buf_view = self.buf.as_view_mut();
        let mut output_view = self.output.as_view_mut();

        for (t, level) in &[(0, 0), (1, header.num_timesteps - 1)] {
            let mut subview = buf_view.subview_mut(*level as usize);
            let reader =
                TeeReader::new(self.file.margin_embeddings(*level), subview.as_mut_slice());
            let output = output_view.subview_mut(*t as usize);
            self.task.process_timestep(*t, reader, output);
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
            let mut output_view = self.output.as_view_mut();
            let output = output_view.subview_mut(t as usize);

            let (left_parent_view, right_parent_view, mut target_view) =
                buf_view.subviews_rrw(left_level as usize, right_level as usize, level as usize);

            let timestep = self.file.timestep(t).unwrap();
            let reader = AccumulatingReader::new(
                timestep.reader(),
                left_parent_view.as_slice(),
                right_parent_view.as_slice(),
            );

            // TODO: we don't need a TeeReader on the last level.
            let reader = TeeReader::new(reader, target_view.as_mut_slice());
            self.task.process_timestep(t, reader, output);

            self.traverse_subtree(level + 1, left_t, left_level, t, level);
            self.traverse_subtree(level + 1, t, level, right_t, right_level);
        }
    }
}

struct PairwiseTrajectories {
    file: EmbeddingFile,
    unique_words: Vec<u32>,
    words1: Vec<u32>,
    words2: Vec<u32>,
    scale_factor: f32,
}

// impl TraversalTask for PairwiseTrajectories {
//     type Output = f32;

//     fn process_margin_timestep(
//         &mut self,
//         t: u32,
//         level: u32,
//         input: &mut RankTwoTensorViewMut<i8>,
//         output: &mut [Self::Output],
//     ) -> bool {
//         let mut source = self.file.margin_embeddings(level);
//         for word in self.unique_words.iter() {
//             source
//                 .next_diff_vector_in_ascending_order(
//                     *word,
//                     buf.subview_mut(*word as usize).iter_mut(),
//                     |value, dest| *dest = value,
//                 )
//                 .unwrap();
//         }

//         true
//     }

//     fn finalize_timestep(
//         &mut self,
//         t: u32,
//         buf: &mut RankTwoTensorViewMut<i8>,
//         output: &mut [Self::Output],
//     ) -> bool {
//         for ((w1, w2), dest) in self
//             .words1
//             .iter()
//             .zip(self.words2.iter())
//             .zip(output.iter_mut())
//         {
//             let embedding1 = buf.subview(*w1 as usize);
//             let embedding2 = buf.subview(*w2 as usize);
//             *dest = self.scale_factor
//                 * embedding1
//                     .iter()
//                     .zip(embedding2)
//                     .map(|(x1, x2)| *x1 as i32 * *x2 as i32)
//                     .sum::<i32>() as f32;
//         }

//         true
//     }

//     /// Returns true if search should proceed down this subtree.
//     fn process_inner_timestep(
//         &mut self,
//         t: u32,
//         level: u32,
//         buf: &mut RankTwoTensorViewMut<i8>,
//         output: &mut [Self::Output],
//         left_parent_t: u32,
//         left_parent_buf: &mut RankTwoTensorViewMut<i8>,
//         right_parent_t: u32,
//         right_parent_buf: &mut RankTwoTensorViewMut<i8>,
//     ) -> bool {
//         todo!()
//     }
// }

// TODO: remove if not needed
fn make_unique<T: Eq + Clone>(sorted: &mut Vec<T>, invalid_value: T) {
    let mut last_entry = invalid_value;
    sorted.retain(|current_entry| {
        let is_unique = *current_entry != last_entry;
        last_entry = current_entry.clone();
        is_unique
    });
}
