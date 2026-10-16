//! # 04 · Structs & methods
//!
//! Methods live in an `impl` block. The first parameter tells you how the
//! method uses the value:
//!
//! - `&self`: reads it (most methods)
//! - `&mut self`: changes it
//! - `self`: **consumes** it (the value is gone afterwards)
//! - no `self` at all: an "associated function", like a constructor (`Rectangle::new`)
//!
//! `Self` is just a shortcut for the type's own name.
//!
//! Book: chapter 5, "Using Structs to Structure Related Data"
//!
//! Run: `cargo test -p rust-basics structs`

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    pub fn square(size: u32) -> Self {
        todo!()
    }

    pub fn area(&self) -> u32 {
        todo!()
    }

    pub fn is_square(&self) -> bool {
        todo!()
    }

    /// True if `other` fits strictly inside `self` (smaller in both directions).
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// Multiplies both sides by `factor`.
    pub fn scale(&mut self, factor: u32) {
        todo!()
    }

    /// Consumes the rectangle and gives back `(width, height)`.
    pub fn into_dimensions(self) -> (u32, u32) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub active: bool,
}

impl User {
    /// New users start out active.
    pub fn new(name: &str, email: &str) -> Self {
        todo!()
    }

    pub fn deactivate(&mut self) {
        todo!()
    }

    /// The part of the email after '@'. Borrowed from `self`, no copy.
    /// (You may assume the email contains an '@'.)
    pub fn email_domain(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_area() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r.area(), 12);
    }

    #[test]
    fn square_is_square() {
        let s = Rectangle::square(5);
        assert_eq!(s.area(), 25);
        assert!(s.is_square());
    }

    #[test]
    fn a_rectangle_is_not_a_square() {
        todo!("YOUR TEST")
    }

    #[test]
    fn can_hold_a_smaller_rectangle() {
        let big = Rectangle::new(10, 8);
        let small = Rectangle::new(5, 3);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn scale_changes_the_rectangle() {
        let mut r = Rectangle::new(2, 3);
        r.scale(2);
        assert_eq!(r, Rectangle::new(4, 6));
    }

    #[test]
    fn into_dimensions_consumes_the_rectangle() {
        let r = Rectangle::new(7, 9);
        assert_eq!(r.into_dimensions(), (7, 9));
        // `r` is gone now. Try `r.area()` here and read the error.
    }

    #[test]
    fn new_users_are_active() {
        let u = User::new("ada", "ada@example.com");
        assert_eq!(u.name, "ada");
        assert!(u.active);
    }

    #[test]
    fn users_can_be_deactivated() {
        todo!("YOUR TEST")
    }

    #[test]
    fn email_domain() {
        let u = User::new("grace", "grace@navy.mil");
        assert_eq!(u.email_domain(), "navy.mil");
    }
}
