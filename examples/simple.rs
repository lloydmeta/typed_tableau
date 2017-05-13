#[macro_use]
extern crate frunk_core;
extern crate typed_tableau;
extern crate console;

use console::{Style, Color};
use typed_tableau::*;

/// Wrapped port of https://github.com/mitsuhiko/tableau/blob/master/examples/simple.rs
fn main() {
    let mut t = HeaderedTable(hlist![Header::<String>("Name".to_string())
                                         .style(Style::new().bg(Color::Magenta)),
                                     Header::<usize>("Age".to_string())
                                         .style(Style::new().bg(Color::Blue)),
                                     Header::<bool>("Married".to_string())
                                         .style(Style::new().bg(Color::Red))]);
    t.add_row(hlist![Cell::new("Joe".to_string()),
                     Cell::new(10),
                     Cell::new(false)]);
    t.add_row(hlist![Cell::new("Mary".to_string()),
                     Cell::new(23),
                     Cell::new(true)]);
    t.add_row(hlist![Cell::new("John".to_string()),
                     Cell::new(53),
                     Cell::new(false)]);
    t.add_row(hlist![Cell::new("Rob".to_string()), Cell::new(41), Cell::new(true)]);
    let untyped_t = t.into_untyped();

    untyped_t.display()
}
