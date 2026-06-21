mod models;
mod utilities;
use core::fmt;
use std::{
    cmp::Ordering, error::Error, fmt::Debug, marker::PhantomData, ops::Index,
    ptr::NonNull,
};

use models::*;
use utilities::*;

use crate::{
    OrderedSymbolTable, SymbolTable,
    collections::queue::{self, Queue},
};

/// An ordered symbol table implemented as left-leaning red-black 2-3 search tree.
///
/// This has a guaranteed O(log *n*) running time for [RedBlackBST::get], [RedBlackBST::put] & [RedBlackBST::delete],
/// alongside most ordered operations.
/// Iteration and [RedBlackBST::clear] takes O(*n* log *n*).
///
/// # Examples
///
/// ```
/// # use algorithms4::{RedBlackBST, SymbolTable};
/// let mut bst = RedBlackBST::new();
///
/// bst.put("Test1", 1);
/// bst.put("Test2", 2);
/// bst.put("can", -10);
/// bst.put("corn", 5);
///
/// assert_eq!(bst.get(&"corn"), Some(&5));
/// ```
#[derive(Default)]
pub struct RedBlackBST<K: Ord, V> {
    root: Link<K, V>,
    _data: PhantomData<(K, V)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Checks {
    IsNotInSymmetricOrder,
    SubtreeCountsAreNotConsistent,
    RanksAreNotConsistent,
    NotA23Tree,
    NotBalanced,
}

impl fmt::Display for Checks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for Checks {}

impl<K: Ord, V> RedBlackBST<K, V> {
    /// Initialize a new Binary Search Tree
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: None,
            _data: PhantomData,
        }
    }

    /// Returns an iterator over all the entries of this [RedBlackBST].
    pub fn iter<'a>(&'a self) -> queue::IntoIter<(&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
    {
        if !self.is_empty() {
            let mut queue: Queue<(&'a K, &'a V)> = Queue::new();
            iter_entries(
                self.root,
                &mut queue,
                self.min().unwrap(),
                self.max().unwrap(),
            );
            queue.into_iter()
        } else {
            Queue::new().into_iter()
        }
    }

    /// Returns an iterator between these two keys in this [RedBlackBST]
    pub fn iter_between<'a>(
        &'a self,
        lo: &K,
        hi: &K,
    ) -> queue::IntoIter<(&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
    {
        if !self.is_empty() {
            let mut queue: Queue<(&'a K, &'a V)> = Queue::new();
            iter_entries(self.root, &mut queue, lo, hi);
            queue.into_iter()
        } else {
            Queue::new().into_iter()
        }
    }

    /// Returns [Ok] if this is a valid RedBlack tree.
    /// Else it returns the error where it failed.
    fn check(&self) -> Result<(), Checks> {
        if !Self::is_bst(self.root, None, None) {
            Err(Checks::IsNotInSymmetricOrder)
        } else if !Self::is_size_consistent(self.root) {
            Err(Checks::SubtreeCountsAreNotConsistent)
        } else if !self.is_rank_consistent() {
            Err(Checks::RanksAreNotConsistent)
        } else if !Self::is_23(self.root) {
            Err(Checks::NotA23Tree)
        } else if !self.is_balanced() {
            Err(Checks::NotBalanced)
        } else {
            Ok(())
        }
    }

    /// Indicates whether the sub-tree is actually symmetric and within the correct min and max?
    ///
    /// If `min` or `max` is [None], it is treated as unbounded.
    ///
    /// # Safety
    ///
    /// If `x` is Some, it must be valid.
    fn is_bst(x: Link<K, V>, min: Option<&K>, max: Option<&K>) -> bool {
        if let Some(x) = x {
            // SAFETY: assumed to be valid
            let x = unsafe { x.as_ref() };
            if let Some(min) = min
                && &x.key <= min
            {
                false
            } else if let Some(max) = max
                && &x.key >= max
            {
                false
            } else {
                Self::is_bst(x.left, min, Some(&x.key))
                    && Self::is_bst(x.right, Some(&x.key), max)
            }
        } else {
            true
        }
    }

    /// Are all [Node].size correct?
    ///
    /// # Safety
    ///
    /// If `x` is Some, it must be valid.
    fn is_size_consistent(x: Link<K, V>) -> bool {
        if let Some(x) = x {
            // SAFETY: assumed to be valid
            let x = unsafe { x.as_ref() };
            if x.size != Node::size(x.left) + Node::size(x.right) + 1 {
                false
            } else {
                Self::is_size_consistent(x.left)
                    && Self::is_size_consistent(x.right)
            }
        } else {
            true
        }
    }

    /// Are all [RedBlackBST::rank] correct?
    fn is_rank_consistent(&self) -> bool {
        for i in 0..self.size() {
            if i != self.rank(self.select(i).unwrap()) {
                return false;
            }
        }
        for (key, _) in self.iter() {
            if key != self.select(self.rank(key)).unwrap() {
                return false;
            }
        }

        true
    }

    /// Is this a correct Red-Black 2-3 tree?
    /// Checks for;
    ///
    /// - no right-leaning red links
    /// - at most one left-leaning red links in a row on any path
    ///
    /// # Safety
    ///
    /// If `x` is Some, it must be valid.
    fn is_23(x: Link<K, V>) -> bool {
        if let Some(x) = x {
            // SAFETY: assumed to be valid
            let x = unsafe { x.as_ref() };
            if Node::is_red(x.right) {
                return false;
            }
            if x.color == Color::Red && Node::is_red(x.left) {
                return false;
            }

            Self::is_23(x.left) && Self::is_23(x.right)
        } else {
            true
        }
    }

    /// Indicates whether all paths from root to a leaf has the same number of `black` black links.
    ///
    /// # Safety
    ///
    /// If `x` is Some, it must be valid.
    fn is_balanced(&self) -> bool {
        let mut black = 0; // number of black links from root to min
        let mut current = self.root;
        while current.is_some() {
            if !Node::is_red(current) {
                black += 1;
            }
            current = unsafe { current.unwrap().as_ref().left }
        }

        Self::is_node_balanced(self.root, black)
    }

    /// Indicates whether all paths from sub-tree `x` to a leaf has the same number of `black` black links.
    ///
    /// # Safety
    ///
    /// If `x` is Some, it must be valid.
    fn is_node_balanced(x: Link<K, V>, mut black: usize) -> bool {
        if let Some(x) = x {
            // SAFETY: assumed to be valid
            let x = unsafe { x.as_ref() };
            if x.color == Color::Black {
                black -= 1;
            }

            Self::is_node_balanced(x.left, black)
                && Self::is_node_balanced(x.right, black)
        } else {
            black == 0
        }
    }
}

