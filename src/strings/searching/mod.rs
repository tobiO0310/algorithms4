use crate::SymbolTable;

mod ternary;
mod tries;
pub use ternary::TernarySearchTrie;
pub use tries::Trie;

/// A [StringSymbolTable] is a specialized [SymbolTable] for strings.
///
/// This may clone the keys during computations, so performance may vary.
pub trait StringSymbolTable<V>: SymbolTable<String, V> {
    /// This function returns the longest key that is a prefix of `s`.
    #[must_use]
    fn longest_prefix_of(&self, s: &str) -> String;
    /// This function returns all the entries that has `s` as a prefix in the key.
    #[must_use]
    fn entries_with_prefix<'a>(
        &'a self,
        s: &str,
    ) -> impl Iterator<Item = (String, &'a V)> where V: 'a;
    /// This function returns all the entries that has a key that match `s`. (where `.` matches any character)
    #[must_use]
    fn entries_that_match<'a>(&'a self, s: &str)
    -> impl Iterator<Item = (String, &'a V)>  where V: 'a;

    /// This function returns all the keys that has `s` as a prefix.
    #[inline]
    #[must_use]
    fn keys_with_prefix<'a>(&'a self, s: &str) -> impl Iterator<Item = String>  where V: 'a {
        self.entries_with_prefix(s).map(|(s, _)| s)
    }
    /// This function returns all the keys that match `s`. (where `.` matches any character)
    #[inline]
    #[must_use]
    fn keys_that_match<'a>(&'a self, s: &str) -> impl Iterator<Item = String>  where V: 'a {
        self.entries_that_match(s).map(|(s, _)| s)
    }
}
