//! This module holds all the different symbol tables described in chapter 3 of the book.
//!
//! ## Examples
//! Create a Balanced Search tree and search
//! ```
//! # use algorithms4::{RedBlackBST, SymbolTable};
//! let mut fruit_list = RedBlackBST::new();
//!
//! fruit_list.put("drupes", vec!["apple", "pear", "peach", "apricot"]);
//! fruit_list.put("citrus", vec!["lemon", "lime", "mandarin", "orange"]);
//! fruit_list.put("berries", vec!["banana", "blueberry", "raspberry", "grape"]);
//! fruit_list.put("melons", vec!["cantaloupe", "mango", "watermelon", "papaya"]);
//!
//! # assert_eq!(fruit_list.get(&"citrus"), Some(&vec!["lemon", "lime", "mandarin", "orange"]));
//! ```

mod bst;
mod sequential_search;
pub use bst::*;
pub use sequential_search::*;

/// A symbol table (ST) allows for inserting keys and their associated values,
/// and then later search for them efficiently.
///
/// See implementations for further examples.
pub trait SymbolTable<K, V>: IntoIterator<Item = (K, V)> {
    /// Adds a key and value to the [SymbolTable] if it does not already exist,
    /// else overrides the value associated with the key.
    fn put(&mut self, key: K, value: V);
    /// Gets the value associated with the key if it exists, else returns [None].
    #[must_use]
    fn get(&self, key: &K) -> Option<&V>;

    /// Removes a key, including the value, if it exists in the [SymbolTable].
    fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone;
    /// Clears the [SymbolTable] of all entries.
    fn clear(&mut self);

    /// Indicates whether the [SymbolTable] has the given key.
    #[must_use]
    #[inline(always)]
    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    /// Indicates whether the [SymbolTable] is empty or has at least one item
    #[must_use]
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
    /// Returns the amount of keys in the [SymbolTable]
    #[must_use]
    fn size(&self) -> usize;

    /// Returns an iterator over all the entries of this [SymbolTable]
    ///
    /// It is not guranteed to be ordered.
    #[must_use]
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a>;
}

/// An ordered symbol table extends a standard [SymbolTable].
/// It allows ranking keys in comparision to each other, finding a key of `x` rank,
/// alongside fidning the minimum and maximum keys inside the symbol table.
///
/// See implementations for examples.
pub trait OrderedSymbolTable<K: Ord, V>: SymbolTable<K, V> {
    /// Returns the smallest key according to its natural ordering
    #[must_use]
    fn min(&self) -> Option<&K>;
    /// Returns the biggest key according to its natural ordering
    #[must_use]
    fn max(&self) -> Option<&K>;

    /// Returns the biggest key <= `key` according to its natural ordering
    #[must_use]
    fn floor(&self, key: &K) -> Option<&K>;
    /// Returns the smallest key >= `key` according to its natural ordering
    #[must_use]
    fn ceiling(&self, key: &K) -> Option<&K>;

    /// Returns the number of keys less than `key`
    #[must_use]
    fn rank(&self, key: &K) -> usize;
    /// Returns the key that satisfies [self.rank(key)](OrderedSymbolTable::rank) == `rank`.
    #[must_use]
    fn select(&self, rank: usize) -> Option<&K>;
    /// Returns the number of keys between `lo` and `hi`
    #[must_use]
    fn size_betwen(&self, lo: &K, hi: &K) -> usize {
        if hi < lo {
            0
        } else if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            self.rank(hi) - self.rank(lo)
        }
    }

    /// Deletes the smallest key
    ///
    /// If the [SymbolTable] is empty, this function does nothing.
    #[inline]
    fn delete_min(&mut self)
    where
        K: Clone,
        V: Clone,
    {
        let val = self.min();
        if let Some(val) = val {
            self.delete(&val.clone())
        }
    }
    /// Deletes the biggest key
    ///
    /// If the [SymbolTable] is empty, this function does nothing.
    #[inline]
    fn delete_max(&mut self)
    where
        K: Clone,
        V: Clone,
    {
        let val = self.max();
        if let Some(val) = val {
            self.delete(&val.clone())
        }
    }

    /// Returns an iterator between these two keys
    #[must_use]
    fn iter_between<'a>(
        &'a self,
        lo: &K,
        hi: &K,
    ) -> Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a>;
}