/// Recursively iterates over all nodes, and enqueues all key-value reference tuples
/// where `lo <= key <= hi` according to the natural order.
fn into_iter_entries<K: Ord, V>(x: Link<K, V>, queue: &mut Queue<(K, V)>) {
    if let Some(n) = x {
        // SAFETY: as long as x.is_some(), x is guaranteed to be initialized
        let node = unsafe { Box::from_raw(n.as_ptr()) };

        into_iter_entries(node.left, queue);
        queue.enqueue((node.key, node.value)); // take it
        into_iter_entries(node.right, queue);
    }
}

/// Recursively iterates over all nodes, and enqueues all key-value reference tuples
/// where `lo <= key <= hi` according to the natural order.
fn iter_entries<K: Ord, V>(
    x: Link<K, V>,
    queue: &mut Queue<(&K, &V)>,
    lo: &K,
    hi: &K,
) {
    if let Some(n) = x {
        // SAFETY: as long as x.is_some(), x is guaranteed to be initialized
        let node = unsafe { &*n.as_ptr() };
        let cmp_lo = lo.cmp(&node.key);
        let cmp_hi = hi.cmp(&node.key);
        if cmp_lo.is_lt() {
            iter_entries(node.left, queue, lo, hi);
        }
        if cmp_lo.is_le() && cmp_hi.is_ge() {
            queue.enqueue((&node.key, &node.value));
        }
        if cmp_hi.is_gt() {
            iter_entries(node.right, queue, lo, hi);
        }
    }
}

