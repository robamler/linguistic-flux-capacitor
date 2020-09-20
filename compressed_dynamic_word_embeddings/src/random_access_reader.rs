use std::cmp::Ordering::*;
use std::collections::BinaryHeap;
use std::ops::Range;

use super::embedding_file::{EmbeddingFile, FileHeader, TimestepReader};
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

    pub fn most_related_to_at_t(
        &self,
        target_words: Vec<u32>,
        t: u32,
        amt: u32,
    ) -> RankTwoTensor<u32> {
        MostRelatedToAtT::new(target_words, amt).run(t, &self)
    }

    pub fn largest_changes_wrt(
        &self,
        target_word: u32,
        amt: u32,
        min_increasing: u32,
        min_decreasing: u32,
    ) -> Vec<u32> {
        todo!()
        // let first_timestep_data = self.file.margin_embeddings(0).uncompressed;
        // let last_timestep_data = self.file.margin_embeddings(1).uncompressed;

        // let header = self.file.header();
        // let emb_dim = header.embedding_dim;

        // let first_target = &first_timestep_data
        //     [(target_word * emb_dim) as usize..((target_word + 1) * emb_dim) as usize];
        // let last_target = &last_timestep_data
        //     [(target_word * emb_dim) as usize..((target_word + 1) * emb_dim) as usize];

        // let mut increasing_front_runners = Vec::<FrontRunnerCandidate>::new();
        // increasing_front_runners.resize_with(amt as usize, Default::default);

        // let mut decreasing_front_runners = Vec::<FrontRunnerCandidate>::new();
        // decreasing_front_runners.resize_with(amt as usize, Default::default);

        // for (word, (first_emb, last_emb)) in first_timestep_data
        //     .chunks_exact(emb_dim as usize)
        //     .zip(last_timestep_data.chunks_exact(emb_dim as usize))
        //     .enumerate()
        // {
        //     if word as u32 != target_word {
        //         let first_dot_product = first_target
        //             .iter()
        //             .zip(first_emb)
        //             .map(|(a, b)| *a as i32 * *b as i32)
        //             .sum::<i32>();
        //         let last_dot_product = last_target
        //             .iter()
        //             .zip(last_emb)
        //             .map(|(a, b)| *a as i32 * *b as i32)
        //             .sum::<i32>();

        //         let diff = last_dot_product - first_dot_product;

        //         let increasing_last_better = increasing_front_runners
        //             .iter()
        //             .enumerate()
        //             .rev()
        //             .find(|(_index, fr)| fr.n >= diff);
        //         let increasing_first_worse =
        //             increasing_last_better.map_or(0, |(index, _)| index + 1);
        //         let mut insert_fr = FrontRunnerCandidate {
        //             word: word as u32,
        //             n: diff,
        //         };
        //         for dest in increasing_front_runners[increasing_first_worse..].iter_mut() {
        //             std::mem::swap(dest, &mut insert_fr);
        //         }

        //         let neg_diff = -diff;
        //         let decreasing_last_better = decreasing_front_runners
        //             .iter()
        //             .enumerate()
        //             .rev()
        //             .find(|(_index, fr)| fr.n >= neg_diff);
        //         let decreasing_first_worse =
        //             decreasing_last_better.map_or(0, |(index, _)| index + 1);
        //         let mut insert_fr = FrontRunnerCandidate {
        //             word: word as u32,
        //             n: neg_diff,
        //         };
        //         for dest in decreasing_front_runners[decreasing_first_worse..].iter_mut() {
        //             std::mem::swap(dest, &mut insert_fr);
        //         }
        //     }
        // }

        // let mut combined = Vec::with_capacity((2 * amt) as usize);

        // // Put the required words (`min_increasing` and `min_decreasing`) first.
        // combined.extend_from_slice(&increasing_front_runners[..min_increasing as usize]);
        // combined.extend_from_slice(&decreasing_front_runners[..min_decreasing as usize]);

        // // Then put the remaining items and sort them.
        // combined.extend_from_slice(&increasing_front_runners[min_increasing as usize..]);
        // combined.extend_from_slice(&decreasing_front_runners[min_decreasing as usize..]);

        // // We will keep on only the first half of the list. Sort it as well by magnitude
        // // of the change, so that in particular the first result (which a viewer may
        // // highlight by default) is the one with the largest change in magnitude.
        // combined[..amt as usize].sort_by_key(|fr| -fr.n);

        // // Retain only the `word` part of the first half of the list.
        // combined
        //     .into_iter()
        //     .take(amt as usize)
        //     .map(|fr| fr.word)
        //     .collect()
    }
}

