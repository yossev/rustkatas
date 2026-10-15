//! Prints the shop's state for N days (default 2).
//!   cargo run -p gilded-rose --example texttest -- 30

fn main() {
    let days = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(2);
    print!("{}", gilded_rose::texttest::report(days));
}
