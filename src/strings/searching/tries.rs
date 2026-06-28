use std::{fmt, marker::PhantomData, ops::Index, ptr::NonNull};

#[allow(unused_imports)]
use crate::strings::searching::ternary::TernarySearchTrie;
use crate::{
    SymbolTable,
    collections::{Queue, queue},
    strings::{ASCII_SIZE, searching::StringSymbolTable},
};

const R: usize = ASCII_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node<V> {
    val: Option<V>,
    next: [Link<V>; R],
}

impl<V> Node<V> {
    fn new() -> NonNull<Self> {
        // SAFETY: seeing as nodes are only created here, they are all guaranteed to
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                val: None,
                next: [None; R],
            })))
        }
    }
}

type Link<V> = Option<NonNull<Node<V>>>;
fn drop_link<V>(x: Link<V>) {
    if let Some(x) = x {
        let x = unsafe { Box::from_raw(x.as_ptr()) };
        for l in x.next {
            drop_link(l);
        }
    }
}

fn char_at(s: &str, d: usize) -> i16 {
    if d >= s.len() {
        -1
    } else {
        s.as_bytes()[d] as i16
    }
}

fn get<V>(x: Link<V>, key: &str, d: usize) -> Link<V> {
    // SAFETY: all links are valid, as long as they're Some (as they are ONLY created in Node::new)
    let node = unsafe { x?.as_ref() };
    if d == key.len() {
        x
    } else {
        get(node.next[char_at(key, d) as usize], key, d + 1)
    }
}

fn collect<V>(x: Link<V>, s: String, iter_q: &mut Queue<(String, &V)>) {
    let mut queue: Queue<(String, _)> = Queue::new();
    queue.enqueue((s, x));

    while let Some((str, Some(x))) = queue.pop() {
        // SAFETY: this function does not delete data, and all Some links are guaranteed to be valid
        let x = unsafe { x.as_ref() };
        if let Some(val) = &x.val {
            iter_q.enqueue((str.clone(), val));
        }
        for c in 0..R {
            if x.next[c].is_some() {
                queue.enqueue((
                    str.clone()
                        + &String::from_utf8(vec![c as u8])
                            .expect("c is a valid u8"),
                    x.next[c],
                ));
            }
        }
    }
}
fn collect_pat<V>(
    x: Link<V>,
    s: String,
    pat: &str,
    iter_q: &mut Queue<(String, &V)>,
) {
    let mut queue: Queue<(String, _)> = Queue::new();
    queue.enqueue((s, x));

    while let Some((str, Some(x))) = queue.pop() {
        // SAFETY: this function does not delete data, and all Some links are guaranteed to be valid
        let x = unsafe { x.as_ref() };
        if str.len() == pat.len()
            && let Some(val) = &x.val
        {
            iter_q.enqueue((str.clone(), val));
        }
        if str.len() == pat.len() {
            continue;
        }

        let next = char_at(pat, str.len()) as usize;
        for c in 0..R {
            if (next == b"."[0] as usize || next == c) && x.next[c].is_some() {
                queue.enqueue((
                    str.clone()
                        + &String::from_utf8(vec![c as u8])
                            .expect("c is a valid u8"),
                    x.next[c],
                ));
            }
        }
    }
}

fn search<V>(x: Link<V>, s: String, d: usize, mut len: usize) -> usize {
    if let Some(x) = x {
        // SAFETY: all links are valid, as long as they're Some (as they are ONLY created in put)
        let x = unsafe { x.as_ref() };
        if x.val.is_some() {
            len = d;
        }
        if d == s.len() {
            len
        } else {
            let c = char_at(&s, d) as usize;
            search(x.next[c], s, d + 1, len)
        }
    } else {
        len
    }
}

/// # Safety
///
/// Must be called like this:
///
/// ```
/// # fn put(a:u8, b:u8, c:u8, d:u8) -> u8 {// bare-bone example
/// # a
/// # }
/// # let (mut x, key, val, d) = (0, 0, 0, 0);
/// x = put(x, key, val, d);
/// ```
unsafe fn put<V>(mut x: Link<V>, key: String, val: V, d: usize) -> Link<V> {
    if x.is_none() {
        x = Some(Node::new());
    }

    if d == key.len() {
        unsafe {
            x?.as_mut().val = Some(val);
        }
        return x;
    }

    // SAFETY: all nodes are guaranteed to be valid, when the link is Some
    unsafe {
        let c = char_at(&key, d) as usize;

        let x = x?.as_mut();
        let next = x.next[c];
        x.next[c] = put(next, key, val, d + 1);
    }

    x
}

