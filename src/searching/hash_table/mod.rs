//! This module holds different Hash Table implementations for a [super::SymbolTable]
pub mod seperate;
pub use seperate::{HashTable, SeperateChainingHashTable};
