# Kata 02 — Prime Factors

**Run:** `cargo test -p prime-factors` · **Time:** 20–30 min · **Difficulty:** ★☆☆☆☆

## Why this kata

Robert C. Martin's favourite demonstration of TDD. If you follow the tests in
order and make **only the smallest change** each time, a beautiful 5-line
algorithm *emerges* — you never sit down and "design" it. This kata teaches you
to trust that process.

## Rules

`prime_factors(n)` returns the prime factors of `n` in ascending order.

| n | result |
|---|--------|
| 1 | `[]` |
| 2 | `[2]` |
| 3 | `[3]` |
| 4 | `[2, 2]` |
| 6 | `[2, 3]` |
| 8 | `[2, 2, 2]` |
| 9 | `[3, 3]` |

## Steps

Go strictly in this order — it matters.

1. `1 → []` (given, active)
2. `2 → [2]` (given)
3. `3 → [3]` (given) — can you pass this *without* adding an `if`?
4. `4 → [2, 2]` — **you write it.** Smallest change: an `if` that divides by 2.
5. `6 → [2, 3]` — you write it.
6. `8 → [2, 2, 2]` — you write it. Watch an `if` turn into a `while`.
7. `9 → [3, 3]` — you write it. A second `while` appears… and then you notice
   the two loops are the same loop over a divisor.
8. Add one big test: `2*2*3*3*5*7*11*13`.

## Watch for

After step 7, refactor until it looks like *one* outer loop over a divisor and
*one* inner loop. The final code should be tiny. If yours is long, go back and
take smaller steps.

## Stretch

- Rewrite it with no `let mut` (hint: recursion, or `std::iter::successors`).
- What happens with `prime_factors(0)`? Decide, then write a test for it.
- Performance: only check divisors up to `√n`. Write a test with a big prime
  like `1_000_000_007` first so you *feel* the problem.

<details><summary>Hint: the if → while transformation</summary>

When a test needs "do this once" to become "do this repeatedly", the smallest
change is often to literally replace `if` with `while`.
</details>
