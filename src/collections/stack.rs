//! This module contains the code to create a simple Pushdown stack

use std::{
    cmp::Ordering,
    fmt,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ptr::NonNull,
};

use crate::collections::extend_singly_linked_list;

/// A stack (LIFO) implemented as a linked list.
///
/// All methods are `O(1)` expect iterators which are `O(n)`
#[derive(Default)]
pub struct Stack<T> {
    head: Link<T>,
    len: usize,
    _data: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    elem: T,
}

impl<T> Stack<T> {
    /// Creates a new stack
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0,
            _data: PhantomData,
        }
    }

    /// Returns the amount of elements in the stack
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indicates whether the stack is empty or not
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Pushes an element onto the stack
    pub fn push(&mut self, elem: T) {
        // SAFETY: linked lists, uhh :3
        unsafe {
            // creates the actual NonNull node
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                next: None,
                elem,
            })));
            if let Some(old) = self.head {
                (*new.as_ptr()).next = Some(old);
            }
            self.head = Some(new);
            self.len += 1;
        }
    }

    /// Pops the last inserted element, if any exist
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            // only do stuff if head actually exists lol

            self.head.map(|node| {
                // get the box black from the raw pointer and drop it at the end of this call :3
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                // make sure to set next head and length
                self.head = boxed_node.next;
                self.len -= 1;

                result
            })
        }
    }

    /// Peeks at the top element of the stack
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        unsafe { Some(&(*self.head?.as_ptr()).elem) }
    }

    /// Gets a mutable reference to the next element
    #[must_use]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut (*self.head?.as_ptr()).elem) }
    }

    /// Returns an iterator for the stack
    #[must_use]
    pub fn iter(&'_ self) -> Iter<'_, T> {
        Iter {
            curr: self.head,
            len: self.len,
            _boo: PhantomData,
        }
    }

    /// Returns an iterator for the stack with mutable references instead
    #[must_use]
    pub fn iter_mut(&'_ mut self) -> IterMut<'_, T> {
        IterMut {
            curr: self.head,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

extend_singly_linked_list!(Stack<T>; Link<T>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut list = Stack::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        list.push(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        list.push(10);
        assert_eq!(list.len(), 1);
        list.push(20);
        assert_eq!(list.len(), 2);
        list.push(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some(30));
        assert_eq!(list.len(), 2);
        list.push(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }
}
