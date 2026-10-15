# Kata 01 — FizzBuzz

**Run:** `cargo test -p fizzbuzz` · **Time:** 20–30 min · **Difficulty:** ★☆☆☆☆

## Why this kata

Everyone knows the answer, which is exactly why it's good: you can focus 100%
on the *rhythm* of red → green → refactor without the problem distracting you.
Practise being disciplined and almost silly about small steps.

## Rules

Write `fizzbuzz(n)` that returns:

- `"Fizz"` if `n` is divisible by 3
- `"Buzz"` if `n` is divisible by 5
- `"FizzBuzz"` if divisible by both
- the number itself as a string otherwise

Then `fizzbuzz_up_to(n)` returns the results for `1..=n`.

## Steps

1. `cargo test -p fizzbuzz` → red (`not yet implemented`). Make it green with
   the dumbest code possible: `"1".to_string()`. Yes, really.
2. Un-ignore `two_is_just_the_number`. Now you're *forced* to generalise.
3. Un-ignore `three_is_fizz`, then `six_is_fizz` (triangulation: one example
   can be faked, two make you write the rule).
4. Write the TODO tests yourself: `five_is_buzz`, `ten_is_buzz`,
   `fifteen_is_fizzbuzz`, `thirty_is_fizzbuzz`.
5. Write `fizzbuzz_up_to` test-first.
6. **Refactor:** is your code an `if/else if` chain? Try a `match` on
   `(n % 3, n % 5)`. Which reads better?

## Stage 2 — requirements change (do this after green!)

> A number is also "Fizz" if it **contains** a 3 and "Buzz" if it **contains** a 5.
> So `13 → "Fizz"`, `52 → "Buzz"`, `35 → "FizzBuzz"` (divisible by 5, contains 3).

Write tests for these first. Notice how easy or hard your current design makes
the change — that's the point of stage 2s.

## Stretch

- Do it again with **no `if` and no `match`** (hint: think about
  `Option<&str>` and combining them).
- Make the rules configurable: `fizzbuzz_with(n, &[(3, "Fizz"), (5, "Buzz"), (7, "Bazz")])`.

<details><summary>Hint: match on a tuple</summary>

```rust
match (n % 3, n % 5) {
    (0, 0) => ...,
    (0, _) => ...,
    ...
}
```
</details>

<details><summary>Hint: fizzbuzz_up_to with iterators</summary>

`(1..=n).map(fizzbuzz).collect()` — a function name can be passed directly to `map`.
</details>
