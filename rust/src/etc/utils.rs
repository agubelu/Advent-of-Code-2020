use std::ops::{Index, IndexMut};

pub struct Mat<T> {
    width: usize,
    height: usize,
    content: Vec<T>,
}

impl<T: Clone> Mat<T> {
    fn new(width: usize, height: usize, default_val: Option<T>) -> Self {
        let cont = match default_val {
            None => Vec::with_capacity(width * height),
            Some(val) => vec![val; width * height],
        };
        return Mat::<T>{width, height, content: cont};
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        return self.content.get(y * self.height + x);
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        self.content[y * self.height + x] = val;
    }
}

impl<T: Clone> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        return self.get(pos.0, pos.1).unwrap();
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        return self.content.get_mut(pos.1 * self.height + pos.0).unwrap();
    }
}
