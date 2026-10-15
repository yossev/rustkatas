//! Kata 06 — Word Wrap
//!
//! A village newspaper still prints in narrow columns, and the typesetter is
//! retiring. For forty years they've laid out each line by hand: fit what
//! you can, break at a space, and if a word is just too long, cut it.
//! Before leaving, they sit you in their chair. "Don't start with the front
//! page," they say. "Start with a single letter."
//!
//! Whisper: if the next step feels like a leap, you chose the wrong step.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// Insert line breaks so that no line in `text` is longer than `width`.
pub fn wrap(text: &str, width: usize) -> String {
    todo!("make the first test pass")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- degenerate ----------

    #[test]
    fn empty_string() {
        assert_eq!(wrap("", 1), "");
    }

    #[test]
    #[ignore]
    fn one_short_word() {
        assert_eq!(wrap("x", 1), "x");
    }

    // ---------- words longer than width ----------

    #[test]
    #[ignore]
    fn word_twice_the_width_is_split() {
        assert_eq!(wrap("xx", 1), "x\nx");
    }

    #[test]
    #[ignore]
    fn word_three_times_the_width() {
        todo!("YOUR TEST: \"xxx\", 1")
    }

    #[test]
    #[ignore]
    fn long_word_with_wider_width() {
        todo!("YOUR TEST: e.g. \"xxxxx\", 2 -> ?")
    }

    // ---------- spaces ----------

    #[test]
    #[ignore]
    fn space_becomes_a_line_break() {
        todo!("YOUR TEST: \"x x\", 1")
    }

    #[test]
    #[ignore]
    fn break_at_the_space_not_inside_the_word() {
        todo!("YOUR TEST: \"x xx\", 3")
    }

    #[test]
    #[ignore]
    fn words_that_fit_stay_on_one_line() {
        todo!("YOUR TEST: \"x x\", 3 -> \"x x\"")
    }

    // ---------- the real thing ----------

    #[test]
    #[ignore]
    fn four_score_width_7() {
        assert_eq!(
            wrap("four score and seven years ago", 7),
            "four\nscore\nand\nseven\nyears\nago"
        );
    }

    #[test]
    #[ignore]
    fn four_score_width_9() {
        todo!("YOUR TEST: see README")
    }

    #[test]
    #[ignore]
    fn long_word_mixed_with_short_words() {
        todo!("YOUR TEST: e.g. \"a supercalifragilistic b\", 6 — work it out by hand")
    }
}
