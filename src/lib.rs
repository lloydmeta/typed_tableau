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
//! # #[macro_use] extern crate frunk_core;
//! # extern crate typed_tableau;
//! # fn main() {
//! use typed_tableau::*;
//! // Declare our table with headers
//! let mut t = HeaderedTable(hlist![
//!     Header::<String>("Name".to_string()), Header::<usize>("Age".to_string()), Header::<bool>("Married".to_string())
//! ]);
//!
//! // Add rows
//! t.add_row(hlist![
//!     Cell::new("Joe".to_string()), Cell::new(10), Cell::new(false)
//! ]);
//! t.add_row(hlist![
//!     Cell::new("Mary".to_string()), Cell::new(23), Cell::new(true)]
//! );
//! t.add_row(hlist![
//!     Cell::new("John".to_string()), Cell::new(53), Cell::new(false)
//! ]);
//! t.add_row(hlist![
//!     Cell::new("Rob".to_string()), Cell::new(41), Cell::new(true)]
//! );
//! // This will fail at compile time because we're trying to stuff the wrong type (f32) into the age column (usize)
//! // t.add_row(hlist![
//! //     Cell::new("Rob".to_string()), Cell::new(41f32), Cell::new(true)]
//! // );
//! // Get back the untyped Tableau table
//! let untyped_t = t.into_untyped();
//!
//! untyped_t.display()
//! # }
//! ```
#[allow(unused)]
#[macro_use]
extern crate frunk_core;
extern crate console;
extern crate tableau;

pub use tableau::TableStyle;

pub mod table;

pub use table::*;
