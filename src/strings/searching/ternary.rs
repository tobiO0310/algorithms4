use core::fmt;
use std::{
    cmp::{self, Ordering},
    ops::Index,
    ptr::NonNull,
};

#[allow(unused_imports)]
use crate::strings::searching::tries::Trie;
use crate::{
    SymbolTable,
    collections::{Queue, Stack, queue},
    strings::{StringSymbolTable, char_at},
};

/// This [SymbolTable] implementation is a 256-way Ternary Search Trie for string-key-based symbol tables.
/// This is often a more space saving option, however it uses logarithmic character compares.
/// For a more performant option see [Trie].
///
/// The *put*, *contains*, *delete*, and *longest prefix* operations
/// take time proportional to the length of the key (in the worst case).
#[derive(Default)]
pub struct TernarySearchTrie<V> {
    size: usize,
    root: Link<V>,
}

struct Node<V> {
    c: char,
    val: Option<V>,
    left: Link<V>,
    middle: Link<V>,
    right: Link<V>,
}
impl<V> Node<V> {
    pub fn new(c: char) -> Self {
        Self {
            c,
            val: None,
            left: None,
            middle: None,
            right: None,
        }
    }
}
type Link<V> = Option<NonNull<Node<V>>>;

/// # Safety
///
/// The link given MUST not be used again, as the link will be made Null. Call like so:
/// ```
/// # fn drop_link(x: Option<i32>) {};
/// # let mut x = Some(1);
/// drop_link(x.take());
/// ```
unsafe fn drop_link<V>(x: Link<V>) {
    if let Some(x) = x {
        // SAFETY: x's left, middle and right links will be dropped as x is dropped,
        // thereby making sure those links are no longer valid (and follow invariants)
        unsafe {
            let x = Box::from_raw(x.as_ptr());
            drop_link(x.left);
            drop_link(x.middle);
            drop_link(x.right);
        }
    }
}

fn get<V>(mut x: Link<V>, key: &str, mut d: usize) -> Link<V> {
    // keep following the links until d is max index in key
    // (if any links are None, ? will return None)
    while d < key.len() {
        // SAFETY: all Some links are valid :)
        let n = unsafe { x?.as_ref() };
        let c = char_at(key, d) as u8 as char;
        match c.cmp(&n.c) {
            Ordering::Less => x = n.left,
            Ordering::Greater => x = n.right,
            Ordering::Equal => {
                if d < key.len() - 1 {
                    x = n.middle;
                }
                d += 1;
            }
        }
    }

    x
}
fn put<V>(root: &mut Link<V>, key: &str, val: V) {
    let mut d = 0;
    let mut curr_ptr: *mut Link<V> = root;
    while d < key.len() {
        let c = char_at(key, d) as u8 as char;
        let curr_link: &mut Link<V> = unsafe { &mut *curr_ptr };

        if curr_link.is_none() {
            // SAFETY: All Some-variant links are made here, this allows for the invariant that all Some links
            // are guaranteed to be valid and NonNull pointers.
            *curr_link = unsafe {
                Some(NonNull::new_unchecked(Box::into_raw(Box::new(
                    Node::new(c),
                ))))
            }
        }

        // SAFETY: curr_link is guaranteed to be Some, since the above code would've otherwise created it.
        let n = unsafe { curr_link.as_mut().unwrap_unchecked().as_mut() };

        match c.cmp(&n.c) {
            Ordering::Less => curr_ptr = &mut n.left,
            Ordering::Greater => curr_ptr = &mut n.right,
            Ordering::Equal => {
                if d < key.len() - 1 {
                    // go down the middle and continue in the key
                    curr_ptr = &mut n.middle;
                }
                d += 1;
            }
        }
    }
    let n = unsafe { (*curr_ptr).as_mut().unwrap_unchecked().as_mut() };
    n.val = Some(val);
}

