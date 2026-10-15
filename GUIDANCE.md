# Rust TDD Katas — Guidance

Fifteen classic katas, ordered from warm-up to legacy-code rescue. Each one is a
small Cargo crate with:

- a `README.md` with the rules, the step-by-step plan, stretch goals and hidden hints
- a `src/lib.rs` with **stubs** (`todo!()`) and a test module
- some tests **written for you**, some tests **with only a name and a TODO** for you to fill in

No solutions are included on purpose. The value is in the struggle.

---

## 1. The loop

```
        ┌──────────────┐
        │  RED         │  Write ONE small test. Run it. Watch it fail
        │              │  (and check it fails for the reason you expect).
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │  GREEN       │  Write the SIMPLEST code that passes.
        │              │  Hard-coding is allowed. Ugly is allowed.
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │  REFACTOR    │  Tests are green: now clean up code AND tests.
        │              │  Run tests after every small change.
        └──────┬───────┘
               └──────────► back to RED
```

### Uncle Bob's three laws

1. You may not write production code unless it is to make a failing test pass.
2. You may not write more of a test than is sufficient to fail (and in Rust,
   **not compiling counts as failing**).
3. You may not write more production code than is sufficient to pass the one failing test.

### Habits that make TDD actually work

- **Tiny steps.** If you have been red for more than ~5 minutes, your step was
  too big. Revert (`git checkout .`) and take a smaller one.
- **Watch it fail first.** A test you've never seen fail might be testing nothing.
- **Fake it till you make it.** Return a constant. Let the *next* test force
  you to generalise ("triangulation").
- **Refactor only on green.** Never refactor and add behaviour at the same time.
- **Tests are code too.** Remove duplication in tests, name them like
  sentences: `three_is_fizz`, `strike_adds_next_two_rolls_as_bonus`.
- **Transformation Priority Premise** — when going from red to green, prefer
  the simplest transformation:
  `{} → nil → constant → variable → add computation → split flow (if) →
  array/collection → loop/iteration (while) → recursion → ...`.
  If you jump straight to a complex one, you often skip the elegant solution.

---

## 2. How this repo works

It's a Cargo workspace. Each kata is a crate.

```bash
# run one kata's tests (package names are in each Cargo.toml)
cargo test -p fizzbuzz

# include the tests marked #[ignore] (i.e. "everything")
cargo test -p fizzbuzz -- --include-ignored

# run only tests matching a name
cargo test -p bowling strike

# see println! output from tests
cargo test -p bowling -- --nocapture
```

### The `#[ignore]` workflow

Only the **first** test in each kata is active. All the others are marked
`#[ignore]`. Your loop is:

1. Make the active test pass.
2. Delete the `#[ignore]` from the **next** test. Run. Red.
3. Green. Refactor. Repeat.

Tests that look like this are **yours to write**:

```rust
#[test]
#[ignore]
fn five_is_buzz() {
    todo!("YOUR TEST: ...")
}
```

Replace the `todo!` with real assertions *before* you remove `#[ignore]`.
(If you un-ignore it without writing it, it fails with "not yet implemented" — a
nice reminder.)

Each stub file starts with `#![allow(unused)]` so the compiler doesn't nag you
about unused parameters at the start. Delete that line when you finish the
kata and fix any warnings — that's part of the refactor step.

### Auto-run tests on save (highly recommended)

```bash
cargo install cargo-watch
cargo watch -x "test -p fizzbuzz"
```

With Neovim + rust-analyzer you can also run a single test from the code lens
above each `#[test]`.

### Use git as your undo button

```bash
git init && git add -A && git commit -m "start"
```

Commit on every green. If you get lost, `git checkout .` takes you back to
the last green state. That freedom is what makes tiny steps feel safe.

---

## 3. The katas

| #  | Kata | Package | TDD skill you practise | Rust skill you practise |
|----|------|---------|------------------------|-------------------------|
| 01 | [FizzBuzz](katas/01-fizzbuzz/README.md) | `fizzbuzz` | The red/green/refactor rhythm, fake it | `String`, `match` on tuples, iterators |
| 02 | [Prime Factors](katas/02-prime-factors/README.md) | `prime-factors` | Triangulation, letting the algorithm emerge | `Vec`, `while` loops |
| 03 | [Roman Numerals](katas/03-roman-numerals/README.md) | `roman-numerals` | Spotting patterns → data-driven code | Arrays of tuples, `Result`, custom errors |
| 04 | [String Calculator](katas/04-string-calculator/README.md) | `string-calculator` | Incremental requirements, not reading ahead | `&str` parsing, `split`, `Result`, `?` |
| 05 | [Bowling Game](katas/05-bowling/README.md) | `bowling` | Test helpers, designing via tests | structs, `&mut self`, indices |
| 06 | [Word Wrap](katas/06-word-wrap/README.md) | `word-wrap` | Getting **stuck** and choosing a better test | slices, `char` vs byte, recursion |
| 07 | [Game of Life](katas/07-game-of-life/README.md) | `game-of-life` | Outside-in vs inside-out, testing pure rules | 2D data, `Display`, `FromStr` |
| 08 | [Mars Rover](katas/08-mars-rover/README.md) | `mars-rover` | Evolving a design with new requirements | enums with methods, `Copy` types, `HashSet` |
| 09 | [Tennis](katas/09-tennis-refactoring/README.md) | `tennis` | **Refactoring** under a test safety net | removing mutable state, `match` guards |
| 10 | [Gilded Rose](katas/10-gilded-rose/README.md) | `gilded-rose` | **Legacy code**: characterisation tests, golden master | traits / enums for polymorphism |
| 11 | [Diamond](katas/11-diamond/README.md) | `diamond` | Testing **properties** instead of whole outputs | `char` ranges, `String` building |
| 12 | [Minesweeper](katas/12-minesweeper/README.md) | `minesweeper` | Pure core vs. parsing edges, revisiting an old design | 2D indexing, `lines()`, parsing |
| 13 | [Bank OCR](katas/13-bank-ocr/README.md) | `bank-ocr` | Working from user stories, small building blocks | string slicing, `chunks`, `match` on `&str` |
| 14 | [Bank Account](katas/14-bank-account/README.md) | `bank-account` | **Outside-in** TDD, test doubles (fakes and spies) | traits, `&dyn Trait`, `RefCell` |
| 15 | [Trip Service](katas/15-trip-service/README.md) | `trip-service` | **Breaking dependencies** in legacy code, seams | trait objects, dependency injection |

