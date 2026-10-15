# Kata 07 — Conway's Game of Life

**Run:** `cargo test -p game-of-life` · **Time:** 60–90 min · **Difficulty:** ★★★☆☆

## Why this kata

The classic kata of Code Retreats. It's big enough that **where you start**
matters. This one is set up **inside-out**: you test the tiny pure rule
first, then neighbours, then whole-world ticks, then parsing/rendering. On
round 2, try **outside-in** (start from the blinker test) and compare.

## Rules

A grid of cells, each alive or dead. Every tick, each cell looks at its 8
neighbours:

1. A live cell with **< 2** live neighbours dies (underpopulation).
2. A live cell with **2 or 3** live neighbours lives on.
3. A live cell with **> 3** live neighbours dies (overpopulation).
4. A dead cell with **exactly 3** live neighbours becomes alive (reproduction).

All cells update **simultaneously** — compute the next world from the old
one; never mutate in place while reading.

For this kata the world is **finite**: cells outside the grid are dead.

## Text format

`#` = alive, `.` = dead, rows separated by `\n`:

```
.....        .....
..#..        .....
..#..  tick→ .###.     (the "blinker")
..#..        .....
.....        .....
```

## Steps

1. **`next_state`** — the four rules as a pure function. Write all four rule
   tests (some given, some yours). Make it a *single* `match` expression.
2. **`World::parse` + `is_alive`** — parse the text format.
3. **`live_neighbours`** — careful at the edges! (`usize` underflow will panic
   in debug builds: `0 - 1`.) Write a corner-cell test.
4. **`render`** — the inverse of parse; `parse(s).render() == s`.
5. **`tick`** — combine everything. Blinker and block tests.

## Stretch

- Implement `std::str::FromStr` and `std::fmt::Display` for `World` instead of
  `parse`/`render`.
- **Infinite world:** store only live cells in a `HashSet<(i64, i64)>`. Your
  `next_state` tests shouldn't need to change at all — if they do, why?
- Add a glider test: after 4 ticks it has moved one cell diagonally.
- `fn main` that animates it in the terminal.

<details><summary>Hint: next_state as one match</summary>

```rust
match (alive, live_neighbours) {
    (true, 2) | (_, 3) => true,
    _ => false,
}
```
(Try to get there by refactoring, not by copying this.)
</details>

<details><summary>Hint: neighbours without underflow</summary>

Iterate `dr in -1i64..=1`, `dc in -1i64..=1`, skip `(0, 0)`, convert
`row as i64 + dr` and check bounds before indexing. Or write
`fn get(&self, r: i64, c: i64) -> bool` that returns `false` outside the grid.
</details>