/// Returns [Err] if key does not exist following the default way of a [TernarySearchTrie]
fn delete<V>(root: &mut Link<V>, key: &str) -> Result<(), ()> {
    let mut d = 0;
    let mut stack = Stack::new();
    stack.push(root);

    while d < key.len() {
        // SAFETY: all Some links are valid :)
        // and if any become None, it returns Err
        let n = unsafe { stack.peek().unwrap().ok_or(())?.as_mut() };
        let c = char_at(key, d) as u8 as char;
        match c.cmp(&n.c) {
            Ordering::Less => stack.push(&mut n.left),
            Ordering::Greater => stack.push(&mut n.right),
            Ordering::Equal => {
                if d < key.len() - 1 {
                    stack.push(&mut n.middle);
                }
                d += 1;
            }
        }
    }

    // SAFETY: the link is valid, else Err is returned
    unsafe {
        stack.peek().unwrap().ok_or(())?.as_mut().val = None;
    }

    while let Some(l) = stack.pop() {
        let delete = {
            // SAFETY: links are guaranteed to be valid by the two ok_or(())? lines.
            let n = unsafe { l.as_mut().unwrap().as_mut() };

            n.val.is_none()
                && n.left.is_none()
                && n.middle.is_none()
                && n.right.is_none()
        };
        if delete {
            // SAFETY: to make sure invariants are upheld, this value overrides the link to be None,
            // and then automatically drops it via Box.
            let _ = unsafe { Box::from_raw(l.unwrap().as_ptr()) };
            *l = None;
        }
    }

    Ok(())
}

fn collect<V>(x: Link<V>, s: String, iter_q: &mut Queue<(String, &V)>) {
    let mut queue: Queue<(String, _)> = Queue::new();
    queue.enqueue((s, x));

    while let Some((str, x)) = queue.pop() {
        let Some(x) = x else { continue }; // make sure x is Some
        // SAFETY: this function does not delete data, and all Some links are guaranteed to be valid
        let x = unsafe { x.as_ref() };
        let mut cur = str.clone();
        cur.push(x.c);
        if let Some(val) = &x.val {
            iter_q.enqueue((cur.clone(), val));
        }
        queue.enqueue((str.clone(), x.left));
        queue.enqueue((cur, x.middle));
        queue.enqueue((str, x.right));
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

    while let Some((str, x)) = queue.pop() {
        let Some(x) = x else { continue }; // make sure x is Some
        // SAFETY: this function does not delete data, and all Some links are guaranteed to be valid
        let x = unsafe { x.as_ref() };
        let c = char_at(pat, str.len()) as u8 as char;

        match (c == b'.' as char, c.cmp(&x.c)) {
            (true, _) => {
                queue.enqueue((str.clone(), x.left));
                let mut cur = str.clone();
                cur.push(x.c);
                if cur.len() == pat.len()
                    && let Some(val) = &x.val
                {
                    iter_q.enqueue((cur.clone(), val));
                }
                if cur.len() < pat.len() {
                    queue.enqueue((cur, x.middle));
                }
                queue.enqueue((str, x.right));
            }
            (false, cmp::Ordering::Less) => {
                queue.enqueue((str, x.left));
            }
            (false, cmp::Ordering::Equal) => {
                let mut cur = str.clone();
                cur.push(x.c);
                if cur.len() == pat.len()
                    && let Some(val) = &x.val
                {
                    iter_q.enqueue((cur.clone(), val));
                }
                if cur.len() < pat.len() {
                    queue.enqueue((cur, x.middle));
                }
            }
            (false, cmp::Ordering::Greater) => {
                queue.enqueue((str, x.right));
            }
        }
    }
}

impl<V> TernarySearchTrie<V> {
    /// Instantiates a new empty Ternary Search Trie.
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    /// Returns an iterator over this
    pub fn iter(&self) -> queue::IntoIter<(String, &V)> {
        let mut queue = Queue::new();
        collect(self.root, "".into(), &mut queue);
        queue.into_iter()
    }
}
impl<V> StringSymbolTable<V> for TernarySearchTrie<V> {
    fn longest_prefix_of(&self, s: &str) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let mut len = 0;
        let mut x = self.root;
        let mut i = 0;
        while let Some(n) = x
            && i < s.len()
        {
            // SAFETY: Since n is Some, the link is guaranteed to be valid.
            let n = unsafe { n.as_ref() };
            let c = char_at(s, i) as u8 as char;
            if c < n.c {
                x = n.left;
            } else if c > n.c {
                x = n.right;
            } else {
                i += 1;
                if n.val.is_some() {
                    len = i;
                }
                x = n.middle;
            }
        }

        s[0..len].to_string()
    }

    fn entries_with_prefix<'a>(
        &'a self,
        s: &str,
    ) -> impl Iterator<Item = (String, &'a V)>
    where
        V: 'a,
    {
        let mut iter_q = Queue::new();

        let x = match get(self.root, s, 0) {
            Some(v) => Some(v),
            None => return Queue::new().into_iter(),
        };
        collect(x, s.to_string(), &mut iter_q);
        iter_q.into_iter()
    }

    fn entries_that_match<'a>(
        &'a self,
        s: &str,
    ) -> impl Iterator<Item = (String, &'a V)>
    where
        V: 'a,
    {
        let mut iter_q = Queue::new();
        collect_pat(self.root, "".to_string(), s, &mut iter_q);
        iter_q.into_iter()
    }
}
impl<V> SymbolTable<String, V> for TernarySearchTrie<V> {
    fn put(&mut self, key: String, value: V) {
        if key.is_empty() {
            panic!("key must have length >= 1");
        }
        put(&mut self.root, &key, value);
        self.size += 1;
    }

