# Kata 08 — Mars Rover

**Run:** `cargo test -p mars-rover` · **Time:** 60–90 min · **Difficulty:** ★★★☆☆

## Why this kata

Requirements arrive in **three stages**, each one pushing on your design.
It's the perfect kata for practising **primitive obsession → types**: you'll
start with a `char` for direction and `(i32, i32)` for position, and the
refactor step will beg you to introduce `enum Direction` and `struct Position`.

## Rules

A rover lands on a **10×10 grid** at `0:0` facing **North**. It receives a
string of commands:

- `L` / `R` — turn 90° left / right (stays on the same square)
- `M` — move forward one square in the direction it's facing

`execute` returns the final position as `"x:y:D"` where `D` is `N`, `E`, `S`
or `W`. North increases `y`, East increases `x`.

```
"MMRMMLM"     -> "2:3:N"
"RMMLM"       -> "2:1:N"
```

## Stage 1 — turning and moving
Turn in place (`"R" -> "0:0:E"`, `"RRRR" -> "0:0:N"`), then move.

## Stage 2 — the planet is round
The grid **wraps around**: moving North from `y = 9` takes you to `y = 0`;
moving West from `x = 0` takes you to `x = 9`.

```
"MMMMMMMMMM" -> "0:0:N"
"LM"         -> "9:0:W"
```

## Stage 3 — obstacles
The grid may contain obstacles. If a move would hit one, the rover **stops
at the last valid position**, ignores the rest of the commands, and reports
with an `O:` prefix:

```
obstacle at (0, 3):  "MMMM" -> "O:0:2:N"
```

## Steps

Given tests cover the first examples; you write the rest. Commit after each stage.

**Refactor checkpoints:**
- After stage 1: have you got `turn_left`/`turn_right` as methods on a
  `Direction` enum? A `match` with 4 arms each, no `if`s.
- After stage 2: is the grid size (10) a named constant? Is wrapping one
  function (`rem_euclid` is your friend for negatives)?
- After stage 3: is `execute` still short? Is "move" a function that returns
  `Option<Position>` (or `Result`) when blocked?

## Stretch

- Return a proper type from `execute` (e.g. `Status { position, direction, blocked: bool }`)
  and implement `Display` to get the string.
- Make grid size configurable: `MarsRover::new(Grid::new(5, 5))`.
- Invalid commands (`"MXM"`): decide — ignore, error, or panic? Test it.

<details><summary>Hint: Direction</summary>

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { North, East, South, West }
impl Direction {
    fn right(self) -> Self { match self { Direction::North => Direction::East, /* ... */ } }
    fn as_char(self) -> char { /* ... */ }
}
```
</details>

<details><summary>Hint: wrapping</summary>

`(-1i32).rem_euclid(10) == 9`, while `-1 % 10 == -1`.
</details>
