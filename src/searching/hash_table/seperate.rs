//! This module holds a [SymbolTable] implemented as a Seperate Chaining Hash Table.

use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::Index,
};

use crate::{
    collections::queue::{self, Queue},
    searching::{SequentialSearch, SymbolTable},
};

const INITIAL_CAPACITY: usize = 4;

/// An unordered symbol table implemented as seperate-chaining hash table.
///
/// # Examples
///
/// ```
/// # use algorithms4::{SeperateChainingHashTable, SymbolTable};
/// let mut bst = SeperateChainingHashTable::new();
///
/// bst.put("Test1", 1);
/// bst.put("Test2", 2);
/// bst.put("can", -10);
/// bst.put("corn", 5);
///
/// assert_eq!(bst.get(&"corn"), Some(&5));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SeperateChainingHashTable<K: Eq, V> {
    st: Vec<SequentialSearch<K, V>>,
    amount: usize,
}

/// A shorthand for simpler typing.
pub type HashTable<K, V> = SeperateChainingHashTable<K, V>;

impl<K: Eq + Hash, V> SeperateChainingHashTable<K, V> {
    /// Initializes an empty symbol table.
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity_unchecked(INITIAL_CAPACITY)
    }

    /// Initializes an empty symbol table with an amount of chains.
    ///
    /// `amount` will be rounded to nearest power of 2.
    ///
    /// # Panics
    ///
    /// Panics if `amount` is 0
    #[inline]
    pub fn with_capacity(mut amount: usize) -> Self {
        let prev_pow2 = 2usize.pow(amount.ilog2());
        let next_pow2 = amount.checked_next_power_of_two();
        amount = if let Some(next_pow2) = next_pow2 {
            if amount.abs_diff(prev_pow2) <= amount.abs_diff(next_pow2) {
                prev_pow2
            } else {
                next_pow2
            }
        } else {
            prev_pow2
        };

        // SAFETY: amount is a power of two
        Self::with_capacity_unchecked(amount)
    }

    /// Initializes an empty symbol table with an amount of chains.
    ///
    /// # Panics
    ///
    /// Panics if `amount` is 0
    ///
    /// # Safety
    ///
    /// `amount` must be given in a power of 2, else the hashing breaks down
    pub fn with_capacity_unchecked(amount: usize) -> Self {
        debug_assert!(amount.is_power_of_two());
        let mut st = Vec::with_capacity(amount);
        st.resize_with(amount, || SequentialSearch::new());
        Self { st, amount: 0 }
    }

    fn resize(&mut self, amount: usize) {
        let mut temp = SeperateChainingHashTable::with_capacity(amount);
        for st in self.st.split_off(0) {
            for (key, value) in st {
                temp.put(key, value);
            }
        }
        self.st = temp.st;
    }

    /// A simple hash function, as given in the book :)
    ///
    /// This assumes st.len() is a power of two (for an optimized modulus)
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let mut h = hasher.finish() as usize;
        h ^= (h >> 20) ^ (h >> 12) ^ (h >> 7) ^ (h >> 4);
        h & (self.st.len() - 1)
    }

    /// Returns an iterator over all the entries of this [SeperateChainingHashTable].
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
    {
        Iter {
            table: self,
            current: (0, Box::new(self.st[0].iter())),
        }
    }
}

/// An iterator over a [SeperateChainingHashTable].
pub struct Iter<'a, K: Eq + Hash, V> {
    table: &'a SeperateChainingHashTable<K, V>,
    current: (usize, Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a>),
}

impl<K: Eq + Hash, V> SymbolTable<K, V> for SeperateChainingHashTable<K, V> {
    fn put(&mut self, key: K, value: V) {
        if self.amount >= 10 * self.st.len() {
            self.resize(self.st.len() * 2);
        }

        let hash = self.hash(&key);
        self.st[hash].put(key, value);
        self.amount += 1;
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.st[self.hash(key)].get(key)
    }

    fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone,
    {
        let hash = self.hash(key);
        self.st[hash].delete(key);
        self.amount -= 1;

        // halve table size if average length of list <= 2
        let chains = self.st.len();
        if chains > INITIAL_CAPACITY && self.amount <= 2 * chains {
            self.resize(chains / 2)
        }
    }

    fn clear(&mut self) {
        for st in &mut self.st {
            st.clear();
        }
    }

    fn size(&self) -> usize {
        self.st.iter().map(|v| v.size()).sum()
    }
}

impl<K: Eq + Hash, V> Index<&K> for SeperateChainingHashTable<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found in SymbolTable")
    }
}
impl<'a, K: Eq + Hash, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current.0 != self.table.st.len() {
            let (st, num) = &mut self.current;
            if let Some((key, value)) = num.next() {
                return Some((key, value));
            } else {
                // find the next available item
                *st += 1;
                let max = self.table.st.len() - 1;
                while self.table.st[(*st).max(max)].is_empty() {
                    *st += 1;
                    if *st >= self.table.st.len() {
                        break;
                    }
                }
                let next_iter = self.table.st[*st].iter();
                self.current.1 = Box::new(next_iter);
            }
        }
        None
    }
}
impl<K: Eq, V> IntoIterator for SeperateChainingHashTable<K, V> {
    type Item = (K, V);
    type IntoIter = queue::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut queue = Queue::new();
        for st in self.st {
            queue.extend(st)
        }

        queue.into_iter()
    }
}
impl<'a, K: Eq + Hash, V> IntoIterator for &'a SeperateChainingHashTable<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

super::test_hash_table!(SeperateChainingHashTable);
