#[macro_use]
extern crate frunk_core;
extern crate typed_tableau;
extern crate console;

use console::{Style, Color};
use typed_tableau::*;

/// Wrapped port of https://github.com/mitsuhiko/tableau/blob/master/examples/simple.rs
fn main() {
    let mut t = table(hlist![column::<&str>("Name").style(Style::new().bg(Color::Magenta)),
                             column::<usize>("Age").style(Style::new().bg(Color::Blue)),
                             column::<bool>("Married").style(Style::new().bg(Color::Red))]);
    t.add_row(hlist![cell("Joe"), cell(10), cell(false)]);
    t.add_row(hlist![cell("Mary"), cell(23), cell(true)]);
    t.add_row(hlist![cell("John"), cell(53), cell(false)]);
    t.add_row(hlist![cell("Rob"), cell(41), cell(true)]);
    let untyped_t = t.into_untyped();

    untyped_t.display()
}
