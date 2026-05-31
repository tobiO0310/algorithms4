use crate::union_find::{generate_tests, UnionFind};

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
    fn union(&mut self, p: usize, q: usize) -> Result<(), String> {
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
    /// This runs in `O(log n)` time
    fn find(&mut self, mut p: usize) -> Option<usize> {
        while p != *self.id.get(p)? {
            p = self.id[p];
        }
        Some(p)
    }
}

generate_tests!(WeightedQuickUnion);
