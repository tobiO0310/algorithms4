//! This module compiles all the different variants of
//! the Union-Find data structure given in chapter 1.5

mod dynamic;
mod quick_find;
mod quick_union;
mod quick_union_with_path_compression;
mod weighted_quick_union;
mod weighted_quick_union_with_path_compression;

pub use dynamic::DynamicUnionFind;
pub use quick_find::QuickFind;
pub use quick_union::QuickUnion;
pub use quick_union_with_path_compression::QuickUnionWPC;
pub use weighted_quick_union::WeightedQuickUnion;
pub use weighted_quick_union_with_path_compression::WeightedQuickUnionWPC;

/// The errors generated
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum UnionFindError {
    #[allow(missing_docs)] // struct fields here are pretty self-explanatory LOL
    /// An out-of-bounds error, given when a user supplied a number which is too high
    OutOfBounds { index: usize, len: usize },
}

impl std::fmt::Display for UnionFindError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { index, len } => {
                write!(f, "index {} is out of bounds (len is {})", index, len)
            }
        }
    }
}

impl std::error::Error for UnionFindError {}

/// Union-Find is a solution to the dynamic connectivity problem.
///
/// ## Implementation Notes
///
/// All implementations should implement the functions as they're described.
/// Nodes are in the same set/unioned together, if their representative is the same; see [UnionFind::find].
///
/// ## Mathematical Notes
///
/// "`p` is connected to `q`" means there exists a path between `p` and `q`. Alongside this,
/// it is assumed that "is connected to" is an *equivalence* relation.
/// `p` is connected to `q` iff `p` and `q` are in the same equivalence class
/// in the aforementioned equivalence relation.
pub trait UnionFind {
    /// Creates a new Union-Find object with <em>n</em> elements
    ///
    /// # Panics
    /// It panics if `n = 0`
    fn new(n: usize) -> Self;
    /// Returns the amount of components left
    ///
    /// The amount of components left is equal to the amount of equivalence classes left.
    fn count(&self) -> usize;
    /// Unions `p` and `q` together.
    ///
    /// The union operation needs to uphold the mutually disjoint invariant.
    /// As such, each union operation must decrease the amount of components by 1,
    /// UNLESS `p` and `q` are already in the same set.
    ///
    /// ## Errors
    ///
    /// Returns an error if either `p` or `q` is bigger than or equal to `n`
    fn union(&mut self, p: usize, q: usize) -> Result<(), UnionFindError>;
    /// Returns the representative of `p`.
    ///
    /// The representative should be equal for any connected objects.
    ///
    /// ## Errors
    ///
    /// If `p >= n` it returns [None]
    fn find(&mut self, p: usize) -> Option<usize>;
    /// Returns `true` if `p` is connected to `q`
    ///
    /// See [UnionFind] and [UnionFind::find] for mathematical definitions
    #[inline] // connected is just `find(p) == find(q)` and they both exist
    fn connected(&mut self, p: usize, q: usize) -> bool {
        matches!((self.find(p), self.find(q)), (Some(p_root), Some(q_root)) if p_root == q_root)
    }
}

macro_rules! generate_tests {
    ($t:ident) => {
        #[cfg(test)]
        mod uf_tests {
            use super::*;

            #[test]
            fn test_basic() {
                let mut quf = $t::new(10);
                for i in 1..10 {
                    assert!(!quf.connected(0, i)); // should not be connected
                }
                assert_eq!(quf.count(), 10);
                assert_eq!(quf.union(0, 1), Ok(()));
                assert_eq!(quf.count(), 9); // removed one component
                assert_eq!(quf.union(1, 0), Ok(()));
                assert_eq!(quf.count(), 9); // did not actually change the data structure

                for i in 2..10 {
                    assert!(!quf.connected(0, i)); // should not be connected
                    assert!(!quf.connected(1, i)); // should not be connected
                }
                assert!(quf.connected(0, 0));
                assert!(quf.connected(0, 1));
                assert!(quf.connected(1, 1));

                assert_eq!(quf.union(2, 3), Ok(()));
                assert_eq!(quf.count(), 8); // 0-1, 2-3, 4, 5, 6, 7, 8, 9
                assert_eq!(quf.union(4, 4), Ok(()));
                assert_eq!(quf.count(), 8); // should not change data structure
                assert_eq!(quf.union(4, 5), Ok(()));
                assert_eq!(quf.count(), 7);
                assert_eq!(quf.union(6, 7), Ok(()));
                assert_eq!(quf.count(), 6);
                assert_eq!(quf.union(8, 9), Ok(()));
                assert_eq!(quf.count(), 5);

                assert!(quf.union(9, 10).is_err()); // 10 is not included
                assert_eq!(quf.count(), 5);

                assert_eq!(quf.union(1, 2), Ok(()));
                assert_eq!(quf.count(), 4);
                assert!(quf.connected(0, 3));
                assert!(!quf.connected(1, 4));
                assert!(!quf.connected(2, 5));

                assert_eq!(quf.union(0, 4), Ok(()));
                assert_eq!(quf.count(), 3);
                assert!(quf.connected(3, 5));
                assert!(!quf.connected(4, 6));
                assert_eq!(quf.union(2, 6), Ok(()));
                assert_eq!(quf.count(), 2);
                assert!(quf.connected(4, 6));
                assert!(!quf.connected(5, 8));
                assert_eq!(quf.union(4, 5), Ok(()));
                assert_eq!(quf.count(), 2);
                assert!(!quf.connected(5, 8));
                assert_eq!(quf.union(7, 9), Ok(()));
                assert_eq!(quf.count(), 1);
            }
        }
    };
}

macro_rules! find {
    ($p:ident, $id:expr) => {{
        let mut p = $p;
        while p != *$id.get(p)? {
            p = $id[p];
        }
        Some(p)
    }};
    (pc $p:ident, $id:expr) => {{
        let mut p = $p;
        while p != *$id.get(p)? {
            $id[p] = *$id.get(*$id.get(p)?)?; // path compression
            p = $id[p];
        }
        Some(p)
    }};
}

macro_rules! get_roots {
    ($p:ident, $q:ident, $self:ident) => {{
        let p_root = $self.find($p).ok_or(UnionFindError::OutOfBounds {
            index: $p,
            len: $self.id.len(),
        })?;
        let q_root = $self.find($q).ok_or(UnionFindError::OutOfBounds {
            index: $q,
            len: $self.id.len(),
        })?;

        if p_root == q_root {
            return Ok(()); // nothing to do lol, p and q are connected already
        };

        (p_root, q_root)
    }};
}

use find;
use generate_tests;
use get_roots;
