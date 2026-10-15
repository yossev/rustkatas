# Kata 11 — Diamond

**Run:** `cargo test -p diamond` · **Time:** 30–45 min · **Difficulty:** ★★☆☆☆

## Why this kata

Comparing the whole output gets awkward fast here. The test for `'E'` is a
9×9 block of spaces that's tedious to type and tells you nothing when it
fails. This kata (made popular by Seb Rose and Nat Pryce) pushes you to
test **properties** instead: "it's square", "it's symmetric", "row k
contains the k-th letter". Each property is a small, honest test, and
together they pin the shape down.

## Rules

Given a letter, print a diamond that starts with `A` and has the given letter
at its widest point.

```
diamond('A')     diamond('B')     diamond('C')
A                 A               "  A  "
                 B B              " B B "
                  A               "C   C"
                                  " B B "
                                  "  A  "
```

- Rows are separated by `\n`, with no trailing newline.
- Every row has the same width (pad with spaces, **including trailing spaces**),
  so the output is a square.

## Steps

1. `'A'` → `"A"` (given).
2. Try writing `'B'` as a full-string test. Then `'C'`. Notice how long the
   jump from B to C feels.
3. Switch to property tests (some given, some yours):
   - height == width == `2 * (letter index) + 1`
   - first and last row contain only `A` (and spaces)
   - the diamond is the same upside-down (vertical symmetry)
   - each row reads the same backwards (horizontal symmetry)
   - row `i` from the top contains exactly one letter kind, the `i`-th one
   - every row has exactly 2 letters except the `A` rows, which have 1
4. Run each property for every letter `'A'..='Z'`. One loop, 26 checks.

## Stretch

- Return `Vec<String>` instead of `String`. Does it make the tests nicer?
- Write it without building a 2D grid: work out each row directly from its
  letter (outer padding, letter, inner padding, letter).

<details><summary>Hint: the top half is enough</summary>

Build the rows from `A` up to the given letter, then append the same rows
reversed, minus the middle one. `iter().rev().skip(1)`.
</details>
