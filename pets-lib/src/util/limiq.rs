//!
//! Limited Queue stuff
//!

use crate::prelude::*;
use std::collections::VecDeque;

/// "Limited Queue"
/// Circular buffer that pops from the back if over capacity
/// Technically, yes, it's a deque. However, this is you: 🤓
#[derive(Deref)]
pub struct LimiQ<T> {
    #[target]
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
    /// No longer O(n) time because it uses VecDeque! (yippee!)
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

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl<T> std::ops::Index<usize> for LimiQ<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(index)
    }
}