#[derive(Copy, Clone)]
struct FrontRunnerCandidate {
    word: u32,
    n: i32,
}

impl Default for FrontRunnerCandidate {
    fn default() -> Self {
        Self {
            word: std::u32::MAX,
            n: std::i32::MIN,
        }
    }
}

trait TraversalTask {
    type Output: Default + Clone;

    /// Returns the number of embedding vectors needed in some scratch space.
    fn scratch_size(&self) -> usize;

    /// Returns the number of `Self::Output` values calculated per time step.
    fn output_size(&self) -> usize;

    /// Called once per time step. Expects `callback` to be called exactly
    /// `scratch_size` times per time step with arguments (word indices) in strictly
    /// ascending order. The order of words must be the same on each time step.
    fn iter_words(&mut self, callback: impl FnMut(u32));

    /// Called once per time step, after `iter_words`. Provides the embedding
    /// vectors of the words used in calls to `callback` of the `iter_words` method,
    /// in the same order. The slice `output` is of size `self.output_size()` and is
    /// intended to be used to write out results for this time step.
    fn finalize_timestep(
        &mut self,
        t: u32,
        embeddings: RankTwoTensorView<i8>,
        output: &mut [Self::Output],
    );
}

trait SingleTimestepTask: Sized {
    type Output;

    /// Returns the number of embedding vectors needed in some scratch space.
    fn scratch_size(&self) -> usize;

    /// Called once for each node on the path from the root of the tree to the time
    /// step of interest. Expects `callback` to be called exactly `scratch_size`
    /// times per node step with arguments (word indices) in strictly ascending
    /// order. The order of words must be the same on each time step.
    /// This is a workaround for the absence of GAT. With GAT, we would just have
    /// a method `scratch_words_iterator` that returns an iterator. But this is not
    /// possible at the moment because the returned iterator would have to have a
    /// use defined type with a lifetime parameter.
    fn iter_scratch_words<I: Iterator>(&self, driver: I, callback: impl FnMut(I::Item, u32));

    /// Called once for each chunk in the time step of interest. Only called after
    /// all calls to `prepare` are done. `embeddings` holds the entire chunk, and
    /// `scratch` holds the embeddings extracted in the `prepare` calls.
    fn process_chunk(
        &mut self,
        word_range: Range<u32>,
        embeddings: RankTwoTensorView<i8>,
        scratch: RankTwoTensorView<i8>,
    );

    fn finalize(self) -> Self::Output;

