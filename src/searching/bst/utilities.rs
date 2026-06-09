//! Utitlies for BST
//!
//! # Safety
//!
//! All getters are safe functions, as they do NOT mutate any memory.
//! However, the others may mutate memory, and therefore to uphold invariants they are marked unsafe,
//! and a notice on how to safely use them are given.
//!

use super::*;

/// Makes a right-leaning red link left-leaning. It returns a new link to `raw_h.right`
///
/// It must be called like so
/// ```
/// # let mut raw_h = true;
/// # fn rotate_left(x: bool) -> bool { x }
/// raw_h = rotate_left(raw_h)
/// ```
///
/// # Safety
///
/// Must be called with a valid `raw_h` which also has a valid [Color::Red] right [Node]
#[must_use]
unsafe fn rotate_left<K, V>(raw_h: Link<K, V>) -> Link<K, V> {
    debug_assert!(raw_h.is_some());
    debug_assert!(Node::is_red(unsafe { raw_h.unwrap().as_ref().right }));

    // SAFETY: raw_h is gurantted to be initialized, since it is Some else propagates
    let h = unsafe { raw_h.unwrap().as_mut() };
    let raw_x = h.right;
    // SAFETY: raw_x must always be there (and therefore initialized), else rotate_left was called with a wrong input
    let x = unsafe { raw_x.unwrap().as_mut() };
    h.right = x.left;
    x.left = raw_h;

    x.color = h.color;
    h.color = Color::Red;

    x.size = h.size;
    h.size = 1 + Node::size(h.left) + Node::size(h.right);

    raw_x
}

/// Makes a left-leaning red link right-leaning. It returns a new link to `raw_h.left`
///
/// It must be called like so
/// ```
/// # let mut raw_h = true;
/// # fn rotate_right(x: bool) -> bool { x }
/// raw_h = rotate_right(raw_h)
/// ```
///
/// # Safety
///
/// Must be called with a valid `raw_h` which also has a valid [Color::Red] left [Node]
#[must_use]
unsafe fn rotate_right<K, V>(raw_h: Link<K, V>) -> Link<K, V> {
    debug_assert!(raw_h.is_some());
    debug_assert!(Node::is_red(unsafe { raw_h?.as_ref().left }));

    // SAFETY: raw_h is gurantted to be initialized, since it is Some else propagates
    let h = unsafe { raw_h?.as_mut() };
    let raw_x = h.left;
    // SAFETY: raw_x must always be there (and therefore initialized), else rotate_left was called with a wrong input
    let x = unsafe { raw_x?.as_mut() };
    h.left = x.right;
    x.right = raw_h;

    x.color = h.color;
    h.color = Color::Red;

    x.size = h.size;
    h.size = 1 + Node::size(h.left) + Node::size(h.right);

    raw_x
}

/// Flips the color of the parent with its children.
///
/// The parent must have opposite color of both its children.
///
/// # Safety
///
/// Must be called with a valid `raw_h` which also has a valid left [Node] and valid right [Node].
unsafe fn flip_colors<K, V>(raw_h: Link<K, V>) {
    // SAFETY: it is assumed raw_h is valid and has valid children
    unsafe {
        let h = raw_h.unwrap().as_mut();
        let left = h.left.unwrap().as_mut();
        let right = h.right.unwrap().as_mut();
        debug_assert!(if h.color == Color::Red {
            left.color == Color::Black && right.color == Color::Black
        } else {
            left.color == Color::Red && right.color == Color::Red
        });
        h.color.switch();
        right.color.switch();
        left.color.switch();
    }
}

