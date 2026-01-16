pub struct RankThreeTensor<T> {
    stride0: usize,
    stride1: usize,
    data: Box<[T]>,
}

impl<T: Default> RankThreeTensor<T> {
    pub fn new(shape0: usize, shape1: usize, shape2: usize) -> Self {
        let stride0 = shape1 * shape2;
        let len = shape0 * stride0;
        let mut data = Vec::new();
        data.resize_with(len, Default::default);

        Self {
            stride0,
            stride1: shape2,
            data: data.into(),
        }
    }

    pub fn from_flattened(data: Vec<T>, shape0: usize, shape1: usize, shape2: usize) -> Self {
        let stride0 = shape1 * shape2;
        assert_eq!(data.len(), shape0 * stride0);

        Self {
            stride0,
            stride1: shape2,
            data: data.into(),
        }
    }

    pub fn as_view(&self) -> RankThreeTensorView<'_, T> {
        RankThreeTensorView {
            stride0: self.stride0,
            stride1: self.stride1,
            data: &self.data,
        }
    }

    pub fn as_view_mut(&mut self) -> RankThreeTensorViewMut<'_, T> {
        RankThreeTensorViewMut {
            stride0: self.stride0,
            stride1: self.stride1,
            data: &mut self.data,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RankThreeTensorView<'a, T> {
    stride0: usize,
    stride1: usize,
    data: &'a [T],
}

impl<'a, T> RankThreeTensorView<'a, T> {
    fn from_raw_parts(stride0: usize, stride1: usize, data: &'a [T]) -> Self {
        Self {
            stride0,
            stride1,
            data,
        }
    }

    pub fn shape(self) -> (usize, usize, usize) {
        (
            self.data.len() / self.stride0,
            self.stride0 / self.stride1,
            self.stride1,
        )
    }
    pub fn subview(self, index0: usize) -> RankTwoTensorView<'a, T> {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        RankTwoTensorView::from_raw_parts(self.stride1, &self.data[start..end])
    }

    pub fn slice(self) -> &'a [T] {
        self.data
    }
}

pub struct RankThreeTensorViewMut<'a, T> {
    stride0: usize,
    stride1: usize,
    data: &'a mut [T],
}

impl<T> RankThreeTensorViewMut<'_, T> {
    pub fn subview_mut(&mut self, index0: usize) -> RankTwoTensorViewMut<'_, T> {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        RankTwoTensorViewMut::from_raw_parts_mut(self.stride1, &mut self.data[start..end])
    }

    pub fn subviews_rrw(
        &mut self,
        read1_index: usize,
        read2_index: usize,
        write_index: usize,
    ) -> (
        RankTwoTensorView<'_, T>,
        RankTwoTensorView<'_, T>,
        RankTwoTensorViewMut<'_, T>,
    ) {
        let read1_start = read1_index * self.stride0;
        let read2_start = read2_index * self.stride0;
        let write_start = write_index * self.stride0;

        let max_start = self.data.len() - self.stride0;
        assert!(
            read1_index != write_index
                && read2_index != write_index
                && read1_start <= max_start
                && read2_start <= max_start
                && write_start <= max_start
        );

        unsafe {
            let ptr = self.data.as_mut_ptr();
            (
                RankTwoTensorView::from_raw_parts(
                    self.stride1,
                    std::slice::from_raw_parts(ptr.add(read1_start), self.stride0),
                ),
                RankTwoTensorView::from_raw_parts(
                    self.stride1,
                    std::slice::from_raw_parts(ptr.add(read2_start), self.stride0),
                ),
                RankTwoTensorViewMut::from_raw_parts_mut(
                    self.stride1,
                    std::slice::from_raw_parts_mut(ptr.add(write_start), self.stride0),
                ),
            )
        }
    }

    pub fn downgrade(&self) -> RankThreeTensorView<'_, T> {
        RankThreeTensorView::from_raw_parts(self.stride0, self.stride1, self.data)
    }
}

pub struct RankTwoTensor<T> {
    stride0: usize,
    data: Box<[T]>,
}

impl<T: Default> RankTwoTensor<T> {
    pub fn new(shape0: usize, shape1: usize) -> Self {
        let len = shape0 * shape1;
        let mut data = Vec::new();
        data.resize_with(len, Default::default);

        Self {
            stride0: shape1,
            data: data.into(),
        }
    }

    pub fn from_flattened(data: Vec<T>, shape0: usize, shape1: usize) -> Self {
        let stride0 = shape1;
        assert_eq!(data.len(), shape0 * stride0);

        Self {
            stride0,
            data: data.into(),
        }
    }

    pub fn into_inner(self) -> Vec<T> {
        self.data.into()
    }
}

impl<T> RankTwoTensor<T> {
    pub fn as_view(&self) -> RankTwoTensorView<'_, T> {
        RankTwoTensorView {
            stride0: self.stride0,
            data: &self.data,
        }
    }

    pub fn as_view_mut(&mut self) -> RankTwoTensorViewMut<'_, T> {
        RankTwoTensorViewMut {
            stride0: self.stride0,
            data: &mut self.data,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RankTwoTensorView<'a, T> {
    stride0: usize,
    data: &'a [T],
}

impl<'a, T> RankTwoTensorView<'a, T> {
    pub fn from_flattened(shape0: u32, shape1: u32, data: &'a [T]) -> Self {
        assert_eq!((shape0 * shape1) as usize, data.len());
        Self::from_raw_parts(shape1 as usize, data)
    }

    fn from_raw_parts(stride0: usize, data: &'a [T]) -> Self {
        Self { stride0, data }
    }

    pub fn subview(self, index0: usize) -> &'a [T] {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        &self.data[start..end]
    }

    pub fn slice(self) -> &'a [T] {
        self.data
    }

    pub fn iter_subviews(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks_exact(self.stride0)
    }
}

impl<T: Default + Clone> RankTwoTensorView<'_, T> {
    pub fn to_transposed(&self) -> RankTwoTensor<T> {
        let data = &self.data;
        let len = data.len();
        let stride0 = self.stride0;
        let new_stride0 = len / stride0;

        // TODO: use MaybeUninit here instead
        let mut new_data = Vec::new();
        new_data.resize_with(len, Default::default);

        // Iterate over source rows (times stride) and target columns
        for (src_row, dest_col) in (0..).step_by(self.stride0).zip(0..new_stride0) {
            // Iterate over source columns destination rows (times new stride)
            for (src_index, dest_index) in
                (src_row..src_row + stride0).zip((dest_col..).step_by(new_stride0))
            {
                unsafe {
                    let source = data.get_unchecked(src_index).clone();
                    let dest = new_data.get_unchecked_mut(dest_index);
                    *dest = source;
                }
            }
        }

        RankTwoTensor {
            stride0: new_stride0,
            data: new_data.into(),
        }
    }
}

pub struct RankTwoTensorViewMut<'a, T> {
    stride0: usize,
    data: &'a mut [T],
}

impl<'a, T> RankTwoTensorViewMut<'a, T> {
    fn from_raw_parts_mut(stride0: usize, data: &'a mut [T]) -> Self {
        Self { stride0, data }
    }

    pub fn subview_mut(&mut self, index0: usize) -> &mut [T] {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        &mut self.data[start..end]
    }

    pub fn subview(&self, index0: usize) -> &[T] {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        &self.data[start..end]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data
    }

    pub fn iter_mut_subviews(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_exact_mut(self.stride0)
    }

    pub fn downgrade(&self) -> RankTwoTensorView<'_, T> {
        RankTwoTensorView::from_raw_parts(self.stride0, self.data)
    }
}
