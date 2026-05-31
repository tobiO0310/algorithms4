use crate::union_find::{generate_tests, UnionFind};

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
        let p_root = self.find(p).ok_or("could not find pid in QuickFind")?;
        let q_root = self.find(q).ok_or("could not find qid in QuickFind")?;

        if p_root == q_root {
            return Ok(()); // nothing to do lol, p and q are connected already
        };

        self.id[p_root] = q_root;
        self.count -= 1;

        Ok(())
    }

    /// See [UnionFind::find] for details.
    ///
    /// This runs in `O(n)` time
    fn find(&mut self, mut p: usize) -> Option<usize> {
        while p != *self.id.get(p)? {
            p = self.id[p];
        }
        Some(p)
    }
}

generate_tests!(QuickUnion);
