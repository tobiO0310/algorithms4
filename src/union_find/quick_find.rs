use crate::union_find::{UnionFind, generate_tests};

/// Quick-Union implemented as Quick Find.
pub struct QuickFind {
    id: Vec<usize>,
    count: usize,
}

impl UnionFind for QuickFind {
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
    /// This runs in `O(n)` time
    fn union(&mut self, p: usize, q: usize) -> Result<(), String> {
        let pid = self.find(p).ok_or("could not find pid in QuickFind")?;
        let qid = self.find(q).ok_or("could not find qid in QuickFind")?;

        if pid == qid {
            return Ok(()); // nothing to do lol, p and q are connected already
        };

        // actually move over to the others LOL
        for v in self.id.iter_mut() {
            if *v == pid {
                *v = qid;
            }
        }
        self.count -= 1;

        Ok(())
    }

    /// See [UnionFind::find] for details.
    ///
    /// This runs in `O(1)` time
    fn find(&mut self, p: usize) -> Option<usize> {
        self.id.get(p).copied()
    }
}

generate_tests!(QuickFind);