impl<K: Ord, V> SymbolTable<K, V> for RedBlackBST<K, V> {
    fn size(&self) -> usize {
        Node::size(self.root)
    }

    fn put(&mut self, key: K, value: V) {
        unsafe {
            // SAFETY: put is called correctly, keeping invariants in place
            self.root = put(self.root, key, value);
            // SAFETY: as long as root is not None, it has been initialized and can be used
            self.root.unwrap().as_mut().color = Color::Black;
        }
        debug_assert_eq!(self.check(), Ok(()));
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut current = self.root;
        // while the direction to the searched key exists
        while current.is_some() {
            // SAFETY: nodes are guaranteed to be initialized, IF current.is_some()
            // Because they are only created in put method
            let node = unsafe { &*current?.as_ptr() };
            match key.cmp(&node.key) {
                Ordering::Less => current = node.left,
                Ordering::Greater => current = node.right,
                Ordering::Equal => return Some(&node.value),
            };
        }
        None
    }

    fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone,
    {
        if !self.contains(key) || self.root.is_none() {
            return;
            // if it does not exist, or there is no root (is_empty()),
            // don't try to remove it
        }

        // SAFETY: at this point, self.root is Some, and therefore is guaranteed to be initialized
        unsafe {
            // if both children of root are black, set root to red
            let root = self.root.unwrap().as_mut();
            if !Node::is_red(root.left) && !Node::is_red(root.right) {
                root.color = Color::Red;
            }

            self.root = delete(self.root, key);
            if let Some(mut root) = self.root {
                // SAFETY: as long as root is not None, it has been initialized and can be used
                // (root can None if min() was equal to root)
                root.as_mut().color = Color::Black
            }
        };

        debug_assert!(!self.contains(key));
        debug_assert_eq!(self.check(), Ok(()));
    }

    fn clear(&mut self) {
        drop_tree(self.root.take());
        debug_assert!(self.is_empty());
        debug_assert_eq!(self.size(), 0);
        debug_assert_eq!(self.root, None);
        debug_assert_eq!(self.check(), Ok(())); // idiomatic, but just to be sure
    }
}

impl<K: Ord, V> OrderedSymbolTable<K, V> for RedBlackBST<K, V> {
    fn delete_min(&mut self)
    where
        K: Clone,
        V: Clone,
    {
        if self.root.is_none() {
            return;
            // if there is no root (is_empty()), don't try to delete minimum
            // (there cannot be any)
        }

        // SAFETY: at this point, self.root is Some, and therefore is guaranteed to be initialized
        unsafe {
            // if both children of root are black, set root to red
            let root = self.root.unwrap().as_mut();
            if !Node::is_red(root.left) && !Node::is_red(root.right) {
                root.color = Color::Red;
            }

            // SAFETY: called correctly
            self.root = delete_min(self.root);
            if let Some(mut root) = self.root {
                // SAFETY: as long as root is not None, it has been initialized and can be used
                // (root can None if min() was equal to root)
                root.as_mut().color = Color::Black
            }
        };

        debug_assert_eq!(self.check(), Ok(()));
    }

    fn delete_max(&mut self)
    where
        K: Clone,
        V: Clone,
    {
        if self.root.is_none() {
            return;
            // if there is no root (is_empty()), don't try to delete minimum
            // (there cannot be any)
        }

        // SAFETY: at this point, self.root is Some, and therefore is guaranteed to be initialized
        unsafe {
            // if both children of root are black, set root to red
            let root = self.root.unwrap().as_mut();
            if !Node::is_red(root.left) && !Node::is_red(root.right) {
                root.color = Color::Red;
            }

            // SAFETY: called correctly
            self.root = delete_max(self.root);
            if let Some(mut root) = self.root {
                // SAFETY: as long as root is not None, it has been initialized and can be used
                // (root can None if min() was equal to root)
                root.as_mut().color = Color::Black
            }
        };

        debug_assert_eq!(self.check(), Ok(()));
    }

