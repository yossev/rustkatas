//! # 01 · Ownership
//!
//! Every value in Rust has exactly **one owner**. When the owner goes out of
//! scope, the value is dropped (freed).
//!
//! - Passing a `String` **by value** (`s: String`) *moves* it into the
//!   function. The caller can't use it afterwards, unless the function hands
//!   it back.
//! - Passing a **reference** (`s: &str`, `v: &[i32]`) *lends* it. The caller
//!   keeps ownership and can keep using it.
//!
//! Before writing each body, read the signature. It tells you whether the
//! function owns its input or only borrows it.
//!
//! Book: chapter 4.1, "What is Ownership?"
//!
//! Run: `cargo test -p rust-basics ownership`

use std::fmt::format;

/// Takes ownership of `s` and returns it upper-cased with "!" on the end.
/// `"hi"` -> `"HI!"`
pub fn shout(mut s: String) -> String {
    s.make_ascii_uppercase();
    s.push('!');
    s
}

/// Borrows `s` and returns how many **characters** it has (not bytes!).
pub fn char_count(s: &str) -> usize {
    // s.len() would return the size of the string in bytes
    // s.chars().count() would return the amount of letters instead
    s.chars().count()
}

/// Builds a brand-new String: `"Hello, <name>!"`.
pub fn greeting(name: &str) -> String {
    let greet = format!("Hello, {name}!");
    greet
}

/// Takes ownership of both Strings and returns them joined with a space.
pub fn join(a: String, b: String) -> String {
    let joined = format!("{a} {b}");
    joined
}

/// Returns a new Vec with every number doubled. `v` is only borrowed.
pub fn doubled(v: &[i32]) -> Vec<i32> {
    let mut new_vec = v.to_vec(); //v is borrowed read-only, so make an owned copy we're allowed to change, clone cannot be used here since it will clone the ref not the values
    for x in new_vec.iter_mut() {
        *x *= 2;
    }
    return new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_returns_uppercase_with_a_bang() {
        assert_eq!(shout(String::from("hi")), "HI!");
    }

    #[test]
    fn char_count_counts_characters_not_bytes() {
        assert_eq!(char_count("hello"), 5);
        assert_eq!(char_count("héllo"), 5); // 'é' is 2 bytes, but 1 character
    }

    #[test]
    fn the_caller_keeps_a_borrowed_string() {
        let name = String::from("Ada");
        let g = greeting(&name); // &String turns into &str automatically
        assert_eq!(g, "Hello, Ada!");
        assert_eq!(name, "Ada"); // still ours: we only lent it
    }

    #[test]
    fn join_takes_both_strings() {
        let a = String::from("good");
        let b = String::from("morning");
        assert_eq!(join(a, b), "good morning");
        // a and b have moved into `join`. Try using them here and read the error.
    }

    #[test]
    fn doubled_leaves_the_original_alone() {
        let v = vec![1, 2, 3];
        assert_eq!(doubled(&v), vec![2, 4, 6]);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn doubled_of_nothing_is_nothing() {
        assert_eq!(doubled(&[]), Vec::<i32>::new());
    }

    // THINK ABOUT IT (try it in a test, read the compiler error):
    //
    //     let s = String::from("hi");
    //     let loud = shout(s);
    //     println!("{s}");
    //
    // Why doesn't this compile? How would you fix it if you needed both
    // `s` and `loud`? (Hint: there are two fixes. One costs a copy.)
}
