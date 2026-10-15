//! Kata 13 — Bank OCR
//!
//! A small bank bought a fancy machine that reads deposit slips and prints
//! the account numbers using pipes and underscores, like an old alarm clock.
//! It saved them a fortune in typists. Then the machine started smudging.
//! A digit here, a missing line there, and money landing in the wrong
//! account. The bank manager slides a stack of printouts across the desk
//! and asks you to make sense of it.
//!
//! Whisper: nine digits are just one digit, nine times.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// Story 1: one entry (3 drawn lines, optionally followed by a blank line)
/// into 9 digits. Unreadable digits become '?'.
pub fn parse_entry(entry: &str) -> String {
    todo!("make the first test pass")
}

/// Story 2: the checksum. `account` is 9 digits.
pub fn is_valid(account: &str) -> bool {
    todo!("story 2")
}

/// Story 3: "457508000", "664371495 ERR" or "86110??36 ILL".
pub fn report(entry: &str) -> String {
    todo!("story 3")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ten digits, line by line. Handy for building entries in tests.
    const DIGITS: [[&str; 3]; 10] = [
        [" _ ", "| |", "|_|"], // 0
        ["   ", "  |", "  |"], // 1
        [" _ ", " _|", "|_ "], // 2
        [" _ ", " _|", " _|"], // 3
        ["   ", "|_|", "  |"], // 4
        [" _ ", "|_ ", " _|"], // 5
        [" _ ", "|_ ", "|_|"], // 6
        [" _ ", "  |", "  |"], // 7
        [" _ ", "|_|", "|_|"], // 8
        [" _ ", "|_|", " _|"], // 9
    ];

    /// Draws an entry for a string of digits, e.g. entry_for("123456789").
    fn entry_for(digits: &str) -> String {
        let mut lines = [String::new(), String::new(), String::new()];
        for d in digits.chars() {
            let glyph = DIGITS[d.to_digit(10).expect("digits only") as usize];
            for (line, part) in lines.iter_mut().zip(glyph) {
                line.push_str(part);
            }
        }
        format!("{}\n{}\n{}\n", lines[0], lines[1], lines[2])
    }

    // ================= Story 1: parse =================

    #[test]
    fn all_zeros() {
        let entry = concat!(
            " _  _  _  _  _  _  _  _  _ \n",
            "| || || || || || || || || |\n",
            "|_||_||_||_||_||_||_||_||_|\n",
            "\n",
        );
        assert_eq!(parse_entry(entry), "000000000");
    }

    #[test]
    #[ignore]
    fn all_ones() {
        let entry = concat!(
            "                           \n",
            "  |  |  |  |  |  |  |  |  |\n",
            "  |  |  |  |  |  |  |  |  |\n",
            "\n",
        );
        assert_eq!(parse_entry(entry), "111111111");
    }

    #[test]
    #[ignore]
    fn every_repeated_digit() {
        todo!("YOUR TEST: use entry_for(\"222222222\") ... up to 9")
    }

    #[test]
    #[ignore]
    fn one_to_nine() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn unreadable_digit_becomes_question_mark() {
        let entry = concat!(
            "    _  _  _  _  _  _     _ \n",
            "|_||_|| || ||_   |  |  | _ \n",
            "  | _||_||_||_|  |  |  | _|\n",
            "\n",
        );
        assert_eq!(parse_entry(entry), "49006771?");
    }

    // ================= Story 2: checksum =================

    #[test]
    #[ignore]
    fn valid_checksum() {
        assert!(is_valid("345882865"));
    }

    #[test]
    #[ignore]
    fn invalid_checksum() {
        todo!("YOUR TEST: \"664371495\" is not valid")
    }

    #[test]
    #[ignore]
    fn more_checksums() {
        todo!("YOUR TEST: work out one valid number by hand and test it")
    }

    // ================= Story 3: report =================

    #[test]
    #[ignore]
    fn report_valid_account() {
        assert_eq!(report(&entry_for("457508000")), "457508000");
    }

    #[test]
    #[ignore]
    fn report_checksum_error() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn report_illegible() {
        todo!("YOUR TEST: reuse the unreadable entry from above")
    }
}
