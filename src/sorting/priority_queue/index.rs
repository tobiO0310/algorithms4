//! Holds an Index Priority Queue implementation.

use core::fmt;
use std::error::Error;

use super::{left_child, parent, right_child};

/// An index priority queue implemented with a binary heap,
/// ensuring log<sub>2</sub> *n* amortized time for [IndexPriorityQueue::insert]
/// and [IndexPriorityQueue::pop].
///
/// This priority queue will order such that [std::cmp::Ordering::Greater] elements are first.
/// (Max priority queue)
///
/// # Example
///
/// ```
/// let pq = 1;
/// ```
#[derive(Clone, Default, PartialEq, Eq)]
pub struct IndexPriorityQueue<K>
where
    K: Ord,
{
    heap: Vec<usize>,
    position: Vec<Option<usize>>,
    keys: Vec<Option<K>>,
}

/// The errors that can happen during execution
#[derive(fmt::Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)] // The fields are fine to not have docs, since they're descriptive enough as if
pub enum IndexPriorityQueueErrors {
    /// Given if the operation was given an index that is out of bounds
    OutOfBounds { len: usize, index: usize },
    /// Given if the operation was given an index that is not in the Priority Queue
    DoesNotContain { index: usize },
}

impl fmt::Display for IndexPriorityQueueErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { len, index } => f.write_str(
                format!("Index {} out of bounds with len {}", index, len)
                    .as_str(),
            ),
            Self::DoesNotContain { index } => f.write_str(
                format!("IndexPriorityQueue does not contain {}", index)
                    .as_str(),
            ),
        }
    }
}

impl Error for IndexPriorityQueueErrors {}

impl<K: Ord> IndexPriorityQueue<K> {
    /// Initializes an empty priority queue.
    ///
    /// Uses the default natural ordering of K and can hold at max `max_size` elements.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        let mut keys = Vec::with_capacity(max_size);
        keys.resize_with(max_size, Default::default); // to allow K not to be .clone
        let mut position = Vec::with_capacity(max_size);
        position.resize_with(max_size, Default::default);

        debug_assert_eq!(keys.len(), keys.capacity());
        debug_assert_eq!(position.len(), position.capacity());
        debug_assert_eq!(keys.len(), max_size);
        debug_assert_eq!(position.len(), max_size);

        Self {
            heap: Vec::with_capacity(max_size),
            position,
            keys,
        }
    }

    /// Returns the number of keys on this priority queue.
    #[must_use]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns true if this priority queue is empty.
    #[must_use]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds a new key to this priority queue.
    pub fn insert(
        &mut self,
        index: usize,
        key: K,
    ) -> Result<(), IndexPriorityQueueErrors> {
        self.validate_index(index)?;

        self.heap.push(index);
        self.keys[index] = Some(key);
        self.swim(self.heap.len() - 1);

        Ok(())
    }

    /// Peeks at the priority queue for the next key to be removed.
    #[must_use]
    pub fn peek_key(&self) -> Option<&K> {
        self.keys.get(*self.heap.first()?)?.as_ref()
    }

    /// Peeks at the priority queue for the next index to be removed.
    #[must_use]
    pub fn peek_index(&self) -> Option<usize> {
        self.heap.first().copied()
    }

    /// Checks if contains a value with specified index.
    #[must_use]
    pub fn contains(&self, index: usize) -> bool {
        self.validate_index(index).is_ok() && self.position[index].is_some()
    }

    /// Returns the key associated with index `i`.
    #[must_use]
    pub fn key_of(&self, i: usize) -> Option<&K> {
        self.validate_index(i).ok()?;
        self.keys.get(i)?.as_ref()
    }

    /// Change the key associated with index `i` to the specified value.
    pub fn change_key(
        &mut self,
        i: usize,
        key: K,
    ) -> Result<(), IndexPriorityQueueErrors> {
        self.validate_index(i)?;
        if !self.contains(i) {
            Err(IndexPriorityQueueErrors::DoesNotContain { index: i })
        } else {
            self.keys[i] = Some(key);
            self.swim(self.position[i].unwrap());
            self.sink(self.position[i].unwrap());

            Ok(())
        }
    }

    /// Removes a smallest key and returns the index associated with it on this priority queue.
    pub fn pop(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let n = self.heap.len() - 1;
        self.heap.swap(0, n);
        let key = self.heap.pop()?;
        self.sink(0);
        debug_assert!(self.is_heap_ordered(0));

        self.position[key] = None;
        self.keys[key] = None;

        Some(key)
    }

    fn swim(&mut self, mut pos: usize) {
        while pos > 0 && self.less(parent(pos), pos) {
            self.swap(parent(pos), pos);
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
            self.swap(pos, j);
            pos = j;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.position[self.heap[i]] = Some(i);
        self.position[self.heap[j]] = Some(j);
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.keys[self.heap[i]] < self.keys[self.heap[j]]
    }

    fn is_heap_ordered(&self, pos: usize) -> bool {
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

        self.is_heap_ordered(left) && self.is_heap_ordered(right)
    }

    fn validate_index(
        &self,
        index: usize,
    ) -> Result<(), IndexPriorityQueueErrors> {
        if index >= self.keys.len() {
            Err(IndexPriorityQueueErrors::OutOfBounds {
                len: self.keys.len(),
                index,
            })
        } else {
            Ok(())
        }
    }
}