/// # Safety
///
/// Must be called like this:
///
/// ```
/// # fn delete(a:u8, b:u8, c:u8) -> u8 {// bare-bone example
/// # a
/// # }
/// # let (mut x, key, d) = (0, 0, 0);
/// x = delete(x, key, d);
/// ```
unsafe fn delete<V>(x: Link<V>, key: &String, d: usize) -> Link<V> {
    let n = unsafe { x?.as_mut() };

    // recursively go down until you find the key and then delete it's value
    if d == key.len() {
        n.val = None;
    } else {
        // if not at key yet, continue down
        let c = char_at(key, d) as usize;
        // SAFETY: called correctly, therefore guaranteeing invariants
        n.next[c] = unsafe { delete(n.next[c], key, d + 1) };
    }
    // after deleting the key & val, if this is something always return it
    if n.val.is_some() {
        return x;
    }
    // else if this value HAS a child (aka there exists a path to some other key)
    for c in 0..R {
        if n.next[c].is_some() {
            return x;
        }
    }

    // SAFETY: the following code will automatically drop x,
    // to maintain invariants (when links are valid and actually NonNull), the return type must override the previous link
    // (as stated in function documentation)
    unsafe {
        let _ = Box::from_raw(x?.as_ptr());
    }

    // return None, so links (and invariants) are not broken
    None
}

/// This [SymbolTable] implementation is a 256-way Trie for string-key-based symbol tables.
/// This is often a more performant option, however it does require a lot more memory
/// to store all the 256 sized arrays. Should memory be a limit, checkout a [TernarySearchTrie].
///
/// The *put*, *contains*, *delete*, and *longest prefix* operations
/// take time proportional to the length of the key (in the worst case).
#[derive(Default)]
pub struct Trie<V> {
    root: Link<V>,
    size: usize,
    _data: PhantomData<V>,
}

impl<V> Trie<V> {
    /// Instantiates a new empty Trie.
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
            _data: PhantomData,
        }
    }

    /// Returns an iterator over this
    #[must_use]
    pub fn iter(&self) -> queue::IntoIter<(String, &V)> {
        let mut q = Queue::new();
        collect(get(self.root, "", 0), "".into(), &mut q);
        q.into_iter()
    }
}

impl<V> StringSymbolTable<V> for Trie<V> {
    fn longest_prefix_of(&self, s: &str) -> String {
        let len = search(self.root, s.into(), 0, 0);
        s[0..len].into()
    }

    fn entries_with_prefix<'a>(
        &'a self,
        s: &str,
    ) -> impl Iterator<Item = (String, &'a V)>
    where
        V: 'a,
    {
        let mut q = Queue::new();
        collect(get(self.root, s, 0), s.into(), &mut q);
        q.into_iter()
    }

    fn entries_that_match<'a>(
        &'a self,
        s: &str,
    ) -> impl Iterator<Item = (String, &'a V)>
    where
        V: 'a,
    {
        let mut q = Queue::new();
        collect_pat(self.root, "".into(), s, &mut q);
        q.into_iter()
    }
}
impl<V> SymbolTable<String, V> for Trie<V> {
    fn put(&mut self, key: String, value: V) {
        unsafe { self.root = put(self.root, key, value, 0) }
        self.size += 1;
    }

    fn get(&self, key: &String) -> Option<&V> {
        // SAFETY: all links are valid, as long as they're Some (as they are ONLY created in put)
        unsafe { get(self.root, key, 0)?.as_ref().val.as_ref() }
    }

    fn delete(&mut self, key: &String)
    where
        String: Clone,
        V: Clone,
    {
        if !self.contains(key) {
            // don't delete, if this does not contain the key
            return;
        }
        // SAFETY: called correctly :)
        self.root = unsafe { delete(self.root, key, 0) };
        self.size -= 1;
    }

