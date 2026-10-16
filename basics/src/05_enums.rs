//! # 05 · Enums & match
//!
//! An `enum` is a type that is **one of** several variants. Each variant
//! can carry its own data. `match` makes you handle **every** variant, and
//! the compiler checks that you didn't forget one.
//!
//! `match` can also match on ranges (`1..=9`), several patterns at once
//! (`A | B`) and add conditions (`n if n < 0`).
//!
//! Book: chapter 6, "Enums and Pattern Matching" (and 19 for more patterns)
//!
//! Run: `cargo test -p rust-basics enums`

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// Red -> Green -> Yellow -> Red
    pub fn next(self) -> Self {
        todo!()
    }

    /// How long the light stays on: Red 30, Yellow 5, Green 25.
    pub fn seconds(self) -> u32 {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Square(f64),
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        todo!("hint: std::f64::consts::PI")
    }
}

/// "negative", "zero", "small" (1 to 9) or "large" (10 and up).
pub fn classify(n: i32) -> &'static str {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Quit,
}

/// Move { x: 1, y: 2 } -> "move to (1, 2)", Say("hi") -> "say: hi", Quit -> "quit"
pub fn describe(cmd: &Command) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9 // never compare floats with ==
    }

    #[test]
    fn traffic_light_next() {
        assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
        assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
        assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
    }

    #[test]
    fn three_steps_is_a_full_cycle() {
        todo!("YOUR TEST: for every light, .next().next().next() is itself")
    }

    #[test]
    fn traffic_light_seconds() {
        assert_eq!(TrafficLight::Red.seconds(), 30);
        assert_eq!(TrafficLight::Yellow.seconds(), 5);
        assert_eq!(TrafficLight::Green.seconds(), 25);
    }

    #[test]
    fn shape_areas() {
        assert!(close(Shape::Circle { radius: 1.0 }.area(), std::f64::consts::PI));
        assert!(close(Shape::Square(3.0).area(), 9.0));
        assert!(close(Shape::Rectangle { width: 2.0, height: 5.0 }.area(), 10.0));
    }

    #[test]
    fn classify_numbers() {
        assert_eq!(classify(-3), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(5), "small");
        assert_eq!(classify(42), "large");
    }

    #[test]
    fn classify_boundaries() {
        todo!("YOUR TEST: 1, 9, 10, and i32::MIN")
    }

    #[test]
    fn describe_commands() {
        assert_eq!(describe(&Command::Move { x: 1, y: 2 }), "move to (1, 2)");
        assert_eq!(describe(&Command::Say(String::from("hi"))), "say: hi");
        assert_eq!(describe(&Command::Quit), "quit");
    }

    // THINK ABOUT IT:
    // Add a fourth colour to TrafficLight (say, Blinking). What does the
    // compiler tell you? Why is that a good thing?
}
