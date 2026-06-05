use core::fmt;
use std::{cmp, fmt::Debug};

mod index;
pub use index::*;

// Returns index of parent
fn parent(i: usize) -> usize {
    (i - 1) / 2
}

// Returns index of left child
fn left_child(i: usize) -> usize {
    2 * i + 1
}

// Returns index of right child
fn right_child(i: usize) -> usize {
    2 * i + 2
}

/// A priority queue implemented with a binary heap,
/// ensuring log<sub>2</sub> *n* amortized time for [PriorityQueue::insert]
/// and [PriorityQueue::delete].
///
/// This priority queue will order such that [Ordering::Greater] elements are first.
/// (Max priority queue)
///
/// # Example
///
/// ```
/// # use algorithms4::PriorityQueue;
/// let mut pq = PriorityQueue::new();
/// pq.insert(10);
/// pq.insert(4);
/// pq.insert(100);
/// assert_eq!(pq.pop(), Some(100));
/// ```
#[derive(Clone, Default)]
pub struct PriorityQueue<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    heap: Vec<K>,
    comparator: F,
}

impl<K: Ord> PriorityQueue<K, fn(&K, &K) -> cmp::Ordering> {
    /// Initializes an empty priority queue.
    ///
    /// Uses the default natural ordering of K.
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            comparator: |v1, v2| v1.cmp(v2),
        }
    }
}

impl<K, F> PriorityQueue<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    /// Initializes an empty priority queue.
    ///
    /// Uses the given comparator function to compare any two values.
    pub fn with_comparator(comparator: F) -> Self {
        Self {
            heap: Vec::new(),
            comparator,
        }
    }

    /// Returns the number of keys on this priority queue.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns true if this priority queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    /// Adds a new key to this priority queue.
    pub fn insert(&mut self, key: K) {
        self.heap.push(key);
        self.swim(self.heap.len() - 1);
    }

    /// Peeks at the priority queue for the next element to be removed.
    pub fn peek(&self) -> Option<&K> {
        self.heap.first()
    }

    /// Removes and returns a smallest key on this priority queue.
    pub fn pop(&mut self) -> Option<K> {
        if self.is_empty() {
            return None;
        }
        let n = self.heap.len() - 1;
        self.heap.swap(0, n);
        let key = self.heap.pop()?;
        self.sink(0);
        debug_assert!(self.is_heap_orderd(0));

        Some(key)
    }

    fn swim(&mut self, mut pos: usize) {
        while pos > 0 && self.less(parent(pos), pos) {
            self.heap.swap(parent(pos), pos);
            pos = parent(pos);
        }
    }

    fn sink(&mut self, mut pos: usize) {
        while left_child(pos) < self.heap.len() {
            let mut j = left_child(pos);
            if j < self.heap.len() - 1 && self.less(j, j + 1) {
                j += 1; // right child
            }
            if !self.less(pos, j) {
                break;
            }
            self.heap.swap(pos, j);
            pos = j;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        (self.comparator)(&self.heap[i], &self.heap[j]).is_lt()
    }

    fn is_heap_orderd(&self, pos: usize) -> bool {
        if pos > self.heap.len() {
            return true;
        }
        let left = left_child(pos);
        let right = right_child(pos);
        if left < self.heap.len() && self.less(pos, left)
            || right < self.heap.len() && self.less(pos, right)
        {
            return false;
        }

        self.is_heap_orderd(left) && self.is_heap_orderd(right)
    }
}

/// The iterator of the consumed item
pub struct IntoIter<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    item: PriorityQueue<K, F>,
}

impl<K, F> Extend<K> for PriorityQueue<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}
impl<K: Ord> FromIterator<K> for PriorityQueue<K, fn(&K, &K) -> cmp::Ordering> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}
impl<K: Debug, F> Debug for PriorityQueue<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.heap).finish()
    }
}

impl<K, F> Iterator for IntoIter<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.item.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.item.len(), Some(self.item.len()))
    }
}
impl<K, F> ExactSizeIterator for IntoIter<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    fn len(&self) -> usize {
        self.item.len()
    }
}

impl<K, F> IntoIterator for PriorityQueue<K, F>
where
    K: Ord,
    F: Fn(&K, &K) -> cmp::Ordering,
{
    type Item = K;
    type IntoIter = IntoIter<K, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { item: self }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    #[test]
    fn test_simple() {
        let mut pq = PriorityQueue::new();
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());
        assert_eq!(pq.pop(), None);

        pq.insert(1);
        assert_eq!(pq.len(), 1);
        assert!(!pq.is_empty());
        assert_eq!(pq.peek(), Some(&1));

        pq.insert(100);
        assert_eq!(pq.len(), 2);
        assert!(!pq.is_empty());
        assert_eq!(pq.peek(), Some(&100));

        assert_eq!(pq.pop(), Some(100));
        assert_eq!(pq.len(), 1);
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
    }

    fn get_biggest<T: Ord>(arr: &[T]) -> (usize, &T) {
        arr.iter().enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap()
    }

    #[test]
    fn test_big() {
        let mut rand = rand::rng();
        let mut vec = Vec::with_capacity(1000);
        let mut pq = PriorityQueue::new();

        assert_eq!(pq.pop(), None);

        for item in vec.iter_mut() {
            *item = rand.random_range(-100..=100);
            pq.insert(*item);
        }

        while !pq.is_empty() {
            let (i, &biggest) = get_biggest(&vec);
            assert_eq!(pq.pop(), Some(biggest));

            vec.swap_remove(i);
        }

        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
    }
}
