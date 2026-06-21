//! This module contains the code to create a simple bag

use std::collections::{LinkedList, linked_list};

/// A bag is a collection without the ability to delete from it.
/// Its purpose is to allows collection and iteration.
/// The order is unspecified and should not matter.
///
/// All methods are `O(1)` expect [Bag::iter] which is `O(n)`
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bag<T> {
    data: LinkedList<T>,
}

impl<T> Bag<T> {
    /// Creates a new Bag
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: LinkedList::new(),
        }
    }
    /// Inserts an item into the bag
    pub fn insert(&mut self, data: T) {
        self.data.push_back(data);
    }
    /// Checks if the bag is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Returns the amount of elements in the bag
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the iterator for the bag
    pub fn iter(&self) -> linked_list::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> Extend<T> for Bag<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}

impl<T> FromIterator<T> for Bag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = linked_list::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_bag() {
        let mut bag: Bag<i32> = Bag::new();

        assert!(bag.is_empty());
        assert_eq!(bag.len(), 0);

        bag.insert(1);

        assert_eq!(bag.len(), 1);
        assert!(!bag.is_empty());

        bag.insert(2);
        bag.insert(3);
        bag.insert(4);

        // test iteration
        let mut vec = vec![1, 2, 3, 4];
        for i in bag.iter() {
            let size = vec.len();
            vec.retain(|v| v != i);
            assert_eq!(vec.len(), size - 1); // assert ONE item was deleted
        }
        assert!(vec.is_empty());
    }
}
