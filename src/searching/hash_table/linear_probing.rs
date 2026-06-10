//! This module holds a [SymbolTable] implemented as a Linear Probing Hash Table.

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    iter::successors,
    ops::Index,
};

use crate::{SymbolTable, searching::SearchingNode};

const INITIAL_CAPACITY: usize = 4;

/// An unordered symbol table implemented as linear-probing hash table.
///
/// # Examples
///
/// ```
/// # use algorithms4::{LinearProbingHashTable, SymbolTable};
/// let mut bst = LinearProbingHashTable::new();
///
/// bst.put("Test1", 1);
/// bst.put("Test2", 2);
/// bst.put("can", -10);
/// bst.put("corn", 5);
///
/// assert_eq!(bst.get(&"corn"), Some(&5));
/// ```
#[derive(Debug, Default)]
pub struct LinearProbingHashTable<K, V> {
    items: Vec<Option<SearchingNode<K, V>>>,
    amount: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Checks<'a, K, V> {
    HashTableTooSmall(usize, usize),
    /// key, expected
    CouldNotGetKey(&'a K, Option<&'a V>), // "get[" + keys[i] + "] = " + get(keys[i]) + "; vals[i] = " + vals[i]
}

/// An iterator for a [LinearProbingHashTable].
pub struct Iter<'a, K, V> {
    items: &'a Vec<Option<SearchingNode<K, V>>>,
    current: usize,
}

impl<K: Hash + Eq, V> LinearProbingHashTable<K, V> {
    /// Initializes an empty symbol table.
    pub fn new() -> Self {
        Self::with_capacity_unchecked(INITIAL_CAPACITY)
    }

    /// Initializes an empty symbol table with an amount of preallocated spaces.
    ///
    /// `amount` will be rounded to nearest power of 2.
    ///
    /// # Panics
    ///
    /// Panics if `amount` is 0
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

    /// Initializes an empty symbol table with an amount of preallocated spaces.
    ///
    /// # Panics
    ///
    /// Panics if `amount` is 0
    ///
    /// # Safety
    ///
    /// `amount` must be given in a power of 2, else the hashing breaks down
    pub fn with_capacity_unchecked(amount: usize) -> Self {
        let mut items = Vec::with_capacity(amount);
        items.resize_with(amount, || None);
        Self { items, amount: 0 }
    }

    /// Returns an iterator over all entries in this [LinearProbingHashTable].
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            items: &self.items,
            current: 0,
        }
    }

    fn resize(&mut self, amount: usize) {
        let mut temp = LinearProbingHashTable::with_capacity(amount);
        for SearchingNode(key, value) in
            self.items.split_off(0).into_iter().flatten()
        {
            temp.put(key, value.unwrap());
        }
        self.items = temp.items;
    }

    /// A simple hash function, as given in the book :)
    ///
    /// This assumes st.len() is a power of two (for an optimized modulus)
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let mut h = hasher.finish() as usize;
        h ^= (h >> 20) ^ (h >> 12) ^ (h >> 7) ^ (h >> 4);
        h & (self.items.len() - 1)
    }

    fn hash_iter(start: usize, max: usize) -> impl Iterator<Item = usize> {
        successors(Some(start), move |&a| Some((a + 1) % max))
    }

    // integrity check - don't check after each put() because
    // integrity not maintained during a call to delete()
    fn check<'a>(&'a self) -> Result<(), Checks<'a, K, V>> {
        // check that hash table is at most 50% full
        if self.items.len() < self.amount * 2 {
            Err(Checks::HashTableTooSmall(self.amount, self.items.len()))
        } else {
            // check that each key in table can be found by get()
            for n in self.items.iter().flatten() {
                if self.get(&n.0).is_none() {
                    return Err(Checks::CouldNotGetKey(&n.0, n.1.as_ref()));
                }
            }

            Ok(())
        }
    }
}

impl<K: Hash + Eq, V> SymbolTable<K, V> for LinearProbingHashTable<K, V> {
    fn put(&mut self, key: K, value: V) {
        if self.amount >= self.items.len() / 2 {
            self.resize(self.items.len() * 2);
        }

        let mut i = self.hash(&key);
        while let Some(n) = &mut self.items[i] {
            if n.0 == key {
                n.1 = Some(value);
                return;
            }
            i += 1;
        }
        self.items[i] = Some(SearchingNode(key, Some(value)));
        self.amount += 1;
    }

    fn get(&self, key: &K) -> Option<&V> {
        let start = self.hash(key);
        let max = self.items.len();
        for i in Self::hash_iter(start, max) {
            if let Some(v) = &self.items[i] {
                if &v.0 == key {
                    return v.1.as_ref();
                }
            } else {
                break;
            }
        }
        None
    }

    fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone,
    {
        if self.amount == 0 || !self.contains(key) {
            return;
        }
        let m = self.items.len();

        // find position i of key
        let mut i = self.hash(key);
        while key != &self.items[i].as_ref().unwrap().0 {
            i = (i + 1) % m;
        }

        // delete item
        self.items[i] = None;
        self.amount -= 1;

        // rehash all keys in same cluster
        i = (i + 1) % m;
        while self.items[i].is_some() {
            let item = self.items[i].take().unwrap();
            self.amount -= 1;
            self.put(item.0, item.1.unwrap());
            i = (i + 1) % m;
        }

        // halves size of array if it's 12.5% full or less
        if self.amount > 0 && self.amount <= m / 8 {
            self.resize(m / 2);
        }

        debug_assert!(self.check().is_ok())
    }

    fn clear(&mut self) {
        self.items.clear();
        self.items.resize_with(INITIAL_CAPACITY, || None);
    }

    fn size(&self) -> usize {
        self.amount
    }
}

impl<K: Eq + Hash, V> Index<&K> for LinearProbingHashTable<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found in SymbolTable")
    }
}
impl<'a, K: Eq + Hash, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.items.len();
        while self.current != max {
            // find the next item, or return None if goes over
            match self.items.get(self.current) {
                Some(Some(v)) => {
                    self.current += 1;
                    return Some((&v.0, v.1.as_ref().unwrap()));
                }
                // Some(Some(v)) => v is set in original items vec
                Some(None) => self.current += 1,
                // Some(None) => v is not set in original items vec
                None => return None,
                // None => i is out of bounds
            }
        }
        None
    }
}
impl<K, V> IntoIterator for LinearProbingHashTable<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .into_iter()
            .flatten()
            .map(|SearchingNode(k, v)| (k, v.unwrap()))
            .collect::<Vec<_>>() // simply iterator step
            .into_iter()
    }
}
impl<'a, K: Eq + Hash, V> IntoIterator for &'a LinearProbingHashTable<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

super::test_hash_table!(LinearProbingHashTable);
