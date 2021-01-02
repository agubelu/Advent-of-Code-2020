use std::ops::{Index, IndexMut};

pub type MatPosition = (usize, usize);
pub struct MatNum<T> {
    width: usize,
    height: usize,
    content: Vec<T>,
}

impl<T: Copy> MatNum<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let cont = vec![default; width * height];
        MatNum::<T>{width, height, content: cont}
    }

    pub fn from_content(width: usize, height: usize, content: Vec<T>) -> Self {
        MatNum::<T>{width, height, content}
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        // No bounds check because yolo
        self.content[y * self.height + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.content[y * self.height + x] = val
    }

    pub fn flat(&self) -> &[T] {
        &self.content
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Copy> Clone for MatNum<T> {
    fn clone(&self) -> Self {
        let cont = self.content.clone();
        MatNum::from_content(self.width(), self.height(), cont)
    }
}

impl<T: Copy> Index<MatPosition> for MatNum<T> {
    type Output = T;
    
    fn index(&self, pos: MatPosition) -> &Self::Output {
        &self.content[pos.1 * self.height + pos.0]
    }
}

impl<T: Copy> IndexMut<MatPosition> for MatNum<T> {
    fn index_mut(&mut self, pos: MatPosition) -> &mut Self::Output {
        &mut self.content[pos.1 * self.height + pos.0]
    }
}
