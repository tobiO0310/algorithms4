//! This module holds the code to create a simple FIFO queue

use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Index,
    ptr::NonNull,
};

use crate::collections::extend_singly_linked_list;

/// A FIFO queue implementation as a doubly linked list.
///
/// All methods are `O(1)` expect iterators which are `O(n)`
#[derive(Default)]
pub struct Queue<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _data: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    /// Creates a new queue
    #[must_use]
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _data: PhantomData,
        }
    }

    /// Indicates whether the queue is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.front.is_none()
    }

    /// The length of the queue
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Enqueues the given element to this queue
    pub fn enqueue(&mut self, elem: T) {
        // SAFETY: linked lists, am I right?
        unsafe {
            // creates the actual NonNull node
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                next: None,
                elem,
            })));
            if let Some(old) = self.back {
                (*old.as_ptr()).next = Some(new);
            } else {
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }

    /// An alias for [Queue::enqueue]
    pub fn push(&mut self, elem: T) {
        self.enqueue(elem)
    }

    /// Dequeues the last inserted element from this queue, if it exists
    #[must_use]
    pub fn dequeue(&mut self) -> Option<T> {
        // SAFETY: uhh linked lists again LOL
        unsafe {
            self.front.map(|node| {
                // get the box black from the raw pointer and drop it at the end of this call :3
                let boxed_node = Box::from_raw(node.as_ptr());
                let elem = boxed_node.elem;

                self.front = boxed_node.next;
                if self.front.is_none() {
                    self.back = None; // if front is none, list is empty
                }
                self.len -= 1;

                elem
            })
        }
    }

    /// An alias for [Queue::dequeue]
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        self.dequeue()
    }

    /// Peeks at the top element of the queue
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        unsafe { Some(&(*self.front?.as_ptr()).elem) }
    }

    /// Gets a mutable reference to the next element to be dequeued
    #[must_use]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut (*self.front?.as_ptr()).elem) }
    }

    /// Returns an iterator for the queue
    #[must_use]
    pub fn iter(&'_ self) -> Iter<'_, T> {
        Iter {
            curr: self.front,
            len: self.len,
            _boo: PhantomData,
        }
    }

    /// Returns an iterator for the queue with mutable references instead
    #[must_use]
    pub fn iter_mut(&'_ mut self) -> IterMut<'_, T> {
        IterMut {
            curr: self.front,
            len: self.len,
            _boo: PhantomData,
        }
    }

    /// Deletes an element inside the queue. Returns `true` if key was found and deleted.
    #[must_use]
    pub fn delete_inner(&mut self, elem: &T) -> bool
    where
        T: Eq,
    {
        // SAFETY: handles raw pointers a lot, while keeping invariants
        unsafe {
            if let Some(f) = self.front {
                // if the key is front, delete it and restore links to keep invariants and prevent memory leak
                if &f.as_ref().elem == elem {
                    self.front = f.as_ref().next;
                    let _ = Box::from_raw(f.as_ptr());
                    return true;
                }
            }
            let current = self.front;
            while let Some(mut node) = current {
                let node = node.as_mut();
                if let Some(next) = node.next
                    && &next.as_ref().elem == elem
                {
                    // next node is the node to be removed, so get it's box,
                    // and set node.next = node.next.next to avoid null pointers, memory leak
                    // (and therefore also missing data)
                    let n = Box::from_raw(next.as_ptr());
                    node.next = n.next;
                    return true;
                }
            }
            false
        }
    }

    /// Clears the entire queue
    pub fn clear(&mut self) {
        while self.front.is_some() {
            let _ = self.pop();
        }
    }
}

impl<T> Index<usize> for Queue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut iter = self.iter().skip(index);
        iter.next().unwrap()
    }
}

extend_singly_linked_list!(Queue<T>; Link<T>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();

        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);

        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(3);

        assert_eq!(queue.len(), 4);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(&3));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek(), Some(&3));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);
    }
}