/// The iterator of the consumed item
pub struct IntoIter<K: Ord> {
    item: IndexPriorityQueue<K>,
}

impl<K: fmt::Debug + Ord> fmt::Debug for IndexPriorityQueue<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.heap).finish()
    }
}

impl<K: Ord> Iterator for IntoIter<K> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.item.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.item.len(), Some(self.item.len()))
    }
}
impl<K: Ord> ExactSizeIterator for IntoIter<K> {
    fn len(&self) -> usize {
        self.item.len()
    }
}

impl<K: Ord> IntoIterator for IndexPriorityQueue<K> {
    type Item = usize;
    type IntoIter = IntoIter<K>;

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
        let mut pq = IndexPriorityQueue::new(2);
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());
        assert_eq!(pq.pop(), None);

        pq.insert(0, 1).unwrap();
        assert_eq!(pq.len(), 1);
        assert!(!pq.is_empty());
        assert_eq!(pq.peek_index(), Some(0));
        assert_eq!(pq.peek_key(), Some(&1));

        pq.insert(1, 100).unwrap();
        assert_eq!(pq.len(), 2);
        assert!(!pq.is_empty());
        assert_eq!(pq.peek_index(), Some(1));
        assert_eq!(pq.peek_key(), Some(&100));

        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.len(), 1);
        assert_eq!(pq.pop(), Some(0));
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
    }

    fn get_biggest<T: Ord>(arr: &[T]) -> (usize, &T) {
        arr.iter().enumerate().max_by(|a, b| a.1.cmp(b.1)).unwrap()
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Node<T>(usize, T);

    impl<T: PartialOrd> PartialOrd for Node<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.1.partial_cmp(&other.1)
        }
    }

    impl<T: Ord> Ord for Node<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.1.cmp(&other.1)
        }
    }

    #[test]
    fn test_big() {
        let mut rand = rand::rng();
        let mut vec = Vec::with_capacity(1000);
        let mut pq = IndexPriorityQueue::new(1000);

        assert_eq!(pq.pop(), None);

        for (i, item) in vec.iter_mut().enumerate() {
            *item = Node(i, rand.random_range(-100..=100));
            pq.insert(i, item.1).unwrap();
        }

        while !pq.is_empty() {
            let (i, &biggest) = get_biggest(&vec);
            assert_eq!(pq.peek_key(), Some(&biggest.1));
            assert_eq!(pq.pop(), Some(biggest.0));

            vec.swap_remove(i);
        }

        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.pop(), None);
    }
}
