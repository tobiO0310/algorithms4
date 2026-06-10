//! This module holds different Hash Table implementations for a [super::SymbolTable]
pub mod linear_probing;
pub mod seperate;
pub use linear_probing::LinearProbingHashTable;
pub use seperate::{HashTable, SeperateChainingHashTable};

macro_rules! test_hash_table {
    ($t:ident) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn it_works() {
                let mut items = $t::new();
                println!("test");

                for i in -100..=100 {
                    items.put(i, i);
                }
                println!("test1");

                for i in -100..=100 {
                    assert_eq!(items.get(&i), Some(&i));
                }
                println!("test2");

                for i in -100..=100 {
                    items.delete(&i);
                    assert!(!items.contains(&i));
                    println!("deleted {}", i);
                }
                println!("test3");

                assert_eq!(items.size(), 0, "{:?} has items left", items);
                assert!(items.is_empty(), "{:?} is not empty", items);
            }

            #[test]
            fn it_works_big() {
                let mut items = $t::new();

                for i in -1000..=1000 {
                    items.put(i, i);
                }

                for i in -1000..=1000 {
                    assert_eq!(items.get(&i), Some(&i));
                }

                for i in -1000..=1000 {
                    assert!(items.contains(&i));
                    items.delete(&i);
                    assert!(!items.contains(&i));
                    println!("deleted {}", i);
                }

                assert_eq!(items.size(), 0, "{:?} has items left", items);
                assert!(items.is_empty(), "{:?} is not empty", items);
            }
        }
    };
}
use test_hash_table;