    fn run(mut self, t: u32, rar: &RandomAccessReader) -> Self::Output {
        todo!()
        // let header = rar.file.header();
        // let mut center_scratch =
        //     RankTwoTensor::new(self.scratch_size(), header.embedding_dim as usize);

        // if t == 0 || t == header.num_timesteps - 1 {
        //     let level = if t == 0 { 0 } else { 1 };
        //     let mut center_scratch_view_mut = center_scratch.as_view_mut();
        //     let full_timestep_data = rar.file.margin_embeddings(level).uncompressed;
        //     self.iter_scratch_words(
        //         center_scratch_view_mut.iter_mut_subviews(),
        //         |center_emb, word| {
        //             let start = word * header.embedding_dim;
        //             let end = start + header.embedding_dim;
        //             center_emb.copy_from_slice(&full_timestep_data[start as usize..end as usize]);
        //         },
        //     );

        //     let center_scratch_view = center_scratch_view_mut.downgrade();
        //     let total_chunk_size = header.chunk_size * header.embedding_dim;
        //     for (start_word, embeddings) in (0..)
        //         .step_by(header.chunk_size as usize)
        //         .zip(full_timestep_data.chunks_exact(total_chunk_size as usize))
        //     {
        //         let embeddings = RankTwoTensorView::from_flattened(
        //             header.chunk_size,
        //             header.embedding_dim,
        //             embeddings,
        //         );
        //         let word_range = start_word..start_word + header.chunk_size;
        //         self.process_chunk(word_range, embeddings, center_scratch_view);
        //     }
        // } else {
        //     let first_timestep_data = rar.file.margin_embeddings(0).uncompressed;
        //     let mut left_scratch =
        //         RankTwoTensor::new(self.scratch_size(), header.embedding_dim as usize);
        //     let mut left_scratch_view_mut = left_scratch.as_view_mut();

        //     let last_timestep_data = rar.file.margin_embeddings(1).uncompressed;
        //     let mut right_scratch =
        //         RankTwoTensor::new(self.scratch_size(), header.embedding_dim as usize);
        //     let mut right_scratch_view_mut = right_scratch.as_view_mut();

        //     self.iter_scratch_words(
        //         left_scratch_view_mut
        //             .iter_mut_subviews()
        //             .zip(right_scratch_view_mut.iter_mut_subviews()),
        //         |(left_emb, right_emb), word| {
        //             let start = word * header.embedding_dim;
        //             let end = start + header.embedding_dim;
        //             left_emb.copy_from_slice(&first_timestep_data[start as usize..end as usize]);
        //             right_emb.copy_from_slice(&last_timestep_data[start as usize..end as usize]);
        //         },
        //     );

        //     let mut path_from_root = Vec::new();
        //     traverse_subtree(
        //         2,
        //         0,
        //         0,
        //         header.num_timesteps - 1,
        //         1,
        //         &mut |current_t, _level, _left_t, _left_level, _right_t, _right_level| {
        //             let left_embeddings_view = left_scratch.as_view();
        //             let right_embeddings_view = right_scratch.as_view();
        //             let mut center_scratch_view_mut = center_scratch.as_view_mut();

        //             let timestep = rar.file.timestep(current_t).unwrap();
        //             let diff_reader = timestep.reader();
        //             let mut reader = AccumulatingReader::new(
        //                 diff_reader,
        //                 &left_embeddings_view.slice(),
        //                 &right_embeddings_view.slice(),
        //                 header.embedding_dim,
        //             );

        //             self.iter_scratch_words(
        //                 center_scratch_view_mut.iter_mut_subviews(),
        //                 |dest, word| {
        //                     reader
        //                         .next_diff_vector_in_ascending_order(
        //                             word,
        //                             dest.iter_mut(),
        //                             |src, dest| *dest = src,
        //                         )
        //                         .unwrap();
        //                 },
        //             );
        //             path_from_root.push((timestep, current_t));

        //             match current_t.cmp(&t) {
        //                 Less => {
        //                     // Continue to the right half of the interval.
        //                     std::mem::swap(&mut center_scratch, &mut left_scratch);
        //                     (false, true)
        //                 }
        //                 Greater => {
        //                     // Continue to the left half of the interval.
        //                     std::mem::swap(&mut center_scratch, &mut right_scratch);
        //                     (true, false)
        //                 }
        //                 Equal => {
        //                     // Found the node of interest. Stop iteration.
        //                     (false, false)
        //                 }
        //             }
        //         },
        //     );

        //     let total_chunk_size = (header.chunk_size * header.embedding_dim) as usize;
        //     let mut left_buf =
        //         RankTwoTensor::new(header.chunk_size as usize, header.embedding_dim as usize);
        //     let mut right_buf =
        //         RankTwoTensor::new(header.chunk_size as usize, header.embedding_dim as usize);
        //     let mut center_buf =
        //         RankTwoTensor::new(header.chunk_size as usize, header.embedding_dim as usize);

        //     for chunk_index in 0..(header.vocab_size / header.chunk_size) {
        //         let mut left_buf_view_mut = left_buf.as_view_mut();
        //         let mut right_buf_view_mut = right_buf.as_view_mut();

        //         // TODO: This would be much cleaner if wed' just compress the first and second
        //         //       time step as well.
        //         let chunk_begin = chunk_index as usize * total_chunk_size;
        //         let chunk_end = chunk_begin + total_chunk_size;
        //         left_buf_view_mut
        //             .as_mut_slice()
        //             .copy_from_slice(&first_timestep_data[chunk_begin..chunk_end]);
        //         right_buf_view_mut
        //             .as_mut_slice()
        //             .copy_from_slice(&last_timestep_data[chunk_begin..chunk_end]);

        //         for (timestep, current_t) in path_from_root.iter() {
        //             let left_buf_view = left_buf.as_view();
        //             let right_buf_view = right_buf.as_view();
        //             let mut center_buf_view_mut = center_buf.as_view_mut();

        //             let mut diff_chunk = timestep.chunk(chunk_index).unwrap();
        //             let dest_iter = center_buf_view_mut
        //                 .as_mut_slice()
        //                 .iter_mut()
        //                 .zip(left_buf_view.slice().iter())
        //                 .zip(right_buf_view.slice().iter());
        //             diff_chunk
        //                 .decode(dest_iter, |diff, ((dest, left), right)| {
        //                     let prediction = ((*left as i32 + *right as i32) / 2) as i8;
        //                     *dest = prediction.wrapping_add(diff as i8);
        //                 })
        //                 .unwrap();
        //             assert!(diff_chunk.could_be_end());

        //             match current_t.cmp(&t) {
        //                 Less => std::mem::swap(&mut center_buf, &mut left_buf),
        //                 Greater => std::mem::swap(&mut center_buf, &mut right_buf),
        //                 Equal => (),
        //             }
        //         }

        //         let word_range =
        //             chunk_index * header.chunk_size..(chunk_index + 1) * header.chunk_size;
        //         self.process_chunk(word_range, center_buf.as_view(), center_scratch.as_view());
        //     }
        // }

        // self.finalize()
    }
}

