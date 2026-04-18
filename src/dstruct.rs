use std::slice::Iter;

pub struct Matrix<T> {
    width: usize,
    height: usize,
    arr: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, val: T) -> Self {
        Matrix {
            width,
            height,
            arr: vec![val; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.check_bounds(x, y);
        self.arr[x + y*self.width] = val
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        self.check_bounds(x, y);
        &self.arr[x + y*self.width]
    }

    pub fn iter(&self) -> MatrixIter<'_, T> {
        MatrixIter::new(self)
    }

    fn check_bounds(&self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("Point {x}, {y} out of range for matrix with size {}, {}", self.width, self.height)
        }
    }
}

pub struct MatrixIter<'a, T> {
    count: usize,
    width: usize,
    height: usize,
    arr: &'a Vec<T>,
}

impl<'a, T> MatrixIter<'a, T> {
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        MatrixIter {
            count: 0,
            width: matrix.width,
            height: matrix.height,
            arr: &matrix.arr,
        }
    }
}

impl<'a, T: Clone> Iterator for MatrixIter<'a, T> {
    type Item = Iter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.height {
            let start = self.count * self.width;
            let end = start + self.width;
            self.count += 1;
            Some(self.arr[start..end].iter())
        } else {
            None
        }
    }
}
