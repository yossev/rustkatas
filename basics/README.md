# Rust basics

Ten short drills for getting comfortable with Rust itself before (or while)
doing the katas. Each one is about **one idea**: ownership, borrowing,
`Option`, `Result`, and so on.

They're easier than the katas on purpose. The functions are small, most
tests are already written, and the main job is getting the Rust right, not
solving a puzzle.

## How to do a drill

1. Open the file (e.g. `src/01_ownership.rs`) and read the comment at the
   top. It explains the idea in a few lines and tells you which Rust Book
   chapter covers it.
2. Run its tests:

   ```bash
   cargo test -p rust-basics ownership
   ```

   They're all red at first. That's the point.
3. Fill in the functions one at a time, top to bottom, until the tests go
   green. **Read the signatures carefully.** `s: String` vs `s: &str` vs
   `s: &mut String` is often the whole lesson.
4. Tests that say `todo!("YOUR TEST: ...")` are yours to write.
5. Each file ends with a **THINK ABOUT IT** box. Do it. Breaking things on
   purpose and reading the compiler error teaches more than getting it right
   first time.

## The drills

| # | File | Run with | What it's about |
|---|------|----------|-----------------|
| 01 | [ownership](src/01_ownership.rs) | `ownership` | Moving vs lending, `String` vs `&str` |
| 02 | [borrowing](src/02_borrowing.rs) | `borrowing` | `&` vs `&mut`, changing things in place |
| 03 | [strings](src/03_strings.rs) | `strings` | Slices, chars vs bytes, your first lifetime |
| 04 | [structs](src/04_structs.rs) | `structs` | Methods, `&self` / `&mut self` / `self` |
| 05 | [enums](src/05_enums.rs) | `enums` | Variants with data, `match`, guards |
| 06 | [options](src/06_options.rs) | `options` | `Option`, `map`, `unwrap_or`, `?` |
| 07 | [results](src/07_results.rs) | `results` | `Result`, custom errors, `?`, `collect` |
| 08 | [collections](src/08_collections.rs) | `collections` | `Vec`, `HashMap`, the entry API |
| 09 | [iterators](src/09_iterators.rs) | `iterators` | Iterator chains and closures, no `for` loops allowed |
| 10 | [traits](src/10_traits.rs) | `traits` | Traits, default methods, generics, `Display`, `+` |

The order follows the Rust Book, so you can read a chapter and then do the
matching drill.

## Tips

- **Compiler errors are hints, not insults.** Read the whole message. It
  very often tells you exactly what to write (`help: consider borrowing here: &s`).
- **Look up the method.** Most of these are one or two standard library
  calls. Searching "rust string uppercase" or browsing the
  [std docs](https://doc.rust-lang.org/std/) is part of the practice.
- **Don't `clone()` your way out** unless the drill says it's fine. If the
  borrow checker complains, the drill is usually trying to teach you why.
- **Redo a drill a few days later** without looking at your old answer.
  That's what makes it stick.

When you've done all ten, the katas in `katas/` will feel a lot friendlier.
