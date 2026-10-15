# Kata 05 — Bowling Game

**Run:** `cargo test -p bowling` · **Time:** 45–60 min · **Difficulty:** ★★★☆☆

## Why this kata

The most famous TDD kata (Robert C. Martin). It teaches you to:

- **resist up-front design** (you'll be tempted to create `Frame`, `Roll`,
  `TenthFrame` structs — the tests may show you don't need them),
- write **test helpers** so tests stay readable,
- spot when a test reveals a **design flaw** and refactor *before* making it pass.

## Scoring rules

A game has **10 frames**. In each frame the player gets up to two rolls to
knock down 10 pins.

- **Open frame:** score = pins knocked down in the frame.
- **Spare** (all 10 pins in two rolls): 10 + the **next one** roll.
- **Strike** (all 10 pins in the first roll): 10 + the **next two** rolls.
  The frame ends after one roll.
- **10th frame:** a spare earns one bonus roll, a strike earns two. Max 3 rolls
  in frame 10.

Examples:
- 20 gutter balls → `0`
- 20 rolls of 1 pin → `20`
- `5, 5, 3, then 17 zeros` → `(5+5+3) + 3 = 16`
- `10, 3, 4, then 16 zeros` → `(10+3+4) + (3+4) = 24`
- 12 strikes → `300` (the perfect game)

## The API

```rust
let mut game = Game::new();
game.roll(pins);   // called once per ball
game.score();      // only called at the end of a complete game
```

## Steps

1. `gutter_game` (given) — make it pass. Then `all_ones` (given).
   **Refactor the tests:** extract `roll_many` (provided — use it!).
2. `one_spare` — **you write it.** When you try to pass it, you'll realise
   the current design (accumulating a score in `roll`) can't see the *next*
   roll. **Don't hack it.** Comment out the test, refactor the design so `roll`
   just *stores* rolls and `score` computes, get back to green, then re-enable
   the test. This is the key lesson of the kata.
3. `one_strike` — you write it.
4. `perfect_game` — you write it. Does it pass for free? Why?
5. Write any test you think is missing (all spares with a final 5? a
   9-pin-then-miss game, `9,0` × 10 → `90`?).

## Watch for

The final `score()` usually looks like a loop over **10 frames** with an index
into the rolls. Extract `is_strike(i)`, `is_spare(i)`, `strike_bonus(i)`,
`spare_bonus(i)` until the loop reads like the rules above.

## Stretch — validation

Change `roll` to return `Result<(), BowlingError>` and reject:

- more than 10 pins in a roll,
- more than 10 pins in a frame (`7` then `5`),
- rolling after the game is over,
- calling `score()` on an incomplete game (`score() -> Option<u32>`).

This is much harder and forces you to track frames. Do it test-first.

<details><summary>Hint: the design change at step 2</summary>

```rust
pub struct Game { rolls: Vec<u32> }
// roll() pushes. score() walks the rolls, frame by frame, with an index
// that jumps by 1 for a strike and by 2 otherwise.
```
</details>
