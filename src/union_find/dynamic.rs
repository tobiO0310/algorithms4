use crate::union_find::{generate_tests, UnionFind};

/// Quick-Union implemented as Weighted Quick Union with Path Compression,
/// but this implementation allows adding new vertices if need be.
///
/// This is implemented via the [DynamicUnionFind::new_site] and [DynamicUnionFind::vertices] methods.
pub struct DynamicUnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
    n: usize,
    count: usize,
}

impl UnionFind for DynamicUnionFind {
    /// Creates a dynamic union find object.
    ///
    /// The size parameter should be set to the lowest estimate,
    /// and any new
    fn new(size: usize) -> Self {
        assert!(size > 0);

        Self {
            id: (0..size).collect(),
            size: vec![1; size],
            count: size,
            n: size,
        }
    }
    fn count(&self) -> usize {
        self.count
    }

    /// See [UnionFind::union] for details.
    ///
    /// The running time is directly tied and equal to [DynamicUnionFind::find].
    fn union(&mut self, p: usize, q: usize) -> Result<(), String> {
        if p >= self.n {
            return Err(format!("p {} is too high", p));
        }
        if q >= self.n {
            return Err(format!("q {} is too low", q));
        }
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

impl DynamicUnionFind {
    /// Adds a new vertex to the union find object/graph
    ///
    /// ## Panics
    /// This function panics if the number of vertices ever grows above/equal [usize]
    pub fn new_site(&mut self) -> usize {
        assert!(self.n < usize::MAX);

        let id = self.n;
        self.n += 1;
        self.id.push(id);
        self.size.push(1);
        self.count += 1;

        id
    }

    /// Returns the amount of vertices currently active,
    pub fn vertices(&self) -> usize {
        self.n
    }
}

generate_tests!(DynamicUnionFind);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growing() {
        let mut uf = DynamicUnionFind::new(1);
        assert_eq!(uf.vertices(), 1);
        assert_eq!(uf.count(), 1);
        assert_eq!(uf.find(0), Some(0));
        assert_eq!(uf.find(1), None);
        assert_eq!(uf.find(2), None);

        assert_eq!(uf.new_site(), 1); // id should be == 1
        assert_eq!(uf.vertices(), 2);
        assert_eq!(uf.count(), 2);
        assert_eq!(uf.find(0), Some(0));
        assert_eq!(uf.find(1), Some(1));
        assert_eq!(uf.find(2), None);

        assert!(uf.union(0, 2).is_err());
        assert!(uf.union(0, 1).is_ok());

        assert_eq!(uf.vertices(), 2);
        assert_eq!(uf.count(), 1);
        assert_eq!(uf.find(0), Some(0));
        assert_eq!(uf.find(1), Some(0));
        assert_eq!(uf.find(2), None);

        for _ in 3..=200 {
            uf.new_site();
        }

        assert_eq!(uf.vertices(), 200);
        assert_eq!(uf.count(), 199); // 0-1 union
        assert_eq!(uf.find(0), Some(0));
        assert_eq!(uf.find(1), Some(0));
        assert_eq!(uf.find(150), Some(150));
    }
}
