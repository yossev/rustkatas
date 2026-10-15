# Kata 10 — Gilded Rose (Legacy Code)

**Run:** `cargo test -p gilded-rose` · **Time:** 2 × 60–90 min · **Difficulty:** ★★★★☆

## Why this kata

The final boss, and the most realistic one. You inherit nested-`if` legacy code
with **no tests**, and someone wants a new feature. You'll practise the full
professional workflow:

1. **Characterisation tests** — pin down what the code *does* (not what it should do).
2. **Golden master** — a snapshot test that catches what you forgot.
3. **Refactor** to a design where the new feature is easy.
4. **TDD the new feature** into the clean design.

## The story

You work at the Gilded Rose inn, which sells items. The system updates every
item's `sell_in` and `quality` once per day via `update_quality()`.

### Requirements (as told by the previous developer)

- Every item has `sell_in` (days left to sell) and `quality` (value).
- At the end of each day both values go down by 1 for normal items.
- Once the sell-by date has passed, `quality` degrades **twice** as fast.
- `quality` is never negative.
- **"Aged Brie"** *increases* in quality as it gets older.
- `quality` is never more than **50**…
- …except **"Sulfuras, Hand of Ragnaros"**, a legendary item: always quality
  80, never has to be sold, never changes.
- **"Backstage passes to a TAFKAL80ETC concert"** increase in quality as the
  concert approaches: +2 when there are 10 days or less, +3 when 5 days or
  less, and drop to **0** after the concert.

### The new feature you're asked to add

- **"Conjured"** items (any item whose name starts with `"Conjured"`) degrade
  in quality **twice as fast** as normal items.

### The constraint

Don't change the `Item` struct or its fields — another team owns it.
You may change everything else.

---

## Part 1 — Safety net (no production code changes!)

1. Run the tests. `foo` fails on purpose. Fix the **test** (what *is* the name?).
   This is your warm-up in reading what code actually does.
2. Un-ignore the characterisation tests one at a time and fill them in.
   For each one, **predict** the result, run it, and if you were wrong, trust the
   code, not your prediction (and not the README!). Characterisation tests
   describe *current behaviour*, bugs included.
3. Create the golden master, from the workspace root:

   ```bash
   cargo run -p gilded-rose --example texttest -- 30 > katas/10-gilded-rose/golden_master.txt
   ```

   Un-ignore `golden_master`. It passes. Now **any** behaviour change shows up as a diff.
4. Check your net: break the code on purpose in 3–4 places. Do your
   *named* tests catch it, or only the golden master? Add named tests where
   only the golden master caught it.
5. `git commit`.

## Part 2 — Refactor

Tiny steps, tests after each one, commit often. A possible route:

1. Replace `self.items[i]` with `for item in &mut self.items` (the most
   satisfying first move).
2. Name the magic strings: `const AGED_BRIE: &str = ...`.
3. Flip negated conditions (`!=`) into positive ones and simplify nesting.
   `quality - quality` is just `0`.
4. Extract `fn update_item(item: &mut Item)`.
5. Inside it, `match item.name.as_str()` with one arm per kind of item.
   Duplicate code inside each arm if needed — **duplication is cheaper than the
   wrong abstraction** — then simplify each arm on its own.
6. Extract helpers: `increase_quality(item, amount)` / `decrease_quality(...)`
   that enforce the 0..=50 bounds.
7. (Optional, bigger) Introduce polymorphism: an `enum ItemKind` built from the
   name, or a `trait Updater` with one impl per kind. Since `Item` can't change,
   you derive the kind from the name.

## Part 3 — The new feature, TDD style

Un-ignore the conjured tests and make them pass. In a clean design this should
be a one-arm addition. The golden master will now fail **for the Conjured
row only** — that's an *intentional* change: inspect the diff, then regenerate
the golden master.

## Stretch

- Do Part 2 twice: once with an `enum`, once with a `trait`. Which reads better in Rust?
- Use `cargo mutants -p gilded-rose` to measure how good your test suite is.
- Try the **"lift-up conditional"** refactoring (Emily Bache's technique) on
  round 2 and compare to your approach.

<details><summary>Hint: what does `foo` return?</summary>

`update_quality` never changes names. The assertion should be about `"foo"`.
</details>

<details><summary>Hint: a target shape</summary>

```rust
fn update_item(item: &mut Item) {
    match item.name.as_str() {
        SULFURAS => {}
        AGED_BRIE => update_brie(item),
        BACKSTAGE => update_backstage(item),
        name if name.starts_with("Conjured") => update_normal(item, 2),
        _ => update_normal(item, 1),
    }
}
```
</details>
