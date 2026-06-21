//! Node struct and Color enum

use std::ptr::NonNull;

// do note, link is Copy since NonNull is Copy
pub type Link<K, V> = Option<NonNull<Node<K, V>>>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    #[default]
    Red,
}

impl Color {
    pub fn switch(&mut self) {
        if self == &Color::Red {
            *self = Color::Black;
        } else {
            *self = Color::Red;
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Node<K, V> {
    pub left: Link<K, V>,
    pub right: Link<K, V>,
    pub key: K,
    pub value: V,
    /// total nodes in this sub-tree
    pub size: usize,
    pub color: Color,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
            size: 1,
            color: Color::default(),
        }
    }

    pub fn size(x: Link<K, V>) -> usize {
        match x {
            // SAFETY: v is guaranteed to be initialized, since it is Some
            Some(v) => unsafe { (*v.as_ptr()).size },
            _ => 0,
        }
    }

    pub fn is_red(x: Link<K, V>) -> bool {
        // SAFETY: v is guaranteed to be initialized, since it is Some
        x.is_some_and(|v| unsafe { v.as_ref().color == Color::Red })
    }
}
