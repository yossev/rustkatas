//! Produces a text report of the shop over N days. Used for the golden master.
//! You shouldn't need to change this file.

use crate::{GildedRose, Item};
use std::fmt::Write;

pub fn starting_items() -> Vec<Item> {
    vec![
        Item::new("+5 Dexterity Vest", 10, 20),
        Item::new("Aged Brie", 2, 0),
        Item::new("Elixir of the Mongoose", 5, 7),
        Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
        Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
        // this conjured item does not work properly yet
        Item::new("Conjured Mana Cake", 3, 6),
    ]
}

pub fn report(days: u32) -> String {
    let mut rose = GildedRose::new(starting_items());
    let mut out = String::new();
    for day in 0..=days {
        writeln!(out, "-------- day {day} --------").unwrap();
        writeln!(out, "name, sellIn, quality").unwrap();
        for item in &rose.items {
            writeln!(out, "{item}").unwrap();
        }
        writeln!(out).unwrap();
        rose.update_quality();
    }
    out
}
