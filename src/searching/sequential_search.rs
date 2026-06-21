use std::ops::Index;

use crate::{
    SymbolTable,
    collections::queue::{self, IntoIter, Queue},
    searching::SearchingNode,
};

/// A [SymbolTable] implemented as an unordered linked list.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SequentialSearch<K: Eq, V> {
    queue: Queue<SearchingNode<K, V>>,
}

/// An iterator for &[SequentialSearch].
#[must_use = "iterators are lazy and do nothing on their own"]
pub struct Iter<'a, K, V> {
    queue: queue::Iter<'a, SearchingNode<K, V>>,
}

impl<K: Eq, V> SequentialSearch<K, V> {
    /// Instantiate a new [SequentialSearch] object
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
        }
    }

    /// Returns an iterator over all the entries of this [SequentialSearch].
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
    {
        Iter {
            queue: self.queue.iter(),
        }
    }
}

impl<K: Eq, V> SymbolTable<K, V> for SequentialSearch<K, V> {
    fn put(&mut self, key: K, value: V) {
        for n in self.queue.iter_mut() {
            if n.0 == key {
                n.1 = Some(value);
                return;
            }
        }
        self.queue.enqueue(SearchingNode(key, Some(value)));
    }

    fn get(&self, k: &K) -> Option<&V> {
        for SearchingNode(key, value) in self.queue.iter() {
            if key == k {
                return value.as_ref(); // is always some,
            }
        }

        None
    }

    fn delete(&mut self, key: &K)
    where
        K: Clone,
    {
        let _ = self.queue.delete_inner(&SearchingNode(key.clone(), None));
    }

    fn clear(&mut self) {
        self.queue.clear();
    }

    fn size(&self) -> usize {
        self.queue.len()
    }
}

impl<K: Eq, V> Index<&K> for SequentialSearch<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found in SymbolTable")
    }
}
impl<K: Eq, V> IntoIterator for SequentialSearch<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.queue
            .into_iter()
            .map(|SearchingNode(key, value)| (key, value.unwrap()))
            .collect::<Queue<_>>()
            .into_iter()
    }
}
impl<'a, K: Eq, V> IntoIterator for &'a SequentialSearch<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, K: Eq, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.next()?;
        Some((&node.0, node.1.as_ref().unwrap()))
    }
}
