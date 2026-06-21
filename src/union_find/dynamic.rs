use crate::{
    UnionFindError,
    union_find::{UnionFind, find, generate_tests, get_roots},
};

/// Quick-Union implemented as Weighted Quick Union with Path Compression,
/// but this implementation allows adding new vertices if need be.
///
/// This is implemented via the [DynamicUnionFind::new_site] and [DynamicUnionFind::vertices] methods.
pub struct DynamicUnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
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
        }
    }
    fn count(&self) -> usize {
        self.count
    }

    /// See [UnionFind::union] for details.
    ///
    /// The running time is directly tied and equal to [DynamicUnionFind::find].
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
    /// Time is documented in [super::WeightedQuickUnionWPC]'s [super::WeightedQuickUnionWPC::find] method
    fn find(&mut self, p: usize) -> Option<usize> {
        find!(pc p, self.id)
    }
}

impl DynamicUnionFind {
    /// Adds a new vertex to the union find object/graph
    ///
    /// At the risk that the implementation changes,
    /// please utilize the returned id for the new vertex.
    ///
    /// ## Panics
    /// This function panics if the number of vertices ever grows above/equal [usize]
    #[must_use]
    pub fn new_site(&mut self) -> usize {
        assert!(self.id.len() < usize::MAX);

        let id = self.id.len();
        self.id.push(id);
        self.size.push(1);
        self.count += 1;

        id
    }

    /// Returns the amount of vertices currently active
    #[must_use]
    pub fn vertices(&self) -> usize {
        self.id.len()
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

        for i in 3..=200 {
            let j = uf.new_site();
            assert_eq!(i - 1, j);
        }

        assert_eq!(uf.vertices(), 200);
        assert_eq!(uf.count(), 199); // 0-1 union
        assert_eq!(uf.find(0), Some(0));
        assert_eq!(uf.find(1), Some(0));
        assert_eq!(uf.find(150), Some(150));
    }
}
