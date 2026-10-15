//! Kata 08 — Mars Rover
//!
//! Mission control, 3 a.m. A rover has just landed on a dusty patch of Mars,
//! and you're the engineer on shift. Each message takes minutes to arrive, so
//! you send whole strings of orders at once (turn, turn, move) and the rover
//! reports where it ended up. Then the scientists explain that the plateau
//! loops around. Then someone spots rocks.
//!
//! Whisper: the rover knows where it's facing, not just which letter it got.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

#[derive(Debug, Default)]
pub struct MarsRover {
    // You decide.
}

impl MarsRover {
    /// A rover at 0:0 facing North on a 10x10 grid with no obstacles.
    pub fn new() -> Self {
        MarsRover::default()
    }

    /// Stage 3: same as `new`, but with obstacles at the given (x, y) squares.
    pub fn with_obstacles(obstacles: &[(i32, i32)]) -> Self {
        todo!("stage 3")
    }

    /// Runs the commands and returns "x:y:D" (or "O:x:y:D" if blocked).
    pub fn execute(&mut self, commands: &str) -> String {
        todo!("make the first test pass")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(commands: &str) -> String {
        MarsRover::new().execute(commands)
    }

    // ================= Stage 1: turning =================

    #[test]
    fn no_commands_reports_the_start_position() {
        assert_eq!(run(""), "0:0:N");
    }

    #[test]
    #[ignore]
    fn turns_right() {
        assert_eq!(run("R"), "0:0:E");
        assert_eq!(run("RR"), "0:0:S");
    }

    #[test]
    #[ignore]
    fn four_right_turns_is_a_full_circle() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn turns_left() {
        todo!("YOUR TEST: L, LL, LLL, LLLL")
    }

    // ================= Stage 1: moving =================

    #[test]
    #[ignore]
    fn moves_north() {
        assert_eq!(run("M"), "0:1:N");
    }

    #[test]
    #[ignore]
    fn moves_east() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn turns_and_moves() {
        let cases = [("MMRMMLM", "2:3:N"), ("RMMLM", "2:1:N")];
        for (commands, expected) in cases {
            assert_eq!(run(commands), expected, "commands: {commands}");
        }
    }

    // ================= Stage 2: wrapping =================

    #[test]
    #[ignore]
    fn wraps_from_top_to_bottom() {
        assert_eq!(run("MMMMMMMMMM"), "0:0:N");
    }

    #[test]
    #[ignore]
    fn wraps_from_left_to_right() {
        todo!("YOUR TEST: \"LM\" -> \"9:0:W\"")
    }

    #[test]
    #[ignore]
    fn wraps_going_south_and_east() {
        todo!("YOUR TEST")
    }

    // ================= Stage 3: obstacles =================

    #[test]
    #[ignore]
    fn stops_before_an_obstacle() {
        let mut rover = MarsRover::with_obstacles(&[(0, 3)]);
        assert_eq!(rover.execute("MMMM"), "O:0:2:N");
    }

    #[test]
    #[ignore]
    fn ignores_commands_after_being_blocked() {
        todo!("YOUR TEST: e.g. obstacle at (0,1), \"MRMM\" -> ?")
    }

    #[test]
    #[ignore]
    fn obstacle_across_the_wrap() {
        todo!("YOUR TEST: obstacle at (0, 9), \"LLM\" -> ?")
    }
}
