//! Kata 12 — Minesweeper
//!
//! The war ended decades ago, but the fields outside the village still hold
//! surprises. A team of deminers maps each field square by square: a star
//! where they found a mine, a dot where the ground is clear. Before farmers
//! go back out, each dot needs a number, a warning of how many mines lie
//! within one step. Nobody wants a typo on this map.
//!
//! Whisper: you've counted neighbours before. Did you like how you did it?
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// Stage 1: one field, '*' = mine, '.' = safe.
pub fn annotate(field: &str) -> String {
    todo!("make the first test pass")
}

/// Stage 2: several fields with "rows cols" headers, ending in "0 0".
pub fn annotate_all(input: &str) -> String {
    todo!("stage 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ================= Stage 1 =================

    #[test]
    fn empty_field() {
        assert_eq!(annotate(""), "");
    }

    #[test]
    #[ignore]
    fn single_safe_square() {
        assert_eq!(annotate("."), "0");
    }

    #[test]
    #[ignore]
    fn single_mine() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn one_row() {
        assert_eq!(annotate(".*."), "1*1");
        todo!("YOUR TEST: add \"*.*\" and \"**.\"")
    }

    #[test]
    #[ignore]
    fn one_column() {
        todo!("YOUR TEST: \".\\n*\\n.\" -> ?")
    }

    #[test]
    #[ignore]
    fn diagonals_count_too() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn four_by_four_example() {
        let field = "*...\n....\n.*..\n....";
        let expected = "*100\n2210\n1*10\n1110";
        assert_eq!(annotate(field), expected);
    }

    #[test]
    #[ignore]
    fn surrounded_square_sees_eight() {
        todo!("YOUR TEST")
    }

    // ================= Stage 2 =================

    #[test]
    #[ignore]
    fn single_field_with_header() {
        assert_eq!(annotate_all("1 3\n.*.\n0 0"), "Field #1:\n1*1");
    }

    #[test]
    #[ignore]
    fn two_fields() {
        todo!("YOUR TEST: use the example from the README")
    }

    #[test]
    #[ignore]
    fn no_fields() {
        todo!("YOUR TEST: just \"0 0\" -> ?")
    }
}
