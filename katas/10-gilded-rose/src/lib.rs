//! Kata 10 — Gilded Rose
//!
//! The Gilded Rose is a small inn in a fantasy city, run by the friendly
//! innkeeper Allison. Her shop trades in fine goods, and the system that
//! tracks them was written by Leeroy, who has since left on new adventures.
//! Now a supplier is offering "Conjured" items, and Allison wants them in
//! the shop by the weekend. You open Leeroy's code. It goes deeper than you
//! hoped. Somewhere in the corner, a goblin glares at you.
//!
//! Whisper: right now, the code knows more than the requirements do.
//!
//! Read README.md first.
//!
//! Legacy code. Nobody remembers exactly why it works. It does work.
//! Do NOT change the `Item` struct (the goblin in the corner will insta-rage).

use std::fmt::{self, Display};

pub mod texttest;

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if self.items[i].name != "Aged Brie"
                && self.items[i].name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != "Aged Brie" {
                    if self.items[i].name != "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].quality > 0 {
                            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper: one item, one day, return the item.
    fn after_one_day(name: &str, sell_in: i32, quality: i32) -> Item {
        let mut rose = GildedRose::new(vec![Item::new(name, sell_in, quality)]);
        rose.update_quality();
        rose.items.remove(0)
    }

    // The famous first test of this kata. It fails. Fix the TEST, not the code.
    #[test]
    fn foo() {
        let mut rose = GildedRose::new(vec![Item::new("foo", 0, 0)]);
        rose.update_quality();
        assert_eq!("fixme", rose.items[0].name);
    }

    // ======== Characterisation tests: describe what the code DOES today ========
    // Write one test per rule in the README. Use `after_one_day`.

    #[test]
    #[ignore]
    fn normal_item_degrades_by_one_and_sell_in_decreases() {
        let item = after_one_day("+5 Dexterity Vest", 10, 20);
        assert_eq!(item.sell_in, 9);
        assert_eq!(item.quality, 19);
    }

    #[test]
    #[ignore]
    fn normal_item_degrades_twice_as_fast_after_sell_by() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn quality_is_never_negative() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn aged_brie_increases_in_quality() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn aged_brie_increases_twice_as_fast_after_sell_by() {
        todo!("YOUR TEST: surprise! the README doesn't say this, but the code does it")
    }

    #[test]
    #[ignore]
    fn quality_never_exceeds_50() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn sulfuras_never_changes() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn backstage_passes_increase_by_one_when_more_than_10_days() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn backstage_passes_increase_by_two_when_10_days_or_less() {
        todo!("YOUR TEST: check the boundaries 11 and 10 and 6")
    }

    #[test]
    #[ignore]
    fn backstage_passes_increase_by_three_when_5_days_or_less() {
        todo!("YOUR TEST: check the boundaries 6, 5 and 1")
    }

    #[test]
    #[ignore]
    fn backstage_passes_drop_to_zero_after_the_concert() {
        todo!("YOUR TEST")
    }

    // ======== Golden master: the safety net that catches what you forgot ========

    #[test]
    #[ignore]
    fn golden_master() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/golden_master.txt");
        let expected = std::fs::read_to_string(path).unwrap_or_else(|_| {
            panic!(
                "No golden master yet. Create it BEFORE refactoring with:\n  \
                 cargo run -p gilded-rose --example texttest -- 30 > {path}"
            )
        });
        assert_eq!(texttest::report(30), expected);
    }

    // ======== NEW FEATURE (after refactoring) ========

    #[test]
    #[ignore]
    fn conjured_items_degrade_twice_as_fast() {
        let item = after_one_day("Conjured Mana Cake", 3, 6);
        assert_eq!(item.quality, 4);
    }

    #[test]
    #[ignore]
    fn conjured_items_degrade_four_per_day_after_sell_by() {
        todo!("YOUR TEST")
    }
}
