# Kata 12 — Minesweeper

**Run:** `cargo test -p minesweeper` · **Time:** 45 min · **Difficulty:** ★★☆☆☆

## Why this kata

It looks like Game of Life's little sibling, and that's on purpose: you get
to **reuse an idea** (counting neighbours) and see whether you'd now design it
better. Stage 2 adds a parsing layer around the core logic, so you practise
keeping the **pure core** separate from the **input/output edges**.

## Rules — stage 1

`annotate(field)` takes a field where `*` is a mine and `.` is a safe square,
and returns it with every `.` replaced by the number of mines touching it
(including diagonals).

```
*...        *100
....   →    2210
.*..        1*10
....        1110
```

Mines stay as `*`. Rows are separated by `\n`.

## Rules — stage 2 (the original contest format)

The input now holds several fields. Each one starts with a header line
`rows cols`, and the input ends with `0 0`.

```
4 4
*...
....
.*..
....
3 5
**...
.....
.*...
0 0
```

Output each field as `Field #n:`, its annotated rows, and a blank line
**between** fields:

```
Field #1:
*100
2210
1*10
1110

Field #2:
**100
33200
1*100
```

## Steps

1. Degenerate cases first: `""`, a single `.`, a single `*`.
2. One row: `".*."` → `"1*1"`, `"*.*"`. Then one column.
3. The 4×4 example.
4. Stage 2: `annotate_all(input)`. Keep it a thin wrapper that splits the
   input and calls `annotate`.

## Stretch

- Return `Result` from `annotate_all` for malformed input (wrong row count,
  wrong width, unknown characters).
- Pull a `neighbours(row, col)` iterator out and use it for both counting
  and for a "flood fill reveal" feature: clicking a `0` reveals everything
  around it.

<details><summary>Hint: count from the mines' point of view</summary>

Instead of asking every square "how many mines are next to you?", let every
mine add +1 to all its neighbours. Both work; one has fewer bounds checks.
</details>
