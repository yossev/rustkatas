//! Kata 04 — String Calculator
//!
//! It's your first week at a small startup. The product manager drops by:
//! "Just a tiny thing: add up some numbers in a string. Easy, right?" You
//! ship it. They come back the next morning. "Customers also use newlines."
//! And the morning after: "One customer uses semicolons..." They will keep
//! coming back, and you can't know what they'll ask for next.
//!
//! Whisper: build for today's request; tomorrow's hasn't been asked yet.
//!
//! Read README.md for the rules. Don't read ahead!
#![allow(unused)] // delete this line when you're done, then fix the warnings

#[derive(Debug, PartialEq, Eq)]
pub enum CalcError {
    /// All negative numbers found in the input, in order.
    Negatives(Vec<i32>),
    /// Anything that can't be parsed.
    InvalidInput(String),
}

pub fn add(numbers: &str) -> Result<i32, CalcError> {
    todo!("make the first test pass")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- Step 1 ----------

    #[test]
    fn empty_string_is_zero() {
        assert_eq!(add(""), Ok(0));
    }

    #[test]
    #[ignore]
    fn single_number_is_itself() {
        assert_eq!(add("1"), Ok(1));
    }

    #[test]
    #[ignore]
    fn two_numbers_are_summed() {
        todo!("YOUR TEST: \"1,2\" -> 3")
    }

    // ---------- Step 2 ----------

    #[test]
    #[ignore]
    fn any_amount_of_numbers() {
        todo!("YOUR TEST")
    }

    // ---------- Step 3 ----------

    #[test]
    #[ignore]
    fn newlines_are_separators() {
        todo!("YOUR TEST")
    }

    // ---------- Step 4 ----------

    #[test]
    #[ignore]
    fn custom_single_char_delimiter() {
        todo!("YOUR TEST")
    }

    // ---------- Step 5 ----------

    #[test]
    #[ignore]
    fn single_negative_is_rejected() {
        assert_eq!(add("1,-2"), Err(CalcError::Negatives(vec![-2])));
    }

    #[test]
    #[ignore]
    fn all_negatives_are_reported() {
        todo!("YOUR TEST")
    }

    // ---------- Step 6 ----------

    #[test]
    #[ignore]
    fn numbers_over_1000_are_ignored() {
        todo!("YOUR TEST: and check that 1000 itself is NOT ignored")
    }

    // ---------- Step 7 ----------

    #[test]
    #[ignore]
    fn long_delimiter() {
        todo!("YOUR TEST")
    }

    // ---------- Step 8 ----------

    #[test]
    #[ignore]
    fn multiple_delimiters() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn multiple_long_delimiters() {
        todo!("YOUR TEST")
    }
}
