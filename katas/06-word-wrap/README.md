# Kata 06 — Word Wrap

**Run:** `cargo test -p word-wrap` · **Time:** 45 min · **Difficulty:** ★★★☆☆

## Why this kata

This kata is famous for **getting you stuck**. If you pick the "obvious"
tests in the "obvious" order, at some point you'll need a huge change to make
the next test pass. The lesson (from Uncle Bob's *The Craftsman*): when that
happens, **back out and pick a simpler test**. Test *order* is a design skill.

## Rules

`wrap(text, width)` inserts `\n` so that no line is longer than `width`.

- Break at spaces when possible (the space is replaced by the newline).
- If a single word is longer than `width`, break the word itself.
- Assume `width >= 1` and the text is plain ASCII single-spaced words
  (stretch: handle Unicode).

```
wrap("four score and seven years ago", 7)
=> "four\nscore\nand\nseven\nyears\nago"

wrap("four score and seven years ago", 9)
=> "four\nscore and\nseven\nyears ago"
```

## Steps

The tests in `lib.rs` are in a **good** order: degenerate cases first, then
long words with no spaces, then spaces. Follow them. Some are yours to write.

1. `""` → `""`
2. `"x"` width 1 → `"x"`
3. `"xx"` width 1 → `"x\nx"`
4. `"xxx"` width 1 → `"x\nx\nx"` (recursion or a loop appears)
5. `"x x"` width 1 → `"x\nx"` (now spaces)
6. `"x xx"` width 3 → `"x\nxx"`
7. The "four score" examples.

## The trap (try it on round 2!)

Start instead with: `"word word"` width 4, then `"word word word"` width 6…
You'll end up writing the whole algorithm in one go to pass a test. Notice the
feeling — that's the sign your test is too big.

## Stretch

- `wrap` currently returns `String`. Make it return `Vec<&str>` (borrowing from
  the input) — no allocation per line. Lifetimes practice!
- Unicode: `"héllo"` width 2. Bytes vs `char`s vs grapheme clusters.
- Multiple spaces / leading spaces: decide the behaviour, write tests.

<details><summary>Hint: recursive shape</summary>

If `text.len() <= width`, return it. Otherwise find where to break (the last
space at or before `width`, or `width` itself if there is no space), and
return `head + "\n" + wrap(tail, width)`.
</details>

<details><summary>Hint: str::rfind</summary>

`text[..=width].rfind(' ')` finds the last space that would still fit.
Careful with the slice bounds — byte indexes on `&str` panic on non-char
boundaries.
</details>
