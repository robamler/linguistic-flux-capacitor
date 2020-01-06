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

    pub fn as_view_mut(&mut self) -> RankThreeTensorViewMut<T> {
        RankThreeTensorViewMut {
            stride0: self.stride0,
            stride1: self.stride1,
            data: &mut self.data,
        }
    }
}

pub struct RankThreeTensorViewMut<'a, T> {
    stride0: usize,
    stride1: usize,
    data: &'a mut [T],
}

impl<'a, T> RankThreeTensorViewMut<'a, T> {
    pub fn subview_mut(&mut self, index0: usize) -> RankTwoTensorViewMut<T> {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        RankTwoTensorViewMut {
            stride0: self.stride1,
            data: &mut self.data[start..end],
        }
    }

    pub fn subviews_rrw(
        &mut self,
        read1_index: usize,
        read2_index: usize,
        write_index: usize,
    ) -> (
        RankTwoTensorView<T>,
        RankTwoTensorView<T>,
        RankTwoTensorViewMut<T>,
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
            // This now has three mutable references pointing at the same
            // memory. `slice`, the rvalue ret.0, and the rvalue ret.1.
            // `slice` is never used after `let ptr = ...`, and so one can
            // treat it as "dead", and therefore, you only have two real
            // mutable slices.
            (
                RankTwoTensorView::from_raw_parts(
                    self.stride1,
                    std::slice::from_raw_parts(ptr.add(read1_start), self.stride0),
                ),
                RankTwoTensorView::from_raw_parts(
                    self.stride1,
                    std::slice::from_raw_parts(ptr.add(read1_start), self.stride0),
                ),
                RankTwoTensorViewMut::from_raw_parts_mut(
                    self.stride1,
                    std::slice::from_raw_parts_mut(ptr.add(read1_start), self.stride0),
                ),
            )
        }
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

    pub fn into_inner(self) -> Vec<T> {
        self.data.into()
    }
}

impl<T> RankTwoTensor<T> {
    pub fn as_view_mut(&mut self) -> RankTwoTensorViewMut<T> {
        RankTwoTensorViewMut {
            stride0: self.stride0,
            data: &mut self.data,
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

    pub fn downgrade(&self) -> RankTwoTensorView<T> {
        RankTwoTensorView::from_raw_parts(self.stride0, self.data)
    }
}

pub struct RankTwoTensorView<'a, T> {
    stride0: usize,
    data: &'a [T],
}

impl<'a, T> RankTwoTensorView<'a, T> {
    fn from_raw_parts(stride0: usize, data: &'a [T]) -> Self {
        Self { stride0, data }
    }

    pub fn subview(&self, index0: usize) -> &[T] {
        let start = index0 * self.stride0;
        let end = start + self.stride0;
        &self.data[start..end]
    }

    pub fn as_slice(&self) -> &[T] {
        self.data
    }
}
