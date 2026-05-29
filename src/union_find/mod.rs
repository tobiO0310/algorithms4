//! This module compiles all the different variants of
//! the Union-Find data structure given in chapter 1.5

mod quick_find;
mod quick_union;
mod weighted_quick_union;
mod weighted_quick_union_with_path_compression;

pub use quick_find::QuickFind;
pub use quick_union::QuickUnion;
pub use weighted_quick_union::WeightedQuickUnion;
pub use weighted_quick_union_with_path_compression::WQUWithPC;

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
///
pub trait UnionFind {
    /// Creates a new Union-Find object
    ///
    /// # Panics
    /// It panics if `size = 0`
    fn new(size: usize) -> Self;
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
    /// Returns an error if either `p` or `q` is bigger than or equal to `size`
    fn union(&mut self, p: usize, q: usize) -> Result<(), &'static str>;
    /// Returns the representative of `p`.
    ///
    /// The representative should be equal for any connected objects.
    ///
    /// ## Errors
    ///
    /// If `p >= size` it returns [None]
    fn find(&mut self, p: usize) -> Option<usize>;
    /// Returns `true` if `p` is connected to `q`
    ///
    /// See [UnionFind] and [UnionFind::find] for mathematical definitions
    fn connected(&mut self, p: usize, q: usize) -> bool {
        let p = self.find(p);
        let q = self.find(q);
        if let Some(p) = p
            && let Some(q) = q
        {
            p == q
        } else {
            false
        }
    }
}

macro_rules! generate_tests {
    ($t:ident) => {
        #[cfg(test)]
        mod tests {
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

use generate_tests;
