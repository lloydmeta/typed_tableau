extern crate typed_tableau;
extern crate console;

use console::{Style, Color};
use typed_tableau::*;

/// Wrapped port of https://github.com/mitsuhiko/tableau/blob/master/examples/simple.rs
fn main() {
    let mut t = table(column::<&str>("Name")
                        .with(
                            column::<usize>("Age").style(Style::new().bg(Color::Blue))
                        ).
                      column::<bool>("Married"));
    t.add_row(cell("Joe").cell(10).cell(false));
    t.add_row(cell("Mary").cell(23).cell(true));
    t.add_row(cell("John").cell(53).cell(false));
    t.add_row(cell("Rob"). cell(41).cell(true));
    let untyped_t = t.into_untyped();

    untyped_t.display()
}