/// Make `h.left` or one of its children red. It returns a new link to set as previous `h`
///
/// It must be called like so
/// ```
/// # let mut raw_h = true;
/// # fn move_red_left(x: bool) -> bool { x }
/// raw_h = move_red_left(raw_h)
/// ```
///
/// # Safety
///
/// Must be called with a valid [Color::Red] `raw_h`,
/// valid [Color::Black] `raw_h.left` and valid [Color::Black] `raw_h.left.left`
#[must_use]
unsafe fn move_red_left<K, V>(mut raw_h: Link<K, V>) -> Link<K, V> {
    // SAFETY: Everything inside handles with raw pointers,
    // however they are assumed to be valid on call
    unsafe {
        flip_colors(raw_h);
        let h = raw_h.unwrap().as_mut();
        let right_left_is_red = if let Some(r) = h.right {
            Node::is_red(r.as_ref().left)
        } else {
            false
        };
        if right_left_is_red {
            h.right = rotate_right(h.right);
            raw_h = rotate_left(raw_h);
            flip_colors(raw_h);
        }
    }
    raw_h
}

/// Make `h.right` or one of its children red. It returns a new link to set as previous `h`
///
/// It must be called like so
/// ```
/// # let mut raw_h = true;
/// # fn move_red_right(x: bool) -> bool { x }
/// raw_h = move_red_right(raw_h)
/// ```
///
/// # Safety
///
/// Must be called with a valid [Color::Red] `raw_h`,
/// valid [Color::Black] `raw_h.right` and valid [Color::Black] `raw_h.right.left`
#[must_use]
unsafe fn move_red_right<K, V>(mut raw_h: Link<K, V>) -> Link<K, V> {
    // SAFETY: Everything inside handles with raw pointers,
    // however they are assumed to be valid on call
    unsafe {
        flip_colors(raw_h);
        let h = raw_h.unwrap().as_mut();
        let left_left_is_red = if let Some(l) = h.left {
            Node::is_red(l.as_ref().left)
        } else {
            false
        };
        if left_left_is_red {
            raw_h = rotate_right(raw_h);
            flip_colors(raw_h);
        }
    }
    raw_h
}

/// Restore Red-Black tree invariants
///
/// # Safety
///
/// Must be called with a valid [Color::Red] `raw_h`
#[must_use]
unsafe fn balance<K, V>(mut raw_h: Link<K, V>) -> Link<K, V> {
    // SAFETY: a lot of raw pointer dereferencing, therefore to keep it simple, everything is in an unsafe block
    unsafe {
        let node = raw_h?.as_ref();
        if Node::is_red(node.right) && !Node::is_red(node.left) {
            // make right-leaning links left-leaning
            // SAFETY: called correctly
            raw_h = rotate_left(raw_h);
        }
        let node = raw_h?.as_ref();
        if Node::is_red(node.left)
            && Node::is_red(node.left.unwrap().as_ref().left)
        {
            // rotate right, if two red links in a row on the left sides
            // this allows for flipping the colors (next check)
            // SAFETY: called correctly
            raw_h = rotate_right(raw_h);
        }
        let node = raw_h?.as_ref();
        if Node::is_red(node.left) && Node::is_red(node.right) {
            // if both left and right are red, flip the color up, so the parent becomes red instead
            flip_colors(raw_h);
        }

        // update the size to correctly keep track of total size of the tree
        let node = raw_h?.as_mut();
        node.size = Node::size(node.left) + Node::size(node.right) + 1;

        raw_h
    }
}

