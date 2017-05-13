//! A typesafe table is a table where you decide ahead of time the types for
//! each column and the type system ensures that when you create a row, the types
//! of the cells line up properly.
//!
//! You can always switch to an untyped Tableau table (and use its many functions) by
//! calling `.to_untyped()`.
//!
//! # Example
//!
//! ```
//! use typed_tableau::*;
//! // Declare our table with headers
//! let mut t =
//!     table(column::<&str>("Name").column::<usize>("Age").column::<bool>("Married"));
//!
//! // Add rows
//! t.add_row(cell("Joe")           .cell(10)              .cell(false));
//! t.add_row(cell("Mary")          .cell(23)              .cell(true ));
//! t.add_row(cell("John")          .cell(53)              .cell(false));
//! t.add_row(cell("Rob")           .cell(41)              .cell(true ));
//! // This will fail at compile time because we're trying to stuff the wrong type (f32) into the age column (usize)
//! // t.add_row(cell("Rob")        .cell(41f32)           .cell(true));
//! // Get back the untyped Tableau table
//! let untyped_t = t.into_untyped();
//!
//! untyped_t.display()
//! ```
#[allow(unused)]
#[macro_use]
extern crate frunk_core;
extern crate console;
extern crate tableau;

mod table;
mod sugar;

pub use table::{Table, Column, Cell, table, cell, column};
pub use sugar::*;
