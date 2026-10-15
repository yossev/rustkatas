//! Kata 05 — Bowling Game
//!
//! At the Lucky Strike Lanes the scoreboard died during league night, and
//! old Marge has been scoring games in pencil ever since. She's quick, but
//! strikes and spares make her squint: "Wait, I need the *next* ball before
//! I can write this one down." She'd like a machine that just listens to the
//! balls fall and tells her the score when the game's over.
//!
//! Whisper: Marge doesn't add up as the balls fall; she waits and looks back.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

#[derive(Debug, Default)]
pub struct Game {
    // You decide what goes in here. Start with nothing!
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    /// Called each time the player rolls a ball. `pins` = pins knocked down.
    pub fn roll(&mut self, pins: u32) {
        todo!()
    }

    /// Called only at the very end of the game.
    pub fn score(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper: roll the same number of pins `n` times.
    fn roll_many(game: &mut Game, n: usize, pins: u32) {
        for _ in 0..n {
            game.roll(pins);
        }
    }

    // Idea: you may want more helpers, e.g. `roll_spare(game)` / `roll_strike(game)`.

    #[test]
    fn gutter_game() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 0);
        assert_eq!(game.score(), 0);
    }

    #[test]
    #[ignore]
    fn all_ones() {
        let mut game = Game::new();
        roll_many(&mut game, 20, 1);
        assert_eq!(game.score(), 20);
    }

    #[test]
    #[ignore]
    fn one_spare() {
        todo!("YOUR TEST: 5, 5, 3, then 17 zeros -> 16")
    }

    #[test]
    #[ignore]
    fn one_strike() {
        todo!("YOUR TEST: 10, 3, 4, then 16 zeros -> 24")
    }

    #[test]
    #[ignore]
    fn perfect_game() {
        todo!("YOUR TEST: 12 strikes -> 300")
    }

    #[test]
    #[ignore]
    fn all_spares_with_final_bonus() {
        todo!("YOUR TEST: 21 rolls of 5 -> ?  (work it out by hand first!)")
    }

    #[test]
    #[ignore]
    fn nine_and_miss_every_frame() {
        todo!("YOUR TEST")
    }
}
