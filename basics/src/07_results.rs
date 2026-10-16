//! # 07 · Result & the ? operator
//!
//! `Option` says "maybe there's a value". `Result` says "this worked, or
//! here's **why** it didn't":
//!
//! ```text
//! enum Result<T, E> { Ok(T), Err(E) }
//! ```
//!
//! Rust has no exceptions. Errors are ordinary values that you return.
//!
//! - `?` after a Result: if it's `Err`, return that error from the current
//!   function right away. If it's `Ok`, unwrap the value and keep going.
//! - `.map_err(|e| ...)` turns one error type into another.
//! - `.ok_or(err)` turns an `Option` into a `Result`.
//! - `.collect()` can turn an iterator of Results into a `Result<Vec<_>, _>`:
//!   it stops at the first error.
//!
//! Book: chapter 9, "Error Handling"
//!
//! Run: `cargo test -p rust-basics results`

use std::num::ParseIntError;

/// Parses a number, ignoring spaces around it. `str::parse` does the work.
pub fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Parses an age.
/// - not a whole positive number -> Err("not a number: <s>")
/// - more than 150               -> Err("too old: <n>")
pub fn parse_age(s: &str) -> Result<u32, String> {
    todo!()
}

/// Parses both and adds them. Use `?` twice.
pub fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Parses every item, failing on the first bad one. Try `map` + `collect`.
pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum WithdrawError {
    ZeroAmount,
    InsufficientFunds { missing: u32 },
}

/// Returns the new balance, or why the withdrawal isn't allowed.
pub fn withdraw(balance: u32, amount: u32) -> Result<u32, WithdrawError> {
    todo!()
}

/// The first character, or Err("empty string"). Try `.chars().next()` + `ok_or`.
pub fn first_char(s: &str) -> Result<char, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_ignores_spaces() {
        assert_eq!(parse_number(" 42 "), Ok(42));
    }

    #[test]
    fn parse_number_rejects_garbage() {
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn parse_age_happy_path() {
        assert_eq!(parse_age("30"), Ok(30));
    }

    #[test]
    fn parse_age_errors_explain_themselves() {
        assert_eq!(parse_age("abc"), Err(String::from("not a number: abc")));
        assert_eq!(parse_age("200"), Err(String::from("too old: 200")));
    }

    #[test]
    fn parse_age_negative() {
        todo!("YOUR TEST: what about \"-5\"?")
    }

    #[test]
    fn sum_two_adds_two_numbers() {
        assert_eq!(sum_two("2", "3"), Ok(5));
        assert!(sum_two("2", "x").is_err());
    }

    #[test]
    fn tests_can_use_the_question_mark_too() -> Result<(), ParseIntError> {
        let n = sum_two("40", "2")?;
        assert_eq!(n, 42);
        Ok(())
    }

    #[test]
    fn parse_all_numbers() {
        assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert!(parse_all(&["1", "oops", "3"]).is_err());
    }

    #[test]
    fn withdraw_rules() {
        assert_eq!(withdraw(100, 30), Ok(70));
        assert_eq!(withdraw(10, 25), Err(WithdrawError::InsufficientFunds { missing: 15 }));
        assert_eq!(withdraw(10, 0), Err(WithdrawError::ZeroAmount));
    }

    #[test]
    fn withdraw_everything() {
        todo!("YOUR TEST: is withdrawing the exact balance allowed?")
    }

    #[test]
    fn first_char_of_strings() {
        assert_eq!(first_char("rust"), Ok('r'));
        assert_eq!(first_char(""), Err(String::from("empty string")));
    }
}
