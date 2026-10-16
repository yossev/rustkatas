//! # 02 · Borrowing
//!
//! Two kinds of references:
//!
//! - `&T`, a **shared** borrow: read-only. You can have as many as you like.
//! - `&mut T`, a **mutable** borrow: can change the value. Only **one** at a
//!   time, and no shared borrows while it exists.
//!
//! That one rule ("many readers OR one writer") is how Rust prevents data
//! races and dangling pointers at compile time.
//!
//! To pass a mutable borrow, the variable itself must be `mut`, and you
//! write `&mut` at the call site: `add_exclamation(&mut s)`.
//!
//! Book: chapter 4.2, "References and Borrowing"
//!
//! Run: `cargo test -p rust-basics borrowing`

/// Adds a '!' to the end of `s`, in place.
pub fn add_exclamation(s: &mut String) {
    // Since its a mutable reference
    s.push('!') // can directly change it since we have a mut reference.
}

/// Pushes `x` onto `v` twice.
pub fn push_twice(v: &mut Vec<i32>, x: i32) {
    v.push(x);
    v.push(x);
    // here, again, we can change the vec since it has a mutable reference.
}

/// Adds 1 to the counter. (Hint: you'll need the `*` operator.)
pub fn increment(counter: &mut u32) {
    *counter += 1; //no automatic help like .
}

/// Swaps the first and last elements. Does nothing if there are fewer than 2.
pub fn swap_ends(v: &mut [i32]) {
    if v.len() < 2{
        return;
    }
    let a = 0;
    let b = v.len()-1;
    v.swap(a, b);
}

/// Removes every negative number from `v`, in place.
pub fn remove_negatives(v: &mut Vec<i32>) {
    // Why Wouldnt this work?
    // for x in v.iter_mut() {
    //    if *x < 0{
    //    }
    // }
    v.retain(|&x| x >= 0);
}

/// Total number of characters across all the words. Only reads, so `&`.
pub fn total_length(words: &[String]) -> usize {
    let mut count = 0;
    for word in words {
        count+= word.chars().count();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exclamation_changes_the_string_in_place() {
        let mut s = String::from("wow");
        add_exclamation(&mut s);
        assert_eq!(s, "wow!");
    }

    #[test]
    fn push_twice_pushes_two_copies() {
        let mut v = vec![1];
        push_twice(&mut v, 7);
        assert_eq!(v, vec![1, 7, 7]);
    }

    #[test]
    fn increment_three_times() {
        let mut count = 0;
        increment(&mut count);
        increment(&mut count);
        increment(&mut count);
        assert_eq!(count, 3);
    }

    #[test]
    fn swap_ends_swaps_first_and_last() {
        let mut v: [i32; 4] = [1, 2, 3, 4];
        swap_ends(&mut v);
        assert_eq!(v, [4, 2, 3, 1]);
    }

    #[test]
    fn swap_ends_on_short_slices_does_nothing() {
        let mut v:[i32; 1] = [1];
        swap_ends(&mut v);
        assert_eq!(v, [1]);
    }

    #[test]
    fn remove_negatives_keeps_the_rest_in_order() {
        let mut v = vec![3, -1, 4, -1, 5];
        remove_negatives(&mut v);
        assert_eq!(v, vec![3, 4, 5]);
    }

    #[test]
    fn total_length_adds_up_characters() {
        let words = vec![String::from("hi"), String::from("there")];
        assert_eq!(total_length(&words), 7);
        assert_eq!(words.len(), 2); // still ours
    }

    // THINK ABOUT IT:
    //
    //     let mut s = String::from("a");
    //     let r1 = &s;
    //     let r2 = &mut s;
    //     println!("{r1}");
    //
    // Why is this an error? Now delete the println!. Why does it compile?
    // (Look up "non-lexical lifetimes".)
}