    fn min(&self) -> Option<&K> {
        // SAFETY: as long as min(self.root) is some (guaranteed by try operator),
        // it is guaranteed to be initialized, and key can therefore be referenced
        unsafe { Some(&(*min(self.root)?.as_ptr()).key) }
    }

    fn max(&self) -> Option<&K> {
        let mut current = self.root;
        while current.is_some() {
            // SAFETY: nodes are guaranteed to be initialized,
            // as this requires current to be Some (and therefore initialized)
            let node = unsafe { current.unwrap().as_ref() };
            if node.right.is_none() {
                return Some(&node.key);
            } else {
                current = node.right;
            }
        }
        None
    }

    fn floor(&self, key: &K) -> Option<&K> {
        // SAFETY: if floor(self.root) == None, it propagates None,
        // else floor(self.root) is guaranteed to be initialized
        unsafe { Some(&(*floor(self.root, key)?.as_ptr()).key) }
    }

    fn ceiling(&self, key: &K) -> Option<&K> {
        // SAFETY: if ceiling(self.root) == None, it propagates None,
        // else ceiling(self.root) is guaranteed to be initialized
        unsafe { Some(&(*ceiling(self.root, key)?.as_ptr()).key) }
    }

    fn rank(&self, key: &K) -> usize {
        rank(self.root, key)
    }

    fn select(&self, mut rank: usize) -> Option<&K> {
        let mut current = self.root;
        while current.is_some() {
            // SAFETY: nodes are guaranteed to be initialized, as x would propagate None if x.is_none()
            let node = unsafe { current?.as_ref() };
            let t = Node::size(node.left);
            match t.cmp(&rank) {
                Ordering::Greater => current = node.left,
                Ordering::Equal => return Some(&node.key),
                Ordering::Less => {
                    current = node.right;
                    rank = rank - 1 - t; // t may be 0 (and therefore -= cannot be used)
                }
            }
        }
        None
    }
}

/// Drops each node recursively
///
/// The dropping happens with DFS-like behavior.
///
/// # Safety
///
/// You CANNOT use the link afterwards, as the underlying.
/// The easiest way to complete this is using
/// ```
/// # let mut link = Some(true);
/// # fn drop_tree(x: Option<bool>) {}
/// drop_tree(link.take());
/// ```
fn drop_tree<K, V>(link: Link<K, V>) {
    if let Some(mut node) = link {
        // SAFETY: as long as node is not None, it is guaranteed to be initialized
        unsafe {
            let node_ref = node.as_mut();
            drop_tree(node_ref.left.take());
            drop_tree(node_ref.right.take());
            drop(Box::from_raw(node.as_ptr()));
        }
    }
}

impl<K: Ord, V> Drop for RedBlackBST<K, V> {
    fn drop(&mut self) {
        drop_tree(self.root.take());
    }
}
impl<K: Clone + Ord, V: Clone> Clone for RedBlackBST<K, V> {
    fn clone(&self) -> Self {
        let mut new_bst = Self::new();
        for (key, value) in self {
            new_bst.put(key.clone(), value.clone());
        }
        new_bst
    }
}
impl<K: Ord, V> Extend<(K, V)> for RedBlackBST<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.put(key, value);
        }
    }
}
impl<K: Ord, V> FromIterator<(K, V)> for RedBlackBST<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}
impl<K: Ord, V> Index<&K> for RedBlackBST<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found in SymbolTable")
    }
}
impl<K: Ord, V: PartialEq> PartialEq for RedBlackBST<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.iter().eq(other)
    }
}
impl<K: Ord, V: Eq> Eq for RedBlackBST<K, V> {}
impl<K: Ord + fmt::Debug, V: fmt::Debug> fmt::Debug for RedBlackBST<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K: Ord, V> IntoIterator for RedBlackBST<K, V> {
    type Item = (K, V);
    type IntoIter = queue::IntoIter<(K, V)>;

