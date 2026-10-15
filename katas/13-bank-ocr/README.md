# Kata 13 — Bank OCR

**Run:** `cargo test -p bank-ocr` · **Time:** 60–90 min · **Difficulty:** ★★★☆☆

## Why this kata

A classic from the Coding Dojo, written as **user stories**, just like
real tickets. It's mostly string slicing and parsing, which Rust makes you
think hard about (`&str`, byte indexes, `chunks`). Each story builds on the
last, so you'll find out whether your design from story 1 holds up.

## The format

A scanning machine reads paper forms and produces "entries". Each entry is
**4 lines**: 3 lines of digits drawn with pipes and underscores, then a blank
line. Each line is 27 characters, so each of the 9 digits is 3 characters wide.

```
    _  _     _  _  _  _  _ 
  | _| _||_||_ |_   ||_||_|
  ||_  _|  | _||_|  ||_| _|

```
→ `123456789`

All ten digits:

```
 _     _  _     _  _  _  _  _ 
| |  | _| _||_||_ |_   ||_||_|
|_|  ||_  _|  | _||_|  ||_| _|
```

## User story 1 — parse

`parse_entry(entry)` turns one entry into a 9-character string of digits.

## User story 2 — checksum

A valid account number satisfies:

```
(d1 + 2*d2 + 3*d3 + ... + 9*d9) mod 11 == 0
```

where **d1 is the rightmost digit**. `is_valid("345882865")` is `true`,
`is_valid("664371495")` is `false`.

## User story 3 — the report

Unreadable digits become `?`. `report(entry)` returns:

```
457508000          valid
664371495 ERR      checksum failed
86110??36 ILL      contains unreadable digits
```

## Steps

1. Start with **one digit**. Write `parse_digit(&[&str; 3]) -> char` (or
   something like it) and test it for `0` and `1` before touching entries.
2. Then `"000000000"`, `"111111111"`, all the way to `"123456789"`.
3. Story 2 is pure arithmetic. Write it as a separate function with its own tests.
4. Story 3 glues stories 1 and 2 together. Its tests should be short because
   the hard parts are already covered.

## Stretch — user story 4 (hard)

Sometimes the scanner misses a single pipe or underscore. If an account is
`ERR` or `ILL`, try every way of **adding or removing one** `|` or `_` per
digit. If exactly one alternative is valid, use it. If several are valid,
output `AMB ['...', '...']`. Example: `"111111111"` → `"711111111"`.

<details><summary>Hint: slicing a digit out</summary>

Digit `i` (0-based) on each line is `&line[i * 3..i * 3 + 3]`. Watch out:
lines might be shorter than 27 characters if trailing spaces got trimmed.
Pad them, or check the length.
</details>

<details><summary>Hint: matching patterns</summary>

Join the three 3-char pieces into one 9-char string and `match` on it:
`" _ | ||_|" => '0'`. Ten arms, very readable.
</details>
