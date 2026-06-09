use crate::{
    SymbolTable,
    collections::queue::{IntoIter, Queue},
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
pub struct SequentialSearch<K, V> {
    queue: Queue<Node<K, V>>,
}

impl<K, V> SequentialSearch<K, V> {
    /// Instansiate a new [SequentialSearch] object
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
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

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a> {
        Box::new(
            self.queue
                .iter()
                .map(|n| (&n.key, n.value.as_ref().unwrap())),
        )
    }
}

impl<K: Eq, V> IntoIterator for SequentialSearch<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.queue
            .into_iter()
            .map(|Node { key, value }| (key, value.unwrap()))
            .collect::<Queue<_>>()
            .into_iter()
    }
}
impl<'a, K: Ord, V> IntoIterator for &'a SequentialSearch<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