/// Finds and inserts the key and value, overriding if key is already present.
///
/// # Safety
///
/// This must be called with complete valid links, and the result must be set as follows
///
/// ```
/// # let mut raw_h = true;
/// # let key = false;
/// # let value = false;
/// # fn put(x: bool, key: bool, value: bool) -> bool { x }
/// raw_h = put(raw_h, key, value);
/// ```
///
/// Doing so gurantees the invariant that all NonNulls are initialized, unless that invairant was already broken.
#[must_use]
pub unsafe fn put<K: Ord, V>(
    raw_h: Link<K, V>,
    key: K,
    value: V,
) -> Link<K, V> {
    if let Some(mut n) = raw_h {
        // SAFETY: since raw_h is Some, it must exist,
        // as all created NonNulls are created in the other part of this if-statement.
        // Furthermore, seeing as balance(raw_h) is returned, that part is called correctly and invariants are restored.
        unsafe {
            let node = n.as_mut();
            match key.cmp(&node.key) {
                Ordering::Less => node.left = put(node.left, key, value),
                Ordering::Greater => node.right = put(node.right, key, value),
                Ordering::Equal => node.value = value,
            }

            balance(raw_h)
        }
    } else {
        // SAFETY: create an initialized node, box it and pack the raw pointer into a NonNull.
        // This allows for the use of Option :)
        // Furthermore, since all nodes are ONLY created here, it gurantees all NonNulls to be initialized,
        // until they're deleted, in which case that function must gurantee that invariant.
        unsafe {
            Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(
                key, value,
            )))))
        }
    }
}

/// Finds and deletes the minimum key, and its associated value, by following left links recursively.
///
/// # Safety
///
/// This must be called with complete valid links,
/// `raw_h`, or one of its children, must be a [Color::Red] link, and the result must be set as follows
///
/// ```
/// # let mut raw_h = true;
/// # fn delete_min(x: bool) -> bool { x }
/// raw_h = delete_min(raw_h);
/// ```
///
/// Doing so gurantees the invariant that all NonNulls are initialized, unless that invairant was already broken.
#[must_use]
pub unsafe fn delete_min<K: Ord, V>(mut raw_h: Link<K, V>) -> Link<K, V> {
    // SAFETY: handles raw pointers, that are assumed to be valid
    unsafe {
        let node = raw_h?.as_mut();

        if node.left.is_none() {
            // SAFETY: if node is the minimum (has no more on left), it is set to be deleted.
            // To uphold the invariants required for proper use, the following requirements must be fulfilled:
            //
            // 1. If a right link exist it must be given to the parent of node. This *SHOULD* always be None to uphold Red-Black invariants.
            // 2. This should always be a red link, and that is guranteed as long as the start call to delete_min is given a red link.
            // Because then move_red_left continously moves the link red down as this function is recursively called.
            // 3. The value must be dropped appropriately to not leak any memory. To do this,
            // the raw pointer is returned to a Box and implicitly dropped.
            //
            // As this is the ONLY place any value is dropped, it is guranteed to always uphold all invariants, if `raw_h` is always a red link.
            let _ = Box::from_raw(raw_h.unwrap().as_ptr());
            return None;
        }

        // unwrap can be used, as node.left is guranteed to be some here (else None is returned by above if-statement)
        if !Node::is_red(node.left)
            && !Node::is_red(node.left.unwrap().as_ref().left)
        {
            // keep the red link with us on the way down to keep uphold the requirements in previous if-statement
            raw_h = move_red_left(raw_h);
        }

        let node = raw_h?.as_mut();

        node.left = delete_min(node.left);
        balance(raw_h)
    }
}

