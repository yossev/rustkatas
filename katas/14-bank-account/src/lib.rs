//! Kata 14 — Bank Account
//!
//! The Rusty Bank has one branch, one teller and a printer that jams on
//! Tuesdays. Customers walk in, deposit their wages, take some cash out, and
//! ask for their statement on the way out. The teller is happy to hand over
//! an account object. "Just don't make me wait for a real Tuesday to test
//! it," they say, "and don't use up my paper."
//!
//! Whisper: what the account can't control, it shouldn't create.
//!
//! Read README.md for the rules.
#![allow(unused)] // delete this line when you're done, then fix the warnings

/// Today's date as "dd/mm/yyyy".
pub trait Clock {
    fn today(&self) -> String;
}

/// Somewhere to print statement lines (the console, in production).
pub trait Printer {
    fn print_line(&self, line: &str);
}

/// The real printer.
pub struct ConsolePrinter;

impl Printer for ConsolePrinter {
    fn print_line(&self, line: &str) {
        println!("{line}");
    }
}

pub struct Account<'a> {
    clock: &'a dyn Clock,
    printer: &'a dyn Printer,
    // add whatever else you need
}

impl<'a> Account<'a> {
    pub fn new(clock: &'a dyn Clock, printer: &'a dyn Printer) -> Self {
        Account { clock, printer }
    }

    pub fn deposit(&mut self, amount: i64) {
        todo!()
    }

    pub fn withdraw(&mut self, amount: i64) {
        todo!()
    }

    pub fn print_statement(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    // ---------------- test doubles ----------------

    /// A clock that returns the dates you give it, in order.
    struct FakeClock {
        dates: RefCell<VecDeque<String>>,
    }

    impl FakeClock {
        fn returning(dates: &[&str]) -> Self {
            FakeClock {
                dates: RefCell::new(dates.iter().map(|d| d.to_string()).collect()),
            }
        }
    }

    impl Clock for FakeClock {
        fn today(&self) -> String {
            self.dates
                .borrow_mut()
                .pop_front()
                .expect("FakeClock ran out of dates")
        }
    }

    /// A printer that remembers what it printed.
    #[derive(Default)]
    struct SpyPrinter {
        lines: RefCell<Vec<String>>,
    }

    impl SpyPrinter {
        fn lines(&self) -> Vec<String> {
            self.lines.borrow().clone()
        }
    }

    impl Printer for SpyPrinter {
        fn print_line(&self, line: &str) {
            todo!("YOUR CODE: record the line")
        }
    }

    // ---------------- the outer loop ----------------

    #[test]
    fn acceptance_prints_statement_newest_first() {
        let clock = FakeClock::returning(&["10/01/2012", "13/01/2012", "14/01/2012"]);
        let printer = SpyPrinter::default();
        let mut account = Account::new(&clock, &printer);

        account.deposit(1000);
        account.deposit(2000);
        account.withdraw(500);
        account.print_statement();

        assert_eq!(
            printer.lines(),
            vec![
                "Date || Amount || Balance",
                "14/01/2012 || -500 || 2500",
                "13/01/2012 || 2000 || 3000",
                "10/01/2012 || 1000 || 1000",
            ]
        );
    }

    // ---------------- the inner loop ----------------
    // These tests drive the pieces INSIDE Account. You'll probably add
    // types and functions above (Transaction? a statement formatter?) and
    // test them here directly. Rename and add tests freely.

    mod unit {
        use super::*;

        #[test]
        #[ignore]
        fn empty_account_prints_only_the_header() {
            todo!("YOUR TEST")
        }

        #[test]
        #[ignore]
        fn a_deposit_is_stored_with_todays_date() {
            todo!("YOUR TEST: how will you observe this without a getter on Account?")
        }

        #[test]
        #[ignore]
        fn a_withdrawal_is_stored_as_a_negative_amount() {
            todo!("YOUR TEST")
        }

        #[test]
        #[ignore]
        fn statement_lines_have_a_running_balance() {
            todo!("YOUR TEST")
        }

        #[test]
        #[ignore]
        fn statement_lines_are_newest_first() {
            todo!("YOUR TEST")
        }
    }
}