    fn into_iter(mut self) -> Self::IntoIter {
        if !self.is_empty() {
            let mut queue = Queue::new();
            into_iter_entries(self.root.take(), &mut queue);
            queue.into_iter()
        } else {
            Queue::new().into_iter()
        }
    }
}
impl<'a, K: Ord, V> IntoIterator for &'a RedBlackBST<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = queue::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::successors;

    use super::*;

    fn swap_iter() -> impl Iterator<Item = i32> {
        successors(Some(0), |&n| {
            if n == 0 {
                Some(1)
            } else if n < 0 {
                Some(-n + 1)
            } else {
                Some(-n)
            }
        })
    }

    #[test]
    fn it_works() {
        let mut bst = RedBlackBST::new();

        for i in swap_iter().take_while(|&n| n.abs() <= 100) {
            bst.put(i, i);
        }

        for i in swap_iter().take_while(|&n| n.abs() <= 100) {
            assert_eq!(bst.get(&i), Some(&i));
        }

        for i in swap_iter().take_while(|&n| n.abs() <= 100) {
            bst.delete(&i);
            assert!(!bst.contains(&i));
            println!("deleted {}", i);
        }

        assert_eq!(bst.size(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn it_works_big() {
        let mut bst = RedBlackBST::new();

        for i in swap_iter().take_while(|&n| n.abs() <= 1_000) {
            bst.put(i, i);
        }

        for i in swap_iter().take_while(|&n| n.abs() <= 1_000) {
            assert_eq!(bst.get(&i), Some(&i));
        }

        for i in swap_iter().take_while(|&n| n.abs() <= 1_000) {
            bst.delete(&i);
            assert!(!bst.contains(&i));
            println!("deleted {}", i);
        }

        assert_eq!(bst.size(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn traits_test() {
        let mut bst = RedBlackBST::new();

        for i in swap_iter().take_while(|&n| n.abs() <= 100) {
            bst.put(i, i);
        }

        let copy = bst.clone();

        assert_eq!(bst, copy);

        println!("{:?}", copy);

        for (i, v) in bst.into_iter() {
            assert_eq!(i, v);
            assert_eq!(copy.get(&i), Some(&v));
        }

        let vec = vec![(10, 10), (9, 9), (5, 5)];
        let mut bst = RedBlackBST::from_iter(vec.clone().into_iter());
        assert!(bst.contains(&5));
        assert!(!bst.contains(&7));
        assert!(!bst.contains(&8));
        assert!(bst.contains(&9));
        assert!(bst.contains(&10));
        assert_eq!(bst.size(), 3);
        assert!(!bst.is_empty());

        for (i, v) in &vec {
            assert_eq!(bst.get(i), Some(v));
            bst.delete(i);
        }
        assert_eq!(bst.size(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn basic_test_order() {
        let mut bst = RedBlackBST::new();

        for i in swap_iter().take_while(|&n| n.abs() <= 10) {
            bst.put(i * 2, (i * 2).to_string());
        }

        assert_eq!(bst.min(), Some(&-20));
        assert_eq!(bst.max(), Some(&20));

        bst.delete_min();
        bst.delete_max();

        assert_eq!(bst.min(), Some(&-18));
        assert_eq!(bst.max(), Some(&18));

        assert_eq!(bst.floor(&17), Some(&16));
        assert_eq!(bst.ceiling(&-17), Some(&-16));

        assert_eq!(bst.floor(&18), Some(&18));
        assert_eq!(bst.ceiling(&-18), Some(&-18));

        assert_eq!(bst.rank(&18), bst.size() - 1);
        // the -1 is because rank is about of keys below it.
        assert_eq!(bst.select(9), Some(&0));
        // -18, -16, -14, -12, -10, -8, -6, -4, -2 (total 9 keys < 0 -> select(9) == 0)

        // test iter_between returns ONLY and ALL (key, value) st. lo <= key <= hi
        let mut vec = vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 10];
        for (i, _) in bst.iter_between(&-10, &10) {
            assert!(&-10 <= i && i <= &10);
            assert!(vec.contains(i));
            vec.swap_remove(vec.iter().position(|j| j == i).unwrap());
        }
        assert!(vec.is_empty());
    }
}
