//!
//! Limited Queue
//!

use std::collections::VecDeque;

pub struct LimiQ<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> LimiQ<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(item);
    }

    // TODO impl deref to vec to reduce boilerplate crap
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        self.buffer.get_mut(index).unwrap()
    }

    pub fn get(&self, index: usize) -> &T {
        self.buffer.get(index).unwrap()
    }

    pub fn try_get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn get_last(&self) -> Option<&T> {
        self.buffer.back()
    }

    pub fn get_last_mut(&mut self) -> Option<&mut T> {
        self.buffer.back_mut()
    }

    /// get the ith item, or the last item if i is out of bounds
    pub fn get_or_last(&self, i: usize) -> &T {
        &self.buffer[std::cmp::min(i, self.buffer.len() - 1)]
    }
}
