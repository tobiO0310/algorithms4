//! This module holds different Hash Table implementations for a [super::SymbolTable]
pub mod linear_probing;
pub mod seperate;
pub use linear_probing::LinearProbingHashTable;
pub use seperate::{HashTable, SeperateChainingHashTable};
