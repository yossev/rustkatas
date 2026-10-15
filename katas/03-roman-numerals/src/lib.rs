//! Kata 03 — Roman Numerals
//!
//! A stonemason in old Rome has more work than hands. Temples want their
//! founding years carved over the door, legions want numbers on their
//! shields, and each order arrives in plain digits. The mason is tired of
//! counting on their fingers and asks you to write down how it's done once,
//! so anyone can carve it. Later the archivists will need the opposite: read
//! the old stones back into numbers.
//!
//! Whisper: the mason never thinks in rules, only in a few familiar symbols.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

#[derive(Debug, PartialEq, Eq)]
pub enum RomanError {
    Empty,
    InvalidSymbol(char),
    Malformed,
}

/// 1994 -> "MCMXCIV"
pub fn to_roman(n: u32) -> String {
    todo!("make the first test pass")
}

/// "MCMXCIV" -> Ok(1994)
pub fn from_roman(s: &str) -> Result<u32, RomanError> {
    todo!("stage 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ================= Stage 1: to_roman =================

    #[test]
    fn one_is_i() {
        assert_eq!(to_roman(1), "I");
    }

    #[test]
    #[ignore]
    fn two_is_ii() {
        assert_eq!(to_roman(2), "II");
    }

    #[test]
    #[ignore]
    fn three_is_iii() {
        assert_eq!(to_roman(3), "III");
    }

    #[test]
    #[ignore]
    fn five_is_v() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn four_is_iv() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn ten_and_nine() {
        todo!("YOUR TEST: 10 -> X, 9 -> IX")
    }

    #[test]
    #[ignore]
    fn forty_is_xl() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn larger_numbers() {
        // A table-driven test: add rows as you go.
        let cases = [(1994, "MCMXCIV"), (3999, "MMMCMXCIX")];
        for (n, expected) in cases {
            assert_eq!(to_roman(n), expected, "to_roman({n})");
        }
    }

    // ================= Stage 2: from_roman =================

    #[test]
    #[ignore]
    fn parses_single_symbols() {
        assert_eq!(from_roman("I"), Ok(1));
        assert_eq!(from_roman("V"), Ok(5));
        assert_eq!(from_roman("M"), Ok(1000));
    }

    #[test]
    #[ignore]
    fn parses_additive_numerals() {
        todo!("YOUR TEST: e.g. \"VIII\" -> 8")
    }

    #[test]
    #[ignore]
    fn parses_subtractive_numerals() {
        todo!("YOUR TEST: e.g. \"IV\", \"MCMXCIV\"")
    }

    #[test]
    #[ignore]
    fn empty_string_is_an_error() {
        assert_eq!(from_roman(""), Err(RomanError::Empty));
    }

    #[test]
    #[ignore]
    fn unknown_symbol_is_an_error() {
        todo!("YOUR TEST: from_roman(\"XAV\") -> Err(InvalidSymbol('A'))")
    }

    // ================= Stage 3: property =================

    #[test]
    #[ignore]
    fn round_trip_for_every_valid_number() {
        todo!("YOUR TEST: for n in 1..=3999, from_roman(&to_roman(n)) == Ok(n)")
    }
}
