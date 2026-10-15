# Kata 14 — Bank Account (Outside-In)

**Run:** `cargo test -p bank-account` · **Time:** 90 min · **Difficulty:** ★★★★☆

## Why this kata

Every kata so far was a pure function: input in, output out. Real code talks
to things you can't control in a test, like **the clock** and **the console**.
This kata (popularised by Sandro Mancuso) teaches:

- **Outside-in TDD** (the "double loop"): one acceptance test stays red
  while unit tests drive the pieces inside it.
- **Test doubles**: a fake clock that returns the dates you choose, and a spy
  printer that records what was printed so the test can check it.
- **Traits as seams**: in Rust that means `&dyn Trait` (or generics), so tests
  can plug in their doubles.

## The acceptance criteria

```
Given a client makes a deposit of 1000 on 10/01/2012
And a deposit of 2000 on 13/01/2012
And a withdrawal of 500 on 14/01/2012
When they print their bank statement
Then they would see:

Date || Amount || Balance
14/01/2012 || -500 || 2500
13/01/2012 || 2000 || 3000
10/01/2012 || 1000 || 1000
```

Newest first. That acceptance test is already written for you in `lib.rs`.

## The constraints

- `Account`'s public API is fixed: `deposit(amount)`, `withdraw(amount)`,
  `print_statement()`. All three return `()`. You can't add getters "just for testing".
- Dates come from a `Clock`, output goes to a `Printer`. Both are traits.
- Amounts are whole numbers (`i64`). No money types needed.

## Steps

1. Run the tests. The acceptance test fails on `SpyPrinter`, which is yours
   to finish (a `RefCell<Vec<String>>` that pushes each line).
2. Now it fails on `todo!()` inside `Account`. **Leave it red** (it's your
   outer loop) and switch to the `unit` tests. Run them alone with
   `cargo test -p bank-account unit`.
3. Let the unit tests drive the pieces you need. A common shape is
   `Transaction { date, amount }`, a list of transactions, and a statement
   formatter that turns them into lines. Your design may differ, and that's fine.
4. When the inner pieces are done, wire them into `Account`. The acceptance
   test goes green, and you're done.

## Stretch

- Swap `&dyn Clock` for generics (`Account<C: Clock, P: Printer>`). What
  changes in the tests? Which do you prefer?
- Reject withdrawals that would overdraw the account. Since `withdraw`
  returns `()`, how do you report it? (Changing the API is allowed for this one.)
- Print amounts with two decimals (`1000.00`) and keep everything in cents.

<details><summary>Hint: why RefCell?</summary>

`Printer::print_line` takes `&self` (the account only holds a shared
reference). To record lines through `&self`, the spy needs interior
mutability: `self.lines.borrow_mut().push(...)`.
</details>

<details><summary>Hint: running balance, newest first</summary>

Compute balances oldest-first (a running sum with `scan` or a `fold`), then
reverse the lines for printing.
</details>
