use crate::{
    UnionFindError,
    union_find::{UnionFind, find, generate_tests, get_roots},
};

/// Quick-Union implemented as Weighted Quick Union.
pub struct WeightedQuickUnion {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind for WeightedQuickUnion {
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
    /// The running time is directly tied and equal to [WeightedQuickUnion::find].
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
    /// This runs in `O(log n)` time
    fn find(&mut self, p: usize) -> Option<usize> {
        find!(p, self.id)
    }
}

generate_tests!(WeightedQuickUnion);
