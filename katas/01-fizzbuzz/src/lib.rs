//! Kata 01 — FizzBuzz
//!
//! Long, long car rides in a small town. To pass the time, the children play
//! a counting game: one, two... and on three someone shouts "Fizz!" Five is
//! "Buzz!", fifteen is a triumphant "FizzBuzz!" Whoever slips up is out. The
//! youngest, tired of losing, has asked you for a little machine that never
//! gets it wrong. They don't care how it works, only that it's right, one
//! number at a time.
//!
//! Whisper: the kids didn't learn the whole game at once.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// Returns "Fizz", "Buzz", "FizzBuzz" or the number itself.
pub fn fizzbuzz(n: u32) -> String {
    todo!("make the first test pass — the simplest way possible")
}

/// Returns fizzbuzz(1), fizzbuzz(2), ..., fizzbuzz(n).
pub fn fizzbuzz_up_to(n: u32) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- numbers ----------

    #[test]
    fn one_is_just_the_number() {
        assert_eq!(fizzbuzz(1), "1");
    }

    #[test]
    #[ignore]
    fn two_is_just_the_number() {
        assert_eq!(fizzbuzz(2), "2");
    }

    // ---------- fizz ----------

    #[test]
    #[ignore]
    fn three_is_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }

    #[test]
    #[ignore]
    fn six_is_fizz() {
        assert_eq!(fizzbuzz(6), "Fizz");
    }

    // ---------- buzz (yours) ----------

    #[test]
    #[ignore]
    fn five_is_buzz() {
        todo!("YOUR TEST: fizzbuzz(5) should be \"Buzz\"")
    }

    #[test]
    #[ignore]
    fn ten_is_buzz() {
        todo!("YOUR TEST")
    }

    // ---------- fizzbuzz (yours) ----------

    #[test]
    #[ignore]
    fn fifteen_is_fizzbuzz() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn thirty_is_fizzbuzz() {
        todo!("YOUR TEST")
    }

    // ---------- sequence ----------

    #[test]
    #[ignore]
    fn sequence_up_to_five() {
        assert_eq!(fizzbuzz_up_to(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    #[ignore]
    fn sequence_up_to_zero_is_empty() {
        todo!("YOUR TEST: what should fizzbuzz_up_to(0) return?")
    }

    // ---------- stage 2: "contains" rules (write these yourself) ----------
    // 13 -> "Fizz", 52 -> "Buzz", 35 -> "FizzBuzz", 53 -> ?
    // Add your tests here.
}
