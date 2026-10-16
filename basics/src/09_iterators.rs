//! # 09 · Iterators & closures
//!
//! **Rule for this drill: no `for` loops, no `let mut`.** Only iterator chains.
//!
//! - `.iter()` borrows each element, `.into_iter()` takes ownership of them.
//! - Adapters (lazy, they do nothing until consumed): `map`, `filter`,
//!   `enumerate`, `rev`, `take`, `skip`, `zip`.
//! - Consumers (actually run the chain): `collect`, `sum`, `count`, `max`,
//!   `min_by_key`, `find`, `any`, `all`, `fold`.
//! - A **closure** is an anonymous function: `|x| x * 2`. It can capture
//!   variables from around it. `move` makes it take ownership of them.
//!
//! Book: chapter 13, "Iterators and Closures"
//!
//! Run: `cargo test -p rust-basics iterators`

/// Square every even number, drop the odd ones.
pub fn squares_of_evens(v: &[i32]) -> Vec<i32> {
    todo!()
}

pub fn sum_of_odds(v: &[i32]) -> i32 {
    todo!()
}

/// How many words are longer than `n` characters.
pub fn count_longer_than(words: &[&str], n: usize) -> usize {
    todo!()
}

/// The longest word. On a tie, the **first** one wins.
/// Careful: `max_by_key` returns the *last* maximum on a tie. Write a test
/// that catches that, then find a way around it.
pub fn longest_word(text: &str) -> Option<&str> {
    todo!()
}

/// ["apple", "pear"] -> ["1. apple", "2. pear"]
pub fn numbered(items: &[&str]) -> Vec<String> {
    todo!()
}

/// Calls `f` twice: f(f(x)).
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!()
}

/// Returns a closure that adds `n` to whatever it gets.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_of_evens_only() {
        assert_eq!(squares_of_evens(&[1, 2, 3, 4]), vec![4, 16]);
    }

    #[test]
    fn sum_of_odds_adds_odd_numbers() {
        assert_eq!(sum_of_odds(&[1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn sum_of_odds_with_negatives() {
        todo!("YOUR TEST: is -3 odd? (careful with % on negative numbers)")
    }

    #[test]
    fn count_longer_than_counts() {
        assert_eq!(count_longer_than(&["a", "abc", "abcd"], 2), 2);
    }

    #[test]
    fn longest_word_finds_it() {
        assert_eq!(longest_word("a tiny elephant"), Some("elephant"));
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn longest_word_first_one_wins_a_tie() {
        todo!("YOUR TEST: \"the quick brown fox\" (quick and brown are both 5)")
    }

    #[test]
    fn numbered_items() {
        assert_eq!(numbered(&["apple", "pear"]), vec!["1. apple", "2. pear"]);
    }

    #[test]
    fn apply_twice_with_closures() {
        assert_eq!(apply_twice(|x| x * 3, 2), 18);
        assert_eq!(apply_twice(|x| x - 1, 0), -2);
    }

    #[test]
    fn make_adder_remembers_n() {
        let add_five = make_adder(5);
        assert_eq!(add_five(10), 15);
        assert_eq!(add_five(-5), 0);
    }

    // THINK ABOUT IT:
    // In `make_adder`, delete the `move`. What does the compiler say, and why?
}