    fn get(&self, key: &String) -> Option<&V> {
        if key.is_empty() {
            panic!("key must have length >= 1");
        }
        // SAFETY: the link is valid as long as it's Some (guaranteed by try operator)
        unsafe { get(self.root, key, 0)?.as_ref().val.as_ref() }
    }

    fn delete(&mut self, key: &String)
    where
        String: Clone,
        V: Clone,
    {
        if self.root.is_none() || !self.contains(key) {
            // do not try to delete, if key doesn't exist
            return;
        }
        // SAFETY: delete only returns an error if the root does not contain key.
        delete(&mut self.root, key).unwrap();
        self.size -= 1;
    }

    fn clear(&mut self) {
        // SAFETY: The root (link) is made None, to guarantee the invariant
        unsafe { drop_link(self.root.take()) };
        self.size = 0;
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<V> IntoIterator for TernarySearchTrie<V> {
    type Item = (String, V);
    type IntoIter = queue::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut iter_q = Queue::new();
        let mut queue: Queue<(String, _)> = Queue::new();
        // SAFETY: make sure to remove root link, so Some links are continued to be valid
        queue.enqueue(("".into(), self.root.take()));

        while let Some((str, Some(x))) = queue.pop() {
            // SAFETY: this function does not delete data, and all Some links are guaranteed to be valid
            let mut x = unsafe { Box::from_raw(x.as_ptr()) };
            let c = x.c.to_string();
            let cur = str.clone() + &c;
            if let Some(val) = x.val {
                iter_q.enqueue((cur.clone(), val));
            }
            queue.enqueue((str.clone(), x.left.take()));
            queue.enqueue((cur, x.middle.take()));
            queue.enqueue((str, x.right.take()));
        }

        iter_q.into_iter()
    }
}
impl<'a, V> IntoIterator for &'a TernarySearchTrie<V> {
    type Item = (String, &'a V);

    type IntoIter = queue::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, V> Index<&'a String> for TernarySearchTrie<V> {
    type Output = V;

    fn index(&self, index: &'a String) -> &Self::Output {
        self.get(index).expect("Key not in TernarySearchTrie")
    }
}
impl<'a, V> Index<&'a str> for TernarySearchTrie<V> {
    type Output = V;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.get(&index.into())
            .expect("Key not in TernarySearchTrie")
    }
}

impl<V> Drop for TernarySearchTrie<V> {
    fn drop(&mut self) {
        // SAFETY: called correctly
        unsafe { drop_link(self.root.take()) };
    }
}
impl<V: Clone> Clone for TernarySearchTrie<V> {
    fn clone(&self) -> Self {
        let mut new_bst = Self::new();
        for (key, value) in self {
            new_bst.put(key.clone(), value.clone());
        }
        new_bst
    }
}
impl<V> Extend<(String, V)> for TernarySearchTrie<V> {
    fn extend<I: IntoIterator<Item = (String, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.put(key, value);
        }
    }
}
impl<V> FromIterator<(String, V)> for TernarySearchTrie<V> {
    fn from_iter<I: IntoIterator<Item = (String, V)>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}
impl<V: fmt::Debug> fmt::Debug for TernarySearchTrie<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

super::test_string_table!(TernarySearchTrie);
