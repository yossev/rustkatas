//! Kata 09 — Tennis Refactoring
//!
//! The local tennis club's scoreboard software was written one summer by a
//! member's nephew, who then left for university. It works, and the umpires
//! love it. But it says "player1" even at the club final, and every time
//! someone opens the code, they close it again quickly. The committee has
//! voted, and you've been handed the racket.
//!
//! Whisper: before you move anything, be sure you'd notice if it broke.
//!
//! Read README.md first.
//!
//! This code WORKS. It's also awful. Your job: make it clean without
//! changing what it does. Tests first, then tiny refactoring steps.

pub struct TennisGame {
    m_score1: i32,
    m_score2: i32,
    player1_name: String,
    player2_name: String,
}

impl TennisGame {
    pub fn new(player1_name: &str, player2_name: &str) -> Self {
        TennisGame {
            m_score1: 0,
            m_score2: 0,
            player1_name: player1_name.to_string(),
            player2_name: player2_name.to_string(),
        }
    }

    pub fn won_point(&mut self, player_name: &str) {
        if player_name == "player1" {
            self.m_score1 += 1;
        } else {
            self.m_score2 += 1;
        }
    }

    pub fn get_score(&self) -> String {
        let mut score = String::new();
        let mut temp_score;
        if self.m_score1 == self.m_score2 {
            match self.m_score1 {
                0 => score = "Love-All".to_string(),
                1 => score = "Fifteen-All".to_string(),
                2 => score = "Thirty-All".to_string(),
                _ => score = "Deuce".to_string(),
            }
        } else if self.m_score1 >= 4 || self.m_score2 >= 4 {
            let minus_result = self.m_score1 - self.m_score2;
            if minus_result == 1 {
                score = "Advantage player1".to_string();
            } else if minus_result == -1 {
                score = "Advantage player2".to_string();
            } else if minus_result >= 2 {
                score = "Win for player1".to_string();
            } else {
                score = "Win for player2".to_string();
            }
        } else {
            for i in 1..3 {
                if i == 1 {
                    temp_score = self.m_score1;
                } else {
                    score.push('-');
                    temp_score = self.m_score2;
                }
                match temp_score {
                    0 => score.push_str("Love"),
                    1 => score.push_str("Fifteen"),
                    2 => score.push_str("Thirty"),
                    3 => score.push_str("Forty"),
                    _ => {}
                }
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (player1 points, player2 points, expected score)
    const CASES: &[(i32, i32, &str)] = &[
        (0, 0, "Love-All"),
        (1, 1, "Fifteen-All"),
        (2, 2, "Thirty-All"),
        (3, 3, "Deuce"),
        (4, 4, "Deuce"),
        (1, 0, "Fifteen-Love"),
        (0, 1, "Love-Fifteen"),
        (2, 0, "Thirty-Love"),
        (0, 2, "Love-Thirty"),
        (3, 0, "Forty-Love"),
        (0, 3, "Love-Forty"),
        (4, 0, "Win for player1"),
        (0, 4, "Win for player2"),
        (2, 1, "Thirty-Fifteen"),
        (1, 2, "Fifteen-Thirty"),
        (3, 1, "Forty-Fifteen"),
        (1, 3, "Fifteen-Forty"),
        (4, 1, "Win for player1"),
        (1, 4, "Win for player2"),
        (3, 2, "Forty-Thirty"),
        (2, 3, "Thirty-Forty"),
        (4, 3, "Advantage player1"),
        (3, 4, "Advantage player2"),
        // TODO (YOU): this table is NOT a complete safety net yet.
        // Before you refactor anything, add the missing cases. Think about:
        //   - wins straight from 4-2 / 2-4
        //   - advantage and win after a long deuce (5-4, 4-5, 6-4, 4-6, 15-14, 16-14, ...)
        // Then prove the table catches bugs: break the code on purpose
        // (change a number, flip a >=) and check that a test goes red.
    ];

    fn play(p1: i32, p2: i32) -> TennisGame {
        let mut game = TennisGame::new("player1", "player2");
        for _ in 0..p1 {
            game.won_point("player1");
        }
        for _ in 0..p2 {
            game.won_point("player2");
        }
        game
    }

    #[test]
    fn scores_match_the_table() {
        for &(p1, p2, expected) in CASES {
            assert_eq!(play(p1, p2).get_score(), expected, "score {p1}-{p2}");
        }
    }

    #[test]
    fn points_can_arrive_in_any_order() {
        todo!(
            "YOUR TEST: alternate player1/player2 points to reach deuce, \
             then advantage. The table only ever plays all p1 points first!"
        )
    }

    // ---------------- NEW FEATURE (after refactoring!) ----------------
    // The code ignores the real player names. Make it use them.

    #[test]
    #[ignore]
    fn uses_real_player_names() {
        let mut game = TennisGame::new("Serena", "Venus");
        for _ in 0..4 {
            game.won_point("Serena");
        }
        for _ in 0..3 {
            game.won_point("Venus");
        }
        assert_eq!(game.get_score(), "Advantage Serena");
    }

    #[test]
    #[ignore]
    fn second_player_by_name() {
        todo!("YOUR TEST: what if Venus wins? And what should happen for an unknown name?")
    }
}
