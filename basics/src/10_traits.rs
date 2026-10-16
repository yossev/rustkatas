//! # 10 · Traits & generics
//!
//! A **trait** is a set of methods a type promises to have, a bit like an
//! interface. Types opt in with `impl Trait for Type`.
//!
//! - Traits can have **default methods** that implementors get for free
//!   (and can override).
//! - **Generics with trait bounds** (`fn largest<T: PartialOrd + Copy>`)
//!   write one function for many types. The compiler makes a copy per type,
//!   so it's as fast as hand-written code.
//! - **Trait objects** (`&dyn Describe`) let you mix different types in one
//!   list, at the cost of a small runtime lookup.
//! - Standard traits like `Display` (for `{}` in `format!`) and `Add` (for `+`)
//!   plug your types into the language itself.
//!
//! Book: chapter 10.1–10.2 (generics, traits) and 17.2 (trait objects)
//!
//! Run: `cargo test -p rust-basics traits`

use std::fmt;
use std::ops::Add;

pub trait Describe {
    fn describe(&self) -> String;
}

pub trait Greet {
    fn name(&self) -> String;

    /// Default: "Hello, <name>!". Write the default body here, once.
    fn greet(&self) -> String {
        todo!()
    }
}

pub struct Dog {
    pub name: String,
}

pub struct Robot {
    pub id: u32,
}

impl Describe for Dog {
    /// "Dog named <name>"
    fn describe(&self) -> String {
        todo!()
    }
}

impl Describe for Robot {
    /// "Robot #<id>"
    fn describe(&self) -> String {
        todo!()
    }
}

impl Greet for Dog {
    fn name(&self) -> String {
        todo!()
    }
    // Dog uses the default greet.
}

impl Greet for Robot {
    /// "Robot #<id>"
    fn name(&self) -> String {
        todo!()
    }

    /// Robots override it: "BEEP BOOP. I AM ROBOT #<id>."
    fn greet(&self) -> String {
        todo!()
    }
}

/// Describe every item, whatever its type.
pub fn describe_all(items: &[&dyn Describe]) -> Vec<String> {
    todo!()
}

/// The largest item, or None for an empty slice. Works for any comparable type.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    /// Point { x: 1, y: -2 } prints as "(1, -2)". Use `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Add for Point {
    type Output = Point;

    /// Adds the x's and the y's.
    fn add(self, other: Point) -> Point {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rex() -> Dog {
        Dog { name: String::from("Rex") }
    }

    #[test]
    fn dogs_and_robots_describe_themselves() {
        assert_eq!(rex().describe(), "Dog named Rex");
        assert_eq!(Robot { id: 7 }.describe(), "Robot #7");
    }

    #[test]
    fn dogs_use_the_default_greeting() {
        assert_eq!(rex().greet(), "Hello, Rex!");
    }

    #[test]
    fn robots_override_the_greeting() {
        todo!("YOUR TEST")
    }

    #[test]
    fn describe_all_mixes_types() {
        let dog = rex();
        let robot = Robot { id: 1 };
        let items: Vec<&dyn Describe> = vec![&dog, &robot];
        assert_eq!(describe_all(&items), vec!["Dog named Rex", "Robot #1"]);
    }

    #[test]
    fn largest_works_for_many_types() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&[1.5, -2.0]), Some(1.5));
        assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
    }

    #[test]
    fn largest_of_nothing() {
        todo!("YOUR TEST: careful, `largest(&[])` alone can't work out what T is. Try `largest::<i32>(&[])`")
    }

    #[test]
    fn points_display_nicely() {
        let p = Point { x: 1, y: -2 };
        assert_eq!(p.to_string(), "(1, -2)");
        assert_eq!(format!("at {p}"), "at (1, -2)");
    }

    #[test]
    fn points_can_be_added() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 10, y: 20 };
        assert_eq!(a + b, Point { x: 11, y: 22 });
    }

    // THINK ABOUT IT:
    // `largest` needs `Copy`. Remove it from the bound. What breaks, and why?
    // Could you return `Option<&T>` instead and drop the `Copy`?
}