struct MostRelatedToAtT {
    unique_words: Vec<u32>,
    target_word_indices: Vec<u32>,
    amt: u32,
    front_runners: RankTwoTensor<FrontRunnerCandidate>,
}

impl MostRelatedToAtT {
    fn new(target_words: Vec<u32>, amt: u32) -> Self {
        let mut unique_words = target_words
            .iter()
            .cloned()
            .collect::<BinaryHeap<u32>>()
            .into_sorted_vec();
        unique_words.dedup();

        let mut target_word_indices = target_words;
        for word in target_word_indices.iter_mut() {
            *word = unique_words.binary_search(word).unwrap() as u32;
        }

        let front_runners = RankTwoTensor::new(unique_words.len(), amt as usize);

        Self {
            unique_words,
            target_word_indices,
            amt,
            front_runners,
        }
    }
}

impl SingleTimestepTask for MostRelatedToAtT {
    type Output = RankTwoTensor<u32>;

    fn scratch_size(&self) -> usize {
        self.unique_words.len()
    }

    fn iter_scratch_words<I: Iterator>(&self, driver: I, mut callback: impl FnMut(I::Item, u32)) {
        for (i, word) in driver.zip(self.unique_words.iter()) {
            callback(i, *word);
        }
    }

    fn process_chunk(
        &mut self,
        word_range: Range<u32>,
        embeddings: RankTwoTensorView<i8>,
        scratch: RankTwoTensorView<i8>,
    ) {
        for (word, word_emb) in word_range.zip(embeddings.iter_subviews()) {
            for ((main_word, main_emb), front_runners) in self
                .unique_words
                .iter()
                .zip(scratch.iter_subviews())
                .zip(self.front_runners.as_view_mut().iter_mut_subviews())
            {
                if *main_word != word {
                    let dot_product = main_emb
                        .iter()
                        .zip(word_emb)
                        .map(|(a, b)| *a as i32 * *b as i32)
                        .sum::<i32>();

                    let last_better = front_runners
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_index, fr)| fr.n >= dot_product);
                    let first_worse = last_better.map_or(0, |(index, _)| index + 1);
                    let mut insert_fr = FrontRunnerCandidate {
                        word,
                        n: dot_product,
                    };
                    for dest in front_runners[first_worse..].iter_mut() {
                        std::mem::swap(dest, &mut insert_fr);
                    }
                }
            }
        }
    }

    fn finalize(self) -> RankTwoTensor<u32> {
        let mut reordered_top_k =
            RankTwoTensor::new(self.target_word_indices.len(), self.amt as usize);
        let mut reordered_top_k_view_mut = reordered_top_k.as_view_mut();
        let front_runners_view = self.front_runners.as_view();

        for (word_index, dest) in self
            .target_word_indices
            .iter()
            .zip(reordered_top_k_view_mut.iter_mut_subviews())
        {
            for (dest_val, fr) in dest
                .iter_mut()
                .zip(front_runners_view.subview(*word_index as usize))
            {
                *dest_val = fr.word
            }
        }

        reordered_top_k
    }
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

// impl<'a, R: TimestepReader> TimestepReader for AccumulatingReader<'a, R> {
//     fn next_diff_vector_in_ascending_order<I: Iterator>(
//         &mut self,
//         index: u32,
//         dest_iter: I,
//         mut callback: impl FnMut(i16, I::Item),
//     ) -> Result<(), ()> {
//         self.inner.next_diff_vector_in_ascending_order(
//             index,
//             dest_iter
//                 .zip(&mut self.left_parent_iter)
//                 .zip(&mut self.right_parent_iter)
//                 .take(self.embedding_dim as usize),
//             |diff, ((dest, left), right)| {
//                 let prediction = (*left as i32 + *right as i32) / 2;
//                 callback((prediction + diff as i32) as i16, dest)
//             },
//         )
//     }
// }

