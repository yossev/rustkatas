# Kata 09 — Tennis (Refactoring)

**Run:** `cargo test -p tennis` · **Time:** 60 min · **Difficulty:** ★★★☆☆

## Why this kata

So far you've written code test-first. Most real work is **changing existing
code**. This kata (from Emily Bache's refactoring collection) gives you code
that works but is ugly. You'll practise the other half of TDD: the
**refactor** step, done safely, in tiny steps, always green.

## Tennis scoring

- Points are called `Love` (0), `Fifteen` (1), `Thirty` (2), `Forty` (3).
- Equal scores: `Love-All`, `Fifteen-All`, `Thirty-All`, then `Deuce` from 3-3 on.
- At least 3 points each and one player ahead by 1: `Advantage <player>`.
- At least 4 points and ahead by 2: `Win for <player>`.

## The mission

### Part 1 — Build the safety net (don't touch the code yet!)
1. Run the tests. `scores_match_the_table` passes;
   `points_can_arrive_in_any_order` is a `todo!()` — **write it**.
2. The table is incomplete. Add the missing cases (see the TODO in the test).
3. **Test your tests:** introduce a deliberate bug in `get_score` (e.g. change
   `>= 2` to `>= 3`). Does a test fail? If not, add a case. Revert the bug.
   Repeat with 3–4 different bugs. (This is manual *mutation testing*; the
   `cargo-mutants` crate automates it.)
4. `git commit`.

### Part 2 — Refactor
Rules: **one small step → run tests → commit.** Never be red for more than a minute.
If you're red, `git checkout .` and take a smaller step.

Smells to hunt (pick them off one at a time):

- [ ] `m_score1` / `m_score2` — Hungarian-ish naming. Rename.
- [ ] `let mut score` assigned in every branch → make each branch an *expression*
      (`let score = if ... { ... } else { ... };` or just `return`).
- [ ] The `for i in 1..3` loop with `temp_score` — it's really "name of p1's
      score" + "-" + "name of p2's score". Extract `fn point_name(points) -> &'static str`.
- [ ] `minus_result` — magic arithmetic. Extract `fn leader(&self)`.
- [ ] The three big branches → three well-named functions (`tied_score`,
      `end_game_score`, `running_score`). `get_score` becomes a 3-arm `match`/if.
- [ ] Hard-coded `"player1"` strings. (The compiler already warns that the
      name fields are never read: that's a clue.)

### Part 3 — New feature
Now that the code is clean, un-ignore `uses_real_player_names` and make it pass.
Notice how much easier it is than it would have been in Part 1.

## Stretch

- Replace the two `i32`s with an `enum Point { Love, Fifteen, Thirty, Forty }`
  plus a separate state for deuce/advantage/win. Can you make *illegal states
  unrepresentable* (e.g. `Forty-Forty` doesn't exist, it's `Deuce`)?
- `cargo install cargo-mutants && cargo mutants -p tennis` — does your table
  catch every mutant?

<details><summary>Hint: a clean target shape</summary>

```rust
pub fn get_score(&self) -> String {
    if self.p1 == self.p2 {
        self.tied_score()
    } else if self.p1 >= 4 || self.p2 >= 4 {
        self.end_game_score()
    } else {
        format!("{}-{}", point_name(self.p1), point_name(self.p2))
    }
}
```
</details>
