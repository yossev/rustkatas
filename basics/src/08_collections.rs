//! # 08 · Vec & HashMap
//!
//! - `Vec<T>`: a growable list. `push`, `len`, indexing `v[0]` (panics if
//!   out of range) or `v.get(0)` (returns an `Option`, so it never panics).
//! - `HashMap<K, V>`: key → value. The **entry API** is the idiomatic way
//!   to "insert or update":
//!
//! ```text
//! *map.entry(key).or_insert(0) += 1;
//! ```
//!
//! Use plain `for` loops here if you like. Iterators come in the next drill.
//!
//! Book: chapter 8, "Common Collections"
//!
//! Run: `cargo test -p rust-basics collections`

use std::collections::HashMap;

/// Only the even numbers, in order.
pub fn evens(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// Sorted, with duplicates removed. Hint: `sort` + `dedup`.
pub fn unique_sorted(v: &[i32]) -> Vec<i32> {
    todo!()
}

/// How often each word appears. Lower-case everything first.
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Groups words by their length, keeping their original order in each group.
pub fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    todo!()
}

#[derive(Debug, Default)]
pub struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory::default()
    }

    pub fn add(&mut self, item: &str, quantity: u32) {
        todo!()
    }

    /// Err("not enough <item>") if there isn't enough in stock.
    pub fn remove(&mut self, item: &str, quantity: u32) -> Result<(), String> {
        todo!()
    }

    /// 0 for items we've never seen.
    pub fn count(&self, item: &str) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evens_keeps_order() {
        assert_eq!(evens(&[5, 2, 8, 3, 4]), vec![2, 8, 4]);
    }

    #[test]
    fn unique_sorted_removes_duplicates() {
        assert_eq!(unique_sorted(&[3, 1, 3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn word_counts_counts_words() {
        let counts = word_counts("the cat and The hat");
        assert_eq!(counts["the"], 2);
        assert_eq!(counts["cat"], 1);
        assert_eq!(counts.get("dog"), None);
    }

    #[test]
    fn word_counts_of_empty_text() {
        todo!("YOUR TEST")
    }

    #[test]
    fn group_by_length_groups_words() {
        let groups = group_by_length(&["hi", "cat", "yo", "dog", "a"]);
        assert_eq!(groups[&2], vec!["hi", "yo"]);
        assert_eq!(groups[&3], vec!["cat", "dog"]);
        assert_eq!(groups[&1], vec!["a"]);
    }

    #[test]
    fn inventory_add_and_count() {
        let mut inv = Inventory::new();
        inv.add("apple", 3);
        inv.add("apple", 2);
        assert_eq!(inv.count("apple"), 5);
        assert_eq!(inv.count("pear"), 0);
    }

    #[test]
    fn inventory_remove() {
        let mut inv = Inventory::new();
        inv.add("apple", 3);
        assert_eq!(inv.remove("apple", 2), Ok(()));
        assert_eq!(inv.count("apple"), 1);
        assert_eq!(inv.remove("apple", 5), Err(String::from("not enough apple")));
        assert_eq!(inv.count("apple"), 1); // a failed remove changes nothing
    }

    #[test]
    fn inventory_remove_unknown_item() {
        todo!("YOUR TEST")
    }
}
