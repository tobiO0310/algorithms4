//! This module holds the most basic collections as described in chapter 1.3 of the book.

pub mod bag;
pub mod queue;
pub mod stack;

pub use bag::Bag;
pub use stack::Stack;

macro_rules! extend_singly_linked_list {
    ($t:ty; $l:ty) => {
        /// An iterator
        pub struct Iter<'a, T> {
            curr: $l,
            len: usize,
            _boo: PhantomData<&'a T>,
        }

        /// An iterator with mutable references
        pub struct IterMut<'a, T> {
            curr: $l,
            len: usize,
            _boo: PhantomData<&'a T>,
        }

        /// The iterator of the consumed item
        pub struct IntoIter<T> {
            item: $t,
        }

        impl<T> Drop for $t {
            fn drop(&mut self) {
                while let Some(_) = self.pop() {}
            }
        }
        impl<T: Clone> Clone for $t {
            fn clone(&self) -> Self {
                let mut new_list = Self::new();
                for item in self {
                    new_list.push(item.clone());
                }
                new_list
            }
        }
        impl<T> Extend<T> for $t {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                for item in iter {
                    self.push(item);
                }
            }
        }
        impl<T> FromIterator<T> for $t {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut list = Self::new();
                list.extend(iter);
                list
            }
        }
        impl<T: Debug> Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self).finish()
            }
        }
        impl<T: PartialEq> PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.len() == other.len() && self.iter().eq(other)
            }
        }
        impl<T: Eq> Eq for $t {}
        impl<T: PartialOrd> PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.iter().partial_cmp(other)
            }
        }
        impl<T: Ord> Ord for $t {
            fn cmp(&self, other: &Self) -> Ordering {
                self.iter().cmp(other)
            }
        }
        impl<T: Hash> Hash for $t {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.len().hash(state);
                for item in self {
                    item.hash(state);
                }
            }
        }

        impl<T> Iterator for IntoIter<T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                self.item.pop()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.item.len, Some(self.item.len))
            }
        }
        impl<T> ExactSizeIterator for IntoIter<T> {
            fn len(&self) -> usize {
                self.item.len
            }
        }
        impl<'a, T> Iterator for Iter<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.len > 0 {
                    self.curr.map(|node| unsafe {
                        self.len -= 1;
                        self.curr = (*node.as_ptr()).next;
                        &(*node.as_ptr()).elem
                    })
                } else {
                    None
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.len, Some(self.len))
            }
        }
        impl<'a, T> ExactSizeIterator for Iter<'a, T> {
            fn len(&self) -> usize {
                self.len
            }
        }
        impl<'a, T> Iterator for IterMut<'a, T> {
            type Item = &'a mut T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.len > 0 {
                    self.curr.map(|node| unsafe {
                        self.len -= 1;
                        self.curr = (*node.as_ptr()).next;
                        &mut (*node.as_ptr()).elem
                    })
                } else {
                    None
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.len, Some(self.len))
            }
        }
        impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
            fn len(&self) -> usize {
                self.len
            }
        }

        impl<T> IntoIterator for $t {
            type Item = T;
            type IntoIter = IntoIter<T>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter { item: self }
            }
        }
        impl<'a, T> IntoIterator for &'a $t {
            type Item = &'a T;
            type IntoIter = Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl<'a, T> IntoIterator for &'a mut $t {
            type Item = &'a mut T;
            type IntoIter = IterMut<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }
    };
}
use extend_singly_linked_list;
