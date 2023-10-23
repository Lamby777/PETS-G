//!
//! Limited Queue
//!

pub struct LimiQ<T> {
    buffer: Vec<T>,
    capacity: usize,
}

impl<T> LimiQ<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.remove(0);
        }

        self.buffer.push(item);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn get_last(&self) -> Option<&T> {
        self.buffer.last()
    }

    pub fn get_last_mut(&self) -> Option<&mut T> {
        self.buffer.last_mut()
    }

    pub fn get_first_or(&self, deft: T) -> &T {
        if self.buffer.is_empty() {
            &deft
        } else {
            &self.buffer[0]
        }
    }

    /// get the ith item, or the last item if i is out of bounds
    pub fn get_or_last(&self, i: usize) -> &T {
        &self.buffer[std::cmp::min(i, self.buffer.len() - 1)]
    }
}