    fn clear(&mut self) {
        drop_link(self.root.take());
        self.size = 0;
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<V> IntoIterator for Trie<V> {
    type Item = (String, V);

    type IntoIter = queue::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut queue: Queue<(String, _)> = Queue::new();
        queue.enqueue(("".into(), self.root.take()));

        let mut iter_q: Queue<Self::Item> = Queue::new();
        while let Some((str, Some(x))) = queue.pop() {
            // SAFETY: this function makes sure to delete ALL links through a queue (instead of recursive)
            // this implementation is a lot like deleting BFS-style of the rooted tree
            let x = unsafe { Box::from_raw(x.as_ptr()) };
            if let Some(val) = x.val {
                iter_q.enqueue((str.clone(), val));
            }
            for c in 0..R {
                if x.next[c].is_some() {
                    queue.enqueue((
                        str.clone()
                            + &String::from_utf8(vec![c as u8])
                                .expect("c is a valid u8"),
                        x.next[c],
                    ));
                }
            }
        }

        iter_q.into_iter()
    }
}
impl<'a, V> IntoIterator for &'a Trie<V> {
    type Item = (String, &'a V);

    type IntoIter = queue::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, V> Index<&'a String> for Trie<V> {
    type Output = V;

    fn index(&self, index: &'a String) -> &Self::Output {
        self.get(index).expect("Key not in Trie")
    }
}
impl<'a, V> Index<&'a str> for Trie<V> {
    type Output = V;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.get(&index.into()).expect("Key not in Trie")
    }
}

impl<V> Drop for Trie<V> {
    fn drop(&mut self) {
        drop_link(self.root.take());
    }
}
impl<V: Clone> Clone for Trie<V> {
    fn clone(&self) -> Self {
        let mut new_bst = Self::new();
        for (key, value) in self {
            new_bst.put(key.clone(), value.clone());
        }
        new_bst
    }
}
impl<V> Extend<(String, V)> for Trie<V> {
    fn extend<I: IntoIterator<Item = (String, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.put(key, value);
        }
    }
}
impl<V> FromIterator<(String, V)> for Trie<V> {
    fn from_iter<I: IntoIterator<Item = (String, V)>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}
impl<V: PartialEq> PartialEq for Trie<V> {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.iter().eq(other)
    }
}
impl<V: Eq> Eq for Trie<V> {}
impl<V: fmt::Debug> fmt::Debug for Trie<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec(size: usize) -> Vec<String> {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(i.to_string());
        }
        vec
    }

    #[test]
    fn it_works() {
        let mut trie1 = Trie::new();
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

        let trie2 = trie1.clone();
        assert_eq!(trie1, trie2);
        let mut trie3 = trie2.into_iter().collect();
        assert_eq!(trie1, trie3);

        trie3.clear();
        assert_eq!(trie3.size, 0);
        assert!(trie3.is_empty());

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
        let mut trie = Trie::new();
        const SIZE: usize = 100;

        assert_eq!(trie.size, 0);
        assert!(trie.is_empty());

        let vec = vec(SIZE);
        for (i, str) in vec.iter().enumerate() {
            trie.put(str.clone(), str.clone());
            assert_eq!(trie.size, i + 1);
        }

        assert_eq!(trie.entries_that_match("").collect::<Vec<_>>().len(), 0);
        assert_eq!(trie.entries_that_match(".").collect::<Vec<_>>().len(), 10);
        assert_eq!(trie.entries_that_match("..").collect::<Vec<_>>().len(), 90);
        for i in 0..SIZE {
            assert_eq!(
                trie.entries_that_match(&i.to_string())
                    .collect::<Vec<_>>()
                    .len(),
                1
            );
        }
    }

    #[test]
    fn prefixes_work() {
        let mut trie = Trie::new();
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
        for i in 1..=9 {
            assert_eq!(
                trie.entries_with_prefix(&i.to_string())
                    .collect::<Vec<_>>()
                    .len(),
                11
            );
        }
    }

    #[test]
    fn longest_prefix() {
        let mut trie = Trie::new();
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

        assert_eq!(trie.longest_prefix_of("166"), "16".to_string());
        assert_eq!(trie.longest_prefix_of("248"), "24".to_string());
        assert_eq!(trie.longest_prefix_of("325"), "32".to_string());
        assert_eq!(trie.longest_prefix_of("487"), "48".to_string());
        assert_eq!(trie.longest_prefix_of("534"), "53".to_string());
        assert_eq!(trie.longest_prefix_of("611"), "61".to_string());
        assert_eq!(trie.longest_prefix_of("752"), "75".to_string());
        assert_eq!(trie.longest_prefix_of("873"), "87".to_string());
        assert_eq!(trie.longest_prefix_of("999"), "99".to_string());
        
        assert_eq!(trie.longest_prefix_of("a"), "".to_string());
    }    
}
