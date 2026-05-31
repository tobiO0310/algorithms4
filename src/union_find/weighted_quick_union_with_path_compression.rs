use crate::union_find::{generate_tests, UnionFind};

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
    fn union(&mut self, p: usize, q: usize) -> Result<(), &'static str> {
        let p_root = self.find(p).ok_or("could not find pid in QuickFind")?;
        let q_root = self.find(q).ok_or("could not find qid in QuickFind")?;

        if p_root == q_root {
            return Ok(()); // nothing to do lol, p and q are connected already
        };

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
    /// Therefore, it can reasonbly be handled as if it's `O(1)`.
    fn find(&mut self, mut p: usize) -> Option<usize> {
        while p != *self.id.get(p)? {
            // the actual path compression :))
            // basically sets p's "parent" to p's parent's parent
            self.id[p] = *self.id.get(*self.id.get(p)?)?;
            p = self.id[p];
        }
        Some(p)
    }
}

generate_tests!(WeightedQuickUnionWPC);
