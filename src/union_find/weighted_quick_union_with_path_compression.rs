use crate::{
    UnionFindError,
    union_find::{UnionFind, find, generate_tests, get_roots},
};

/// Quick-Union implemented as Weighted Quick Union with Path Compression.
pub struct WeightedQuickUnionWPC {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind for WeightedQuickUnionWPC {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        Self {
            id: (0..size).collect(),
            size: vec![1; size],
            count: size,
        }
    }
    fn count(&self) -> usize {
        self.count
    }

    /// See [UnionFind::union] for details.
    ///
    /// The running time is directly tied and equal to [WeightedQuickUnionWPC::find].
    fn union(&mut self, p: usize, q: usize) -> Result<(), UnionFindError> {
        let (p_root, q_root) = get_roots!(p, q, self);

        if self.size[p_root] < self.size[q_root] {
            self.id[p_root] = q_root;
            self.size[q_root] += self.size[p_root];
        } else {
            self.id[q_root] = p_root;
            self.size[p_root] += self.size[q_root];
        }
        self.count -= 1;

        Ok(())
    }

    /// See [UnionFind::find] for details.
    ///
    /// This runs in `O(log* n)` time, where `log*` is the iterated logarithm.
    /// For any `n` in the range `(65.536, 2^65.536]` `lg*(n) = 5`.
    /// Therefore, it can reasonably be handled as if it's `O(1)`.
    fn find(&mut self, p: usize) -> Option<usize> {
        find!(pc p, self.id)
    }
}

generate_tests!(WeightedQuickUnionWPC);
