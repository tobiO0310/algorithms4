use crate::union_find::{UnionFind, find, generate_tests, get_roots};

/// Quick-Union implemented as Quick Union.
pub struct QuickUnion {
    id: Vec<usize>,
    count: usize,
}

impl UnionFind for QuickUnion {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        Self {
            id: (0..size).collect(),
            count: size,
        }
    }
    fn count(&self) -> usize {
        self.count
    }

    /// See [UnionFind::union] for details.
    ///
    /// The running time is directly tied and equal to [QuickUnion::find].
    fn union(&mut self, p: usize, q: usize) -> Result<(), String> {
        let (p_root, q_root) = get_roots!(p, q, self);

        self.id[p_root] = q_root;
        self.count -= 1;

        Ok(())
    }

    /// See [UnionFind::find] for details.
    ///
    /// This runs in `O(n)` time
    fn find(&mut self, p: usize) -> Option<usize> {
        find!(p, self.id)
    }
}

generate_tests!(QuickUnion);
