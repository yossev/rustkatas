# Kata 15 — Trip Service (Legacy Code with Dependencies)

**Run:** `cargo test -p trip-service` · **Time:** 90 min · **Difficulty:** ★★★★★

## Why this kata

Gilded Rose was hard to *read*. Trip Service is hard to *test*: it calls a
global session and a database directly, so any test crashes. This is the
everyday reality of legacy code, and this kata (by Sandro Mancuso) teaches the
moves from Michael Feathers' *Working Effectively with Legacy Code*:

- find the **seams**, the places where you can swap behaviour without
  editing the code around them,
- **break dependencies** with tiny, safe changes,
- get the code under test, **then** refactor it properly.

In Java you'd subclass and override. Rust has no inheritance, so you'll use
**traits** and **dependency injection** instead, which is good practice in itself.

## What the code does

`get_trips_by_user(user)`:

1. Gets the logged-in user from the session. If nobody is logged in →
   `Err(UserNotLoggedIn)`.
2. If the logged-in user is in `user`'s friend list → `user`'s trips from the database.
3. Otherwise → an empty list.

## The rules

- You **can't** change `user_session` or `trip_dao`. Another team owns them.
- Existing callers write `TripService.get_trips_by_user(&user)` today. When
  you're done, production code should still be easy to call (a `TripService::new()`
  or `Default` that wires in the real session and DAO is fine).
- Until tests cover it, change `get_trips_by_user` only with small mechanical
  steps. No rewriting "because it's obviously the same".

## Steps

### Part 1 — create seams
1. Run the tests. `the_service_cannot_be_tested_as_it_is` passes because it
   *expects* the crash. That's your proof of the problem.
2. Introduce a trait for "who's logged in?" and one for "find trips for a user".
   Give `TripService` fields for them. Make production implementations that
   just call `user_session` / `trip_dao`.
3. Point `get_trips_by_user` at the fields instead of the modules. This
   should be a two-line change inside the method.

### Part 2 — tests
4. Write the four tests with your own fakes. Start with the **shortest
   path** through the code (not logged in) and end with the **deepest** (friends).
   This is the opposite of how you'd *refactor*, and that's deliberate.
5. Check your net: break the code (flip `is_friend`) and see a test go red.

### Part 3 — refactor
Now clean it up. Start from the **deepest** branch and work outwards:
- the friend loop is `user.get_friends().contains(&logged_user)`. Better still,
  move it onto `User` as `is_friends_with(&other)`;
- `let mut` + `break` + flags → expressions;
- use `?` / `ok_or` for the logged-in check.

The final method should be about three lines.

## Stretch

- Do it again with generics (`TripService<S: Session, R: TripRepository>`)
  instead of `Box<dyn ...>`. Compare the tests.
- `User` is compared with `==` across the whole struct, friends and trips
  included. Is that right? Give users an `id` and compare that instead (with
  tests first, of course).

<details><summary>Hint: a seam shape</summary>

```rust
pub trait Session { fn logged_user(&self) -> Option<User>; }
pub trait TripRepository { fn trips_by(&self, user: &User) -> Vec<Trip>; }

pub struct TripService {
    session: Box<dyn Session>,
    trips: Box<dyn TripRepository>,
}
```
</details>
