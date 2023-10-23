//!
//! Limited Queue stuff
//!

use std::collections::VecDeque;
use std::ops::{Deref, Index};

/// "Limited Queue"
/// Circular buffer that pops from the back if over capacity
/// Technically, yes, it's a deque. However, this is you: 🤓
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

    /// Push to front, but also pop from back if over capacity
    /// WARNING: O(n) time due to shifting the whole queue
    pub fn push(&mut self, item: T) {
        self.buffer.push_front(item);

        if self.buffer.len() >= self.capacity {
            self.buffer.pop_back();
        }
    }

    /// get the ith item, or the last item if i is out of bounds
    pub fn get_or_last(&self, i: usize) -> &T {
        &self.buffer[std::cmp::min(i, self.buffer.len() - 1)]
    }
}

impl<T> Index<usize> for LimiQ<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(index)
    }
}

impl<T> Deref for LimiQ<T> {
    type Target = VecDeque<T>;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