/// Finds and deletes the key, and its associated value.
///
/// # Safety
///
/// This must be called with complete valid links,
/// `raw_h`, or one of its children, must be a [Color::Red] link, and the result must be set as follows
///
/// ```
/// # let mut raw_h = true;
/// # let key = false;
/// # fn delete(x: bool, key: &bool) -> bool { x }
/// raw_h = delete(raw_h, &key);
/// ```
///
/// Doing so gurantees the invariant that all NonNulls are initialized, unless that invairant was already broken.
#[must_use]
pub unsafe fn delete<K: Clone + Ord, V: Clone>(
    mut raw_h: Link<K, V>,
    key: &K,
) -> Link<K, V> {
    // SAFETY: handling raw pointers
    unsafe {
        let node = raw_h?.as_ref();
        match key.cmp(&node.key) {
            Ordering::Less => {
                if !Node::is_red(node.left)
                    && !Node::is_red(node.left.unwrap().as_ref().left)
                {
                    // move red down our path to the left :))
                    raw_h = move_red_left(raw_h);
                }
                let node = raw_h?.as_mut();
                node.left = delete(node.left, key);
            }
            _ => {
                if Node::is_red(node.left) {
                    // if left is already red, we will make this link right leaning to allow moving right down the sub-tree
                    raw_h = rotate_right(raw_h);
                }
                let node = raw_h?.as_ref();

                if key.cmp(&node.key).is_eq() && node.right.is_none() {
                    let _ = Box::from_raw(raw_h.unwrap().as_ptr());
                    return None;
                }

                if !Node::is_red(node.right)
                    && !Node::is_red(node.right.unwrap().as_ref().left)
                {
                    // keep moving red along our path down
                    raw_h = move_red_right(raw_h);
                }
                let node = raw_h?.as_mut();
                if key.cmp(&node.key).is_eq() {
                    // copies the minimum key and its value on the right side to this node (so this node is still lower than the others)
                    let min_ptr = min(node.right).unwrap().as_ptr();
                    let min = &*min_ptr;

                    node.key = min.key.clone();
                    node.value = min.value.clone();

                    // then deletes this node, only via delete_min, so invariants are upheld.
                    node.right = delete_min(node.right);
                } else {
                    node.right = delete(node.right, key);
                }
            }
        }
        balance(raw_h)
    }
}

/// Finds the minimum node in this sub-tree, follows left links non-recursively.
///
/// # Safety
///
/// Must uphold RedBlack invariants and that Links that are Some must be initialized.
pub fn min<K: Ord, V>(mut current: Link<K, V>) -> Link<K, V> {
    while current.is_some() {
        // SAFETY: nodes are guranteed to be initialized,
        // as this requires current to be Some (and therefore initialized)
        let node = unsafe { current.unwrap().as_ref() };
        if node.left.is_none() {
            return current;
        } else {
            current = node.left;
        }
    }
    None
}

/// Recursively tries to find the highest key <= `key`.
///
/// # Safety
///
/// Must uphold RedBlack invariants and that Links that are Some must be initialized.
#[must_use]
pub fn floor<K: Ord, V>(x: Link<K, V>, key: &K) -> Link<K, V> {
    // SAFETY: nodes are guranteed to be initialized, as x would propagate None if x.is_none()
    let node = unsafe { x?.as_ref() };
    match key.cmp(&node.key) {
        Ordering::Equal => x,
        Ordering::Less => floor(node.left, key),
        Ordering::Greater => floor(node.right, key).or(x),
    }
}

/// Recursively tries to find the lowest key >= `key`.
///
/// # Safety
///
/// Must uphold RedBlack invariants and that Links that are Some must be initialized.
#[must_use]
pub fn ceiling<K: Ord, V>(x: Link<K, V>, key: &K) -> Link<K, V> {
    // SAFETY: nodes are guranteed to be initialized, as x would propagate None if x.is_none()
    let node = unsafe { x?.as_ref() };
    match key.cmp(&node.key) {
        Ordering::Equal => x,
        Ordering::Greater => ceiling(node.right, key),
        Ordering::Less => ceiling(node.left, key).or(x),
    }
}

/// Returns the amount of nodes < `key`.
///
/// # Safety
///
/// Must uphold RedBlack invariants and that Links that are Some must be initialized.
#[must_use]
pub fn rank<K: Ord, V>(x: Link<K, V>, key: &K) -> usize {
    if let Some(n) = x {
        // SAFETY: nodes are guranteed to be initialized, as n is not None
        let node = unsafe { n.as_ref() };
        match key.cmp(&node.key) {
            Ordering::Less => rank(node.left, key),
            Ordering::Greater => {
                1 + Node::size(node.left) + rank(node.right, key)
            }
            Ordering::Equal => Node::size(node.left),
        }
    } else {
        0
    }
}