**Suggested pace:** one kata per session, 30–60 minutes, timeboxed. When the
time is up, stop, even if you aren't done, and write down what you learned.

---

## 4. Practise like a musician

A kata isn't a puzzle you solve once. It's a scale you play over and over.

- **Round 1:** just finish it with TDD.
- **Round 2 (a few days later):** delete your code (`git stash` or a new branch),
  do it again. Compare. You'll be faster, and the design will be different.
- **Round 3:** add a constraint:
  - *No `if` statements* (use `match`, lookup tables, polymorphism).
  - *No mutable variables* (`let mut` banned; iterators only).
  - *Max 5 lines per function.*
  - *Baby steps:* set a 2-minute timer; if you're not green when it rings, `git checkout .`.
  - *Ping-pong* with a friend (or with Claude): one writes the test, the other makes it pass.

---

## 5. Rust testing toolbox

```rust
assert_eq!(actual, expected);               // equality with diff in failure msg
assert_ne!(a, b);
assert!(cond, "custom message {}", x);      // boolean
assert!(matches!(result, Err(CalcError::Negatives(_))));

#[test]
#[should_panic(expected = "out of range")]  // test that code panics
fn panics() { ... }

#[test]
fn returns_result() -> Result<(), String> { // `?` inside tests
    let n = "5".parse::<i32>().map_err(|e| e.to_string())?;
    assert_eq!(n, 5);
    Ok(())
}

// Table-driven tests: great for katas with many examples
#[test]
fn many_cases() {
    let cases = [(1, "I"), (4, "IV"), (9, "IX")];
    for (input, expected) in cases {
        assert_eq!(to_roman(input), expected, "input was {input}");
    }
}
```

Test helpers are just functions inside `mod tests`. Use them freely to keep
tests readable (`fn roll_many(game: &mut Game, n: usize, pins: u32)`).

---

## 6. Refactoring moves to reach for

On green, look for these smells and apply a move:

| Smell | Move |
|-------|------|
| Duplicated code | Extract function / extract constant |
| Magic numbers / strings | Name them (`const`, enum variants) |
| Long `if/else` chain on the same value | `match`, or a lookup table |
| A function doing two things | Split it |
| `bool` flags as parameters | Enum |
| Primitive obsession (`(i32, i32, char)` everywhere) | Introduce a type (`struct Position`, `enum Direction`) |
| Type-switching on a `name` string | Enum or trait with one impl per kind |
| Mutable index-juggling | Iterator adapters (`windows`, `chunks`, `zip`, `fold`) |

Always: one move → run tests → next move.

---

## 7. When you're stuck

1. Is your current test too big? Write a smaller one first.
2. Is the test order wrong? (Word Wrap is designed to teach you this one.)
3. Read the **Hints** section of the kata README. Each hint is folded so you
   only open what you need.
4. Run `cargo clippy -p <kata>`: it often suggests more idiomatic Rust.
5. Ask for a review: paste your code and tests and ask *"what would you
   refactor next?"* rather than *"what's the solution?"*.

---

## 8. Progress tracker

Tick these off as you go (edit this file!).

- [ ] 01 FizzBuzz — round 1
- [ ] 02 Prime Factors — round 1
- [ ] 03 Roman Numerals — round 1
- [ ] 04 String Calculator — round 1
- [ ] 05 Bowling — round 1
- [ ] 06 Word Wrap — round 1
- [ ] 07 Game of Life — round 1
- [ ] 08 Mars Rover — round 1
- [ ] 09 Tennis — refactored
- [ ] 10 Gilded Rose — refactored + Conjured items added
- [ ] 11 Diamond — round 1
- [ ] 12 Minesweeper — round 1 (stage 2 too)
- [ ] 13 Bank OCR — stories 1–3
- [ ] 14 Bank Account — acceptance test green
- [ ] 15 Trip Service — under test and refactored
- [ ] Any kata — round 2 from scratch
- [ ] Any kata — round 3 with a constraint

### After each kata, answer briefly (in the kata's README or a journal)

- Which test forced the biggest change in the code? Why?
- Where did I write more code than the test required?
- What did I refactor? What did I *want* to refactor but didn't?
- What would I do differently next time?
