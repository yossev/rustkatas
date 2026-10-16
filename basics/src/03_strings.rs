//! # 03 · Strings & slices
//!
//! - `String` is an **owned**, growable string on the heap.
//! - `&str` is a **borrowed view** (a "slice") into some string data: a
//!   pointer and a length. String literals like `"hi"` are `&'static str`.
//!
//! A function that returns `&str` from a `&str` input hands back a piece of
//! the *caller's* data, with no copying. That's why `first_word` below can
//! return `&str` without allocating.
//!
//! Lifetimes (`'a`) show up when the compiler can't tell *which* input the
//! returned reference comes from. See `longest`.
//!
//! Careful: `s.len()` counts **bytes**. `s.chars()` walks **characters**.
//!
//! Book: chapters 4.3 ("The Slice Type"), 8.2 ("Strings") and 10.3 ("Lifetimes")
//!
//! Run: `cargo test -p rust-basics strings`

/// Everything before the first space, or the whole string if there's none.
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// The first letter of each word, upper-cased: "Ada Lovelace" -> "AL".
pub fn initials(full_name: &str) -> String {
    todo!()
}

/// Upper-case the first character: "rust" -> "Rust". Empty stays empty.
pub fn capitalize(word: &str) -> String {
    todo!()
}

/// "one two three" -> "three two one"
pub fn reverse_words(s: &str) -> String {
    todo!()
}

/// The longer of the two (in characters). If they tie, return `a`.
/// The `'a` says: the result lives as long as both inputs do.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// Reads the same backwards, ignoring case and spaces.
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_stops_at_the_space() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn first_word_of_a_single_word_is_the_word() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn initials_are_upper_case() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("grace brewster hopper"), "GBH");
    }

    #[test]
    fn capitalize_only_touches_the_first_letter() {
        assert_eq!(capitalize("rust"), "Rust");
        assert_eq!(capitalize("rUST"), "RUST");
    }

    #[test]
    fn capitalize_empty_string() {
        todo!("YOUR TEST")
    }

    #[test]
    fn reverse_words_reverses_the_order() {
        assert_eq!(reverse_words("one two three"), "three two one");
    }

    #[test]
    fn longest_picks_the_longer_one() {
        assert_eq!(longest("apple", "kiwi"), "apple");
        assert_eq!(longest("fig", "banana"), "banana");
    }

    #[test]
    fn longest_prefers_the_first_on_a_tie() {
        assert_eq!(longest("ab", "cd"), "ab");
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome("Racecar"));
        assert!(is_palindrome("never odd or even"));
    }

    #[test]
    fn not_palindromes() {
        todo!("YOUR TEST")
    }

    // THINK ABOUT IT:
    // Why does `first_word` not need a `'a` but `longest` does?
    // (Book 10.3, "Lifetime Elision".)
}
