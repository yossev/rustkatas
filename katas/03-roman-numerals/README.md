# Kata 03 — Roman Numerals

**Run:** `cargo test -p roman-numerals` · **Time:** 30–45 min · **Difficulty:** ★★☆☆☆

## Why this kata

You'll start with `if`s, then a loop, then notice the code wants to become a
**table of data**. Learning to see "this logic is really data" is one of the
most useful refactoring instincts you can build.

## Rules

| Symbol | I | V | X | L | C | D | M |
|--------|---|---|---|---|---|---|---|
| Value  | 1 | 5 | 10 | 50 | 100 | 500 | 1000 |

- Symbols are written largest to smallest and added: `VIII = 8`.
- Subtractive pairs: `IV = 4`, `IX = 9`, `XL = 40`, `XC = 90`, `CD = 400`, `CM = 900`.
- Valid range: 1 to 3999.

## Stage 1 — `to_roman(n: u32) -> String`

Suggested test order (some given, some yours):

`1 → I`, `2 → II`, `3 → III`, `5 → V`, `4 → IV`, `10 → X`, `9 → IX`,
`40 → XL`, `1994 → MCMXCIV`, `3999 → MMMCMXCIX`.

After `III` you'll probably have a loop. After `V` and `X` you'll have
duplication. **Stop and refactor** before going further.

## Stage 2 — `from_roman(s: &str) -> Result<u32, RomanError>`

Now go the other way. Return errors for bad input:

- `""` → `RomanError::Empty`
- `"ABC"` → `RomanError::InvalidSymbol('A')`
- (stretch) `"IIII"`, `"VV"`, `"IC"` → `RomanError::Malformed`

## Stage 3 — a property test

Write **one** test that checks `from_roman(&to_roman(n)) == Ok(n)` for every
`n` in `1..=3999`. This single test is worth more than 50 examples. (This is
the idea behind property-based testing — see the `proptest` crate later.)

## Stretch

- Make `to_roman` return `Option<String>` (or `Result`) and handle `0` and `4000`.
- Implement it as `impl std::fmt::Display for Roman(u32)` and `impl FromStr for Roman`.

<details><summary>Hint: the table</summary>

```rust
const NUMERALS: [(u32, &str); 13] = [
    (1000, "M"), (900, "CM"), (500, "D"), /* ... */ (1, "I"),
];
```
Walk the table; while the value fits, push the symbol and subtract.
</details>

<details><summary>Hint: parsing</summary>

Map each char to its value. If a value is smaller than the one *after* it,
subtract it, otherwise add it. `chars().zip(chars().skip(1))` or
`windows(2)` over a `Vec<u32>` helps.
</details>
