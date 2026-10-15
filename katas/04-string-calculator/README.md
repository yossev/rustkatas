# Kata 04 — String Calculator

**Run:** `cargo test -p string-calculator` · **Time:** 45–60 min · **Difficulty:** ★★☆☆☆

## Why this kata

Roy Osherove's kata simulates real work: **requirements arrive one at a time**
and you can't see the future. The golden rule: **do not read ahead.** Do
step 1, make it clean, *then* scroll down to step 2. You'll practise
designing only for what's needed now, and refactoring when the next
requirement pushes on your design.

It's also great Rust practice for `&str` parsing and error handling.

> Heads-up: the test names in `src/lib.rs` hint at later steps. Try not to scroll past the one you are working on.

## The API

```rust
pub fn add(numbers: &str) -> Result<i32, CalcError>
```

---

## Steps — one at a time, no peeking!

### Step 1
An empty string returns `0`. `"1"` returns `1`. `"1,2"` returns `3`.

<details><summary>Step 2 (open when step 1 is green and refactored)</summary>

Handle **any amount** of numbers: `"1,2,3,4,5"` → `15`.
</details>

<details><summary>Step 3</summary>

Allow **newlines** as separators too: `"1\n2,3"` → `6`.
(`"1,\n"` is not valid input — you don't need to handle it. Or do, and return
`CalcError::InvalidInput`.)
</details>

<details><summary>Step 4</summary>

Support a **custom delimiter**. The input may start with `//<delim>\n`:
`"//;\n1;2"` → `3`. The plain format from earlier steps must keep working.
</details>

<details><summary>Step 5</summary>

**Negative numbers** are an error. Return `Err(CalcError::Negatives(vec))`
containing **all** the negatives, in order: `"1,-2,3,-4"` → `Err(Negatives(vec![-2, -4]))`.
</details>

<details><summary>Step 6</summary>

Numbers **bigger than 1000 are ignored**: `"2,1001"` → `2`. (1000 itself counts.)
</details>

<details><summary>Step 7</summary>

Delimiters can be **any length** with this format: `"//[***]\n1***2***3"` → `6`.
</details>

<details><summary>Step 8</summary>

Allow **multiple delimiters**: `"//[*][%]\n1*2%3"` → `6`.
Make sure multi-char multiple delimiters work too: `"//[**][%%]\n1**2%%3"` → `6`.
</details>

---

## Watch for

- Around step 4 your `add` will get long. Extract a function that splits
  "header" from "body" (`fn parse_delimiters(input: &str) -> (Vec<String>, &str)`).
- Around step 5, use `?` and iterators: parse all numbers into a `Vec<i32>`
  first, *then* validate, *then* sum. Three small stages beat one big loop.

## Stretch

- Use the `?` operator with `str::parse::<i32>()` by adding
  `impl From<std::num::ParseIntError> for CalcError`.
- Implement `std::fmt::Display` for `CalcError` so the message reads
  `"negatives not allowed: -2, -4"`.

<details><summary>Hint: splitting on several delimiters</summary>

`str::split` takes one pattern. Simplest trick: replace every delimiter with
`","` first, then split on `","`. Or split on a closure:
`s.split(|c| c == ',' || c == '\n')` (single chars only).
</details>
