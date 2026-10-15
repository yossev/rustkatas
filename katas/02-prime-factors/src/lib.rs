//! Kata 02 — Prime Factors
//!
//! In the old mint, every coin is struck from a few kinds of pure metal that
//! can't be split any further. An apprentice is handed a coin stamped "12"
//! and told to melt it back into its pure ingots: two, two, three. The master
//! hands over coin after coin, each heavier than the last, and watches
//! quietly. Make a mess with the first coins, the master says, because the
//! method will show itself.
//!
//! Whisper: what you did once, you may need to do again.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// Prime factors of `n` in ascending order. `prime_factors(12) == [2, 2, 3]`.
pub fn prime_factors(n: u64) -> Vec<u64> {
    todo!("make the first test pass")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_has_no_factors() {
        assert_eq!(prime_factors(1), Vec::<u64>::new());
    }

    #[test]
    #[ignore]
    fn two() {
        assert_eq!(prime_factors(2), vec![2]);
    }

    #[test]
    #[ignore]
    fn three() {
        assert_eq!(prime_factors(3), vec![3]);
    }

    #[test]
    #[ignore]
    fn four() {
        todo!("YOUR TEST: 4 -> [2, 2]")
    }

    #[test]
    #[ignore]
    fn six() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn eight() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn nine() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn a_big_composite_number() {
        todo!("YOUR TEST: 2*2*3*3*5*7*11*13 -> ?")
    }

    // Stretch: what about 0? What about a large prime?
}
