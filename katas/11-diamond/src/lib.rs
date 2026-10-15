//! Kata 11 — Diamond
//!
//! A jeweller at the night market cuts glass diamonds for children, one for
//! every letter of the alphabet. "A" is just a speck. "Z" is as wide as the
//! stall. Every stone has the same perfect shape: it widens, turns at its
//! letter, and narrows back down. The jeweller has never drawn a "Q" stone,
//! but they know exactly what it looks like.
//!
//! Whisper: you don't need to see the whole stone to know it's a diamond.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

pub fn diamond(letter: char) -> String {
    todo!("make the first test pass")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(letter: char) -> Vec<String> {
        diamond(letter).lines().map(str::to_string).collect()
    }

    fn all_letters() -> impl Iterator<Item = char> {
        'A'..='Z'
    }

    // ---------- examples ----------

    #[test]
    fn a_is_just_a() {
        assert_eq!(diamond('A'), "A");
    }

    #[test]
    #[ignore]
    fn b_diamond() {
        assert_eq!(diamond('B'), " A \nB B\n A ");
    }

    #[test]
    #[ignore]
    fn c_diamond() {
        todo!("YOUR TEST: write out the whole C diamond. Was it fun?")
    }

    // ---------- properties ----------

    #[test]
    #[ignore]
    fn is_square() {
        for letter in all_letters() {
            let rows = rows(letter);
            let expected = 2 * (letter as usize - 'A' as usize) + 1;
            assert_eq!(rows.len(), expected, "height for {letter}");
            for row in &rows {
                assert_eq!(row.chars().count(), expected, "width for {letter}");
            }
        }
    }

    #[test]
    #[ignore]
    fn first_and_last_rows_contain_a() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn is_vertically_symmetric() {
        todo!("YOUR TEST: the rows reversed are the same rows")
    }

    #[test]
    #[ignore]
    fn is_horizontally_symmetric() {
        todo!("YOUR TEST: each row reads the same backwards")
    }

    #[test]
    #[ignore]
    fn each_row_contains_its_own_letter() {
        todo!("YOUR TEST: top half, row i contains only the i-th letter (and spaces)")
    }

    #[test]
    #[ignore]
    fn every_row_but_a_has_two_letters() {
        todo!("YOUR TEST")
    }
}
