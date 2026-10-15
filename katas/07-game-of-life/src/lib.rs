//! Kata 07 — Game of Life
//!
//! Under a microscope, a biologist watches a strange colony on a grid. Each
//! cell only looks at its eight neighbours. Too few, and it dies of
//! loneliness; too many, and it's crowded out; just enough, and new life
//! appears. From these tiny rules, patterns blink, settle and glide across
//! the dish. The biologist wants a simulator, and they want to trust it.
//!
//! Whisper: one cell's whole world fits in a single question.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// The rules of life for a single cell.
pub fn next_state(alive: bool, live_neighbours: usize) -> bool {
    todo!("make the first test pass")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    // You decide. A Vec<Vec<bool>>? A flat Vec<bool> + width? A HashSet?
}

impl World {
    /// Parse `#` (alive) / `.` (dead), rows separated by '\n'.
    pub fn parse(s: &str) -> World {
        todo!()
    }

    pub fn is_alive(&self, row: usize, col: usize) -> bool {
        todo!()
    }

    pub fn live_neighbours(&self, row: usize, col: usize) -> usize {
        todo!()
    }

    pub fn render(&self) -> String {
        todo!()
    }

    pub fn tick(&self) -> World {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ================= 1. the rules =================

    #[test]
    fn live_cell_with_fewer_than_two_neighbours_dies() {
        assert!(!next_state(true, 0));
        assert!(!next_state(true, 1));
    }

    #[test]
    #[ignore]
    fn live_cell_with_two_or_three_neighbours_survives() {
        assert!(next_state(true, 2));
        assert!(next_state(true, 3));
    }

    #[test]
    #[ignore]
    fn live_cell_with_more_than_three_neighbours_dies() {
        todo!("YOUR TEST: check 4 and 8 (and maybe everything in between)")
    }

    #[test]
    #[ignore]
    fn dead_cell_with_exactly_three_neighbours_comes_alive() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn dead_cell_otherwise_stays_dead() {
        todo!("YOUR TEST: every count 0..=8 except 3")
    }

    // ================= 2. parsing =================

    #[test]
    #[ignore]
    fn parse_reads_live_and_dead_cells() {
        let world = World::parse("#.\n.#");
        assert!(world.is_alive(0, 0));
        assert!(!world.is_alive(0, 1));
        assert!(!world.is_alive(1, 0));
        assert!(world.is_alive(1, 1));
    }

    // ================= 3. neighbours =================

    #[test]
    #[ignore]
    fn counts_neighbours_in_the_middle() {
        let world = World::parse("###\n.#.\n#..");
        assert_eq!(world.live_neighbours(1, 1), 4);
    }

    #[test]
    #[ignore]
    fn a_cell_is_not_its_own_neighbour() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn counts_neighbours_in_a_corner_without_panicking() {
        todo!("YOUR TEST: e.g. (0, 0) in a 3x3 world")
    }

    // ================= 4. rendering =================

    #[test]
    #[ignore]
    fn render_is_the_inverse_of_parse() {
        let text = "#..\n.#.\n..#";
        assert_eq!(World::parse(text).render(), text);
    }

    // ================= 5. ticking =================

    #[test]
    #[ignore]
    fn a_lonely_cell_dies() {
        todo!("YOUR TEST: \"...\\n.#.\\n...\" -> all dead")
    }

    #[test]
    #[ignore]
    fn block_is_stable() {
        let block = "....\n.##.\n.##.\n....";
        assert_eq!(World::parse(block).tick().render(), block);
    }

    #[test]
    #[ignore]
    fn blinker_oscillates() {
        let vertical = ".....\n..#..\n..#..\n..#..\n.....";
        let horizontal = ".....\n.....\n.###.\n.....\n.....";
        todo!("YOUR TEST: vertical -> horizontal -> vertical")
    }
}