struct TreeTraverser<'a, T: TraversalTask> {
    buf: RankThreeTensor<i8>,
    output: RankTwoTensor<T::Output>,
    task: T,
    data: &'a EmbeddingFile,
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
        todo!()
        // let header = self.data.header();
        // let mut buf_view = self.buf.as_view_mut();
        // let mut output_view = self.output.as_view_mut();

        // for (t, level) in &[(0, 0), (header.num_timesteps - 1, 1)] {
        //     let mut buf_subview = buf_view.subview_mut(*level as usize);
        //     let mut reader = self.data.margin_embeddings(*level);
        //     let mut buf_iter_mut = buf_subview.as_mut_slice().iter_mut();
        //     let output = output_view.subview_mut(*t as usize);

        //     self.task.iter_words(|word| {
        //         reader
        //             .next_diff_vector_in_ascending_order(word, &mut buf_iter_mut, |n, dest| {
        //                 *dest = n
        //             })
        //             .unwrap();
        //     });

        //     self.task
        //         .finalize_timestep(*t, buf_subview.downgrade(), output);
        // }

        // traverse_subtree(
        //     2,
        //     0,
        //     0,
        //     header.num_timesteps - 1,
        //     1,
        //     &mut |t, level, _left_t, left_level, _right_t, right_level| {
        //         let mut buf_view = self.buf.as_view_mut();
        //         let (left_parent_view, right_parent_view, mut target_view) = buf_view.subviews_rrw(
        //             left_level as usize,
        //             right_level as usize,
        //             level as usize,
        //         );
        //         let mut buf_iter_mut = target_view.as_mut_slice().iter_mut();

        //         let timestep = self.data.timestep(t).unwrap();
        //         let mut reader = AccumulatingReader::new(
        //             timestep.reader(),
        //             left_parent_view.slice(),
        //             right_parent_view.slice(),
        //             self.data.header().embedding_dim,
        //         );

        //         let mut output_view = self.output.as_view_mut();
        //         let output = output_view.subview_mut(t as usize);

        //         self.task.iter_words(|word| {
        //             reader
        //                 .next_diff_vector_in_ascending_order(word, &mut buf_iter_mut, |n, dest| {
        //                     *dest = n
        //                 })
        //                 .unwrap();
        //         });
        //         self.task
        //             .finalize_timestep(t, target_view.downgrade(), output);

        //         (true, true)
        //     },
        // );

        // self.output.as_view().to_transposed()
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

    /// TODO: replace this by a function that loads a pre-compressed file from disk.
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

        // Convert to i16.
        let input_buf = input_buf
            .iter()
            .map(|&x| x as i8 as i16)
            .collect::<Vec<_>>();

        // Check that negative values are treated correctly.
        assert_eq!(
            input_buf[(3 * VOCAB_SIZE * EMBEDDING_DIM + 5 * EMBEDDING_DIM + 10) as usize] as i8,
            -39
        );

        let uncompressed = RankThreeTensor::from_flattened(
            input_buf,
            NUM_TIMESTEPS as usize,
            VOCAB_SIZE as usize,
            EMBEDDING_DIM as usize,
        );

        let chunk_size = 20;
        let scale_factor = 0.000_227_234_92;

        todo!();
        // EmbeddingFile::from_uncompressed_quantized(uncompressed.as_view(), chunk_size, scale_factor)
        //     .unwrap()
    }

    #[test]
    fn sqrt_u32() {
        /// Calculates the integer square root bounded by one.
        ///
        /// Returns `max(1, sqrt(n))`, where `sqrt(n)` is the square root of `n`,
        /// rounded either up or down.
        fn sqrt_at_least_one(n: u32) -> u32 {
            let n = n | 1;
            let shift = (32 - n.leading_zeros()) / 2;
            let mut xn = 1 << shift;

            // Three Newton iterations turn out to be enough for all numbers in the range of u32.
            xn = (xn + (n >> shift)) / 2; // `(n >> shift) == n / xn` here but slightly faster.
            xn = (xn + n / xn) / 2;
            xn = (xn + n / xn) / 2;
            xn
        }

        assert_eq!(1, sqrt_at_least_one(0));

        for i in 1..(256 * 256) {
            assert_eq!(i, sqrt_at_least_one(i * i));
        }

        // This complete test also passes, but it takes about 33 seconds on my machine.
        // for i in 1..=u32::max_value() {
        //     let sqrt = sqrt_at_least_one(i);
        //     assert!((sqrt - 1) * (sqrt - 1) < i);
        //     assert!((sqrt + 1) as u64 * (sqrt + 1) as u64 > i as u64);
        // }
    }
}
