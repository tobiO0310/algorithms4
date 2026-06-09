use std::ops::Index;

use crate::{
    SymbolTable,
    collections::queue::{self, IntoIter, Queue},
};

#[derive(Debug, Default, Clone, Copy)]
struct Node<K, V> {
    key: K,
    /// This should always be Some(V) for any real value.
    ///
    /// Option<V> instead of V is only used, so comparison independant of value can be done.
    value: Option<V>,
}
impl<K: PartialEq, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<K: Eq, V> Eq for Node<K, V> {}
impl<K: PartialOrd, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
impl<K: Ord, V> Ord for Node<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// A [SymbolTable] implemented as an unordered linked list.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SequentialSearch<K: Eq, V> {
    queue: Queue<Node<K, V>>,
}

/// An iterator for &[SequentialSearch].
pub struct Iter<'a, K, V> {
    queue: queue::Iter<'a, Node<K, V>>,
}

impl<K: Eq, V> SequentialSearch<K, V> {
    /// Instansiate a new [SequentialSearch] object
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
            if n.key == key {
                n.value = Some(value);
                return;
            }
        }
        self.queue.enqueue(Node {
            key,
            value: Some(value),
        });
    }

    fn get(&self, k: &K) -> Option<&V> {
        for Node { key, value } in self.queue.iter() {
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
        let _ = self.queue.delete_inner(&Node {
            key: key.clone(),
            value: None,
        });
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
            .map(|Node { key, value }| (key, value.unwrap()))
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
        Some((&node.key, node.value.as_ref().unwrap()))
    }
}
