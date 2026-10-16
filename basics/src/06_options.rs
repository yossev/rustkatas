//! # 06 · Option
//!
//! Rust has no `null`. A value that might be missing is an `Option<T>`:
//!
//! ```text
//! enum Option<T> { Some(T), None }
//! ```
//!
//! The compiler won't let you use the `T` inside until you've dealt with
//! the `None` case. Ways to get at the value, from most explicit to most compact:
//!
//! - `match opt { Some(x) => ..., None => ... }`
//! - `if let Some(x) = opt { ... }`
//! - `opt.unwrap_or(default)`: a fallback value
//! - `opt.map(|x| ...)`: transform the value if it's there
//! - `opt.and_then(|x| ...)`: when the transformation itself returns an Option
//! - `opt?` inside a function returning Option: "give up and return None early"
//!
//! Avoid `.unwrap()` in real code: it crashes on `None`.
//!
//! Book: chapter 6.1 (Option section) and 9.2 (the `?` operator)
//!
//! Run: `cargo test -p rust-basics options`

/// The first even number, if there is one.
pub fn first_even(v: &[i32]) -> Option<i32> {
    todo!()
}

/// a / b, or None when b is 0.
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    todo!()
}

/// The first character of the name, if the name isn't empty.
pub fn initial(name: &str) -> Option<char> {
    todo!()
}

/// The name, or "anonymous" if there isn't one. Try `unwrap_or`.
pub fn name_or_anonymous(name: Option<&str>) -> String {
    todo!()
}

/// Doubles the number if there is one. Try `map`, no `match`.
pub fn double(x: Option<i32>) -> Option<i32> {
    todo!()
}

/// a + b, but only if both are there. Try the `?` operator.
pub fn add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!()
}

/// Looks up a user's age by name.
pub fn find_age(users: &[(&str, u32)], name: &str) -> Option<u32> {
    todo!()
}

/// The user's age next year. Reuse `find_age`, then `map` or `?`.
pub fn age_next_year(users: &[(&str, u32)], name: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const USERS: &[(&str, u32)] = &[("ada", 36), ("grace", 85), ("linus", 54)];

    #[test]
    fn first_even_finds_one() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    }

    #[test]
    fn first_even_when_there_is_none() {
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn safe_divide_works() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }

    #[test]
    fn initial_of_names() {
        assert_eq!(initial("ada"), Some('a'));
        todo!("YOUR TEST: and the empty name?")
    }

    #[test]
    fn name_or_anonymous_uses_the_fallback() {
        assert_eq!(name_or_anonymous(Some("ada")), "ada");
        assert_eq!(name_or_anonymous(None), "anonymous");
    }

    #[test]
    fn double_keeps_none_as_none() {
        assert_eq!(double(Some(21)), Some(42));
        assert_eq!(double(None), None);
    }

    #[test]
    fn add_needs_both() {
        assert_eq!(add(Some(1), Some(2)), Some(3));
        assert_eq!(add(Some(1), None), None);
        assert_eq!(add(None, Some(2)), None);
    }

    #[test]
    fn find_age_of_known_and_unknown_users() {
        assert_eq!(find_age(USERS, "grace"), Some(85));
        assert_eq!(find_age(USERS, "bob"), None);
    }

    #[test]
    fn age_next_year_works() {
        todo!("YOUR TEST: a known user and an unknown one")
    }
}
