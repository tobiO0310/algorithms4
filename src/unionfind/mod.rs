//! This module compiles all the different variants of
//! the Union-Find data structure given in chapter 1.5

/// Union-Find is a way to test if a number connects to a number through any number of connections.
///
/// This trait is the overall function
pub trait UF {
    /// Creates a new Union-Find object
    fn new(size: usize) -> Self;
    /// Unions `p` and `q` together.
    fn union(&mut self, p: usize, q: usize);
    /// Finds the representative of `p`
    fn find(&mut self, p: usize) -> usize;
    /// Returns true if the representative of `p` and `q` is the same.
    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    /// Returns the amount of components left
    fn count(&self) -> usize;
}