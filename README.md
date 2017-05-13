# Typed Tableau [![Build Status](https://travis-ci.org/lloydmeta/typed_tableau.svg?branch=master)](https://travis-ci.org/lloydmeta/typed_tableau)

A type-safe wrapping around [Tableau](https://github.com/mitsuhiko/tableau). Mostly done as an exercise.

Limitation: No cells that span multiple columns.

For a deep dive, RustDocs are available for:
* Code on [Master](https://beachape.com/typed_tableau)

## Examples

```rust
#[macro_use] extern crate frunk_core;
extern crate typed_tableau;
use typed_tableau::*;

// Declare our table with headers
let mut t = 
    table(hlist![ column::<&str>("Name"), column::<usize>("Age"), column::<bool>("Married")]);

// Add rows
t.add_row(hlist![ cell("Joe"),            cell(10),               cell(false)]);
t.add_row(hlist![ cell("Mary"),           cell(23),               cell(true) ]);
t.add_row(hlist![ cell("John"),           cell(53),               cell(false)]);
t.add_row(hlist![ cell("Rob"),            cell(41),               cell(true) ]);
// This will fail at compile time because we're trying to stuff the wrong type (f32) into the age column (usize)
// t.add_row(hlist![      cell("Rob"),            cell(41f32),            cell(true)]);

// Get back the untyped Tableau table
let untyped_t = t.into_untyped();

untyped_t.display();

// yields
// +------+-----+---------+
// | Name | Age | Married |
// +------+-----+---------+
// | Joe  | 10  |  false  |
// | Mary | 23  |  true   |
// | John | 53  |  false  |
// | Rob  | 41  |  true   |
// +------+-----+---------+

```