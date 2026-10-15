# rustkatas

15 TDD katas in Rust. Some are classics you've probably heard of (FizzBuzz,
Bowling, Gilded Rose), and some are a bit less common but teach something
the others don't (Bank Account for test doubles, Trip Service for breaking
dependencies in legacy code).

The point isn't to solve them. Most of these have a thousand solutions online.
The point is to practise the loop: write a small failing test, make it
pass, clean up, repeat. Doing that over and over on small problems is how it
starts to feel normal on real ones.

There are no solutions in here, on purpose.

## Setup

You need Rust. If you don't have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone and run the first kata:

```bash
git clone https://github.com/yossev/rustkatas.git
cd rustkatas
cargo test -p fizzbuzz
```

It'll fail. That's supposed to happen.

## How a kata works

Every kata lives in `katas/NN-name/` and has two files that matter:

- `README.md` has the rules, a suggested order for the tests, stretch goals
  and some folded hints. Open the hints only when you're stuck.
- `src/lib.rs` has the functions (empty, just `todo!()`) and the tests. There's
  also a little story at the top, mostly for fun, plus a one-line nudge on how
  to think about it.

Only the **first test** in each kata is switched on. The rest have `#[ignore]`
on them. So the routine is:

1. Run the tests. One fails.
2. Make it pass with the simplest code you can get away with. Hard-coding
   the answer is fine at this point.
3. Tidy up if something looks messy.
4. Delete the `#[ignore]` above the next test and go back to step 1.

Some tests are just a name and a `todo!("YOUR TEST: ...")`. Those are yours to
write. Fill in the assertions first, then remove the `#[ignore]`.

A few handy commands:

```bash
cargo test -p bowling                         # one kata
cargo test -p bowling strike                  # only tests with "strike" in the name
cargo test -p bowling -- --include-ignored    # everything, ignored ones too
cargo watch -x "test -p bowling"              # rerun on save (cargo install cargo-watch)
```

## The katas

Roughly easiest to hardest. You don't have to go in order, but the early ones
build habits the later ones lean on.

| # | Kata | What it's really about |
|---|------|------------------------|
| 01 | [FizzBuzz](katas/01-fizzbuzz/README.md) | Getting the rhythm down. Tiny steps, even when it feels silly |
| 02 | [Prime Factors](katas/02-prime-factors/README.md) | Watching an algorithm show up on its own if you don't rush it |
| 03 | [Roman Numerals](katas/03-roman-numerals/README.md) | Noticing when a pile of `if`s is really a lookup table |
| 04 | [String Calculator](katas/04-string-calculator/README.md) | Requirements that change every step. Don't read ahead |
| 05 | [Bowling](katas/05-bowling/README.md) | Changing your design when a test shows it can't work |
| 06 | [Word Wrap](katas/06-word-wrap/README.md) | Getting stuck, and learning that the fix is a smaller test |
| 07 | [Game of Life](katas/07-game-of-life/README.md) | Starting from the smallest rule and building up |
| 08 | [Mars Rover](katas/08-mars-rover/README.md) | Replacing raw `char`s and tuples with real types |
| 09 | [Tennis](katas/09-tennis-refactoring/README.md) | Refactoring ugly code that already works, without breaking it |
| 10 | [Gilded Rose](katas/10-gilded-rose/README.md) | Legacy code with no tests. Pin down the behaviour, then change it |
| 11 | [Diamond](katas/11-diamond/README.md) | Testing properties instead of comparing giant strings |
| 12 | [Minesweeper](katas/12-minesweeper/README.md) | Keeping the logic separate from the parsing |
| 13 | [Bank OCR](katas/13-bank-ocr/README.md) | Working from user stories, one at a time |
| 14 | [Bank Account](katas/14-bank-account/README.md) | Outside-in TDD with a fake clock and a spy printer |
| 15 | [Trip Service](katas/15-trip-service/README.md) | Getting code with hard dependencies under test |

[GUIDANCE.md](GUIDANCE.md) is the longer version: the TDD rules, refactoring
moves, a Rust testing cheat sheet, what to do when you're stuck, and a
checklist to track progress.

## Some advice

- **Use git as an undo button.** Commit every time you're green. If you've
  been red for ten minutes and things are getting worse, `git checkout .` and
  try a smaller step. It's surprisingly freeing.
- **Timebox it.** 30 to 60 minutes per kata. If you run out of time, stop
  anyway and write down what you'd do next.
- **Do them more than once.** The second time through is where most of the
  learning happens. Delete your code and start fresh a few days later. Then
  try it with a constraint, like no `if` statements or no `let mut`.
- **Watch the test fail before you make it pass.** If you never saw it go
  red, you don't actually know it tests anything.

## Credits

None of these katas are mine. They come from people who've been teaching this
stuff for years:

- **Bowling, Prime Factors, Word Wrap**: Robert C. Martin
- **String Calculator**: Roy Osherove
- **Tennis, Gilded Rose** (the refactoring versions): Emily Bache, original
  Gilded Rose by Terry Hughes
- **Bank Account, Trip Service**: Sandro Mancuso
- **Diamond**: Seb Rose
- **Bank OCR**: the Coding Dojo kata catalogue
- **Game of Life**: John Conway, by way of every Code Retreat ever
- **Mars Rover, Roman Numerals, FizzBuzz, Minesweeper**: floating around the
  kata world for ages

The Rust versions, stubs and test layouts are written for this repo.

## License

MIT. Use it, fork it, run it in a workshop. See [LICENSE](LICENSE).
