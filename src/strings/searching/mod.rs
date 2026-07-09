use crate::SymbolTable;

mod substring;
mod ternary;
mod tries;
pub use substring::{
    BoyerMoorePattern, KMPPattern, RabinKarp, SubstringSearch,
};
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
    ) -> impl Iterator<Item = (String, &'a V)>
    where
        V: 'a;
    /// This function returns all the entries that has a key that match `s`. (where `.` matches any character)
    #[must_use]
    fn entries_that_match<'a>(
        &'a self,
        s: &str,
    ) -> impl Iterator<Item = (String, &'a V)>
    where
        V: 'a;

    /// This function returns all the keys that has `s` as a prefix.
    #[inline]
    #[must_use]
    fn keys_with_prefix<'a>(&'a self, s: &str) -> impl Iterator<Item = String>
    where
        V: 'a,
    {
        self.entries_with_prefix(s).map(|(s, _)| s)
    }
    /// This function returns all the keys that match `s`. (where `.` matches any character)
    #[inline]
    #[must_use]
    fn keys_that_match<'a>(&'a self, s: &str) -> impl Iterator<Item = String>
    where
        V: 'a,
    {
        self.entries_that_match(s).map(|(s, _)| s)
    }
}

macro_rules! test_string_table {
    ($e:ident) => {
        #[cfg(test)]
        mod tests {
            use super::{super::super::RANDOM_STRINGS_FOR_TESTING, *};

            fn vec(size: usize) -> Vec<String> {
                let mut vec = Vec::with_capacity(size);
                for i in 0..size {
                    vec.push(RANDOM_STRINGS_FOR_TESTING[i].to_string());
                }
                vec
            }

            #[test]
            fn it_works() {
                let mut trie1 = $e::new();
                const SIZE: usize = 100;

                assert_eq!(trie1.size, 0);
                assert!(trie1.is_empty());

                let vec = vec(SIZE);
                for (i, str) in vec.iter().enumerate() {
                    trie1.put(str.clone(), str.clone());
                    assert_eq!(trie1.size, i + 1);
                }

                assert_eq!(trie1.size, SIZE);
                assert!(!trie1.is_empty());

                for str in &vec {
                    assert_eq!(trie1.get(str), Some(str));
                    assert_eq!(&trie1[str], str);
                }

                let mut trie2 = trie1.clone();

                trie2.clear();
                assert_eq!(trie2.size, 0);
                assert!(trie2.is_empty());

                for (i, str) in vec.iter().enumerate().rev() {
                    trie1.delete(str);
                    assert_eq!(trie1.size, i);
                }

                assert_eq!(trie1.size, 0);
                assert!(trie1.is_empty());

                for str in &vec {
                    trie1.delete(str);
                    assert_eq!(trie1.size, 0);
                    assert!(trie1.is_empty());
                }
            }

            #[test]
            fn patterns_work() {
                let mut trie = $e::new();
                const SIZE: usize = 100;

                assert_eq!(trie.size, 0);
                assert!(trie.is_empty());

                let vec = vec(SIZE);
                for (i, str) in vec.iter().enumerate() {
                    trie.put(str.clone(), str.clone());
                    assert_eq!(trie.size, i + 1);
                }

                assert_eq!(trie.entries_that_match("").collect::<Vec<_>>().len(), 0);
                assert_eq!(
                    trie.entries_that_match("....").collect::<Vec<_>>().len(),
                    15
                );
                assert_eq!(
                    trie.entries_that_match("......").collect::<Vec<_>>().len(),
                    21
                );
                for str in RANDOM_STRINGS_FOR_TESTING {
                    assert_eq!(
                        trie.entries_that_match(&str).collect::<Vec<_>>().len(),
                        1
                    );
                }
            }

            #[test]
            fn prefixes_work() {
                let mut trie = $e::new();
                const SIZE: usize = 100;

                assert_eq!(trie.size, 0);
                assert!(trie.is_empty());

                let vec = vec(SIZE);
                for (i, str) in vec.iter().enumerate() {
                    trie.put(str.clone(), str.clone());
                    assert_eq!(trie.size, i + 1);
                }

                assert_eq!(
                    trie.entries_with_prefix("").collect::<Vec<_>>().len(),
                    SIZE
                );
            }

            #[test]
            fn longest_prefix() {
                let mut trie = $e::new();
                const SIZE: usize = 100;

                assert_eq!(trie.size, 0);
                assert!(trie.is_empty());

                let vec = vec(SIZE);
                for (i, str) in vec.iter().enumerate() {
                    trie.put(str.clone(), str.clone());
                    assert_eq!(trie.size, i + 1);
                }

                for str in &vec {
                    assert_eq!(&trie.longest_prefix_of(str), str);
                }

                assert_eq!(trie.longest_prefix_of("rabbit hole"), "rabbit".to_string());
                assert_eq!(trie.longest_prefix_of("haunting"), "haunt".to_string());
                assert_eq!(trie.longest_prefix_of("taper"), "tape".to_string());
                assert_eq!(trie.longest_prefix_of("given"), "give".to_string());
                assert_eq!(trie.longest_prefix_of("bloody mary"), "bloody".to_string());

                assert_eq!(trie.longest_prefix_of("a"), "".to_string());
            }
        }
    };
}
use test_string_table;
