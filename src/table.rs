//! This module holds logic associated with constructing typed tables
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
//!
//! // Get back the untyped Tableau table
//! let untyped_t = t.into_untyped();
//!
//! untyped_t.display()
//! # }
//! ```
use std::fmt::Display;
use tableau;
use console::{Style, Alignment};
use std::marker::PhantomData;
use frunk_core::hlist::{HCons, HNil};

pub struct Table<R> {
    #[doc(hidden)]
    header: Vec<UntypedHeader>,
    rows: Vec<R>,
    style: Option<tableau::TableStyle>,
}

/// Creates a table with the given column headers.
///
/// Column cell types are automatically extracted
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate frunk_core;
/// # extern crate typed_tableau;
/// # use typed_tableau::*;
/// # fn main() {
/// let t = HeaderedTable(hlist![
///     Header::<String>("Name".to_string()), Header::<usize>("Age".to_string()), Header::<bool>("Married".to_string())
/// ]);
/// # }
/// ```
#[allow(non_snake_case)]
pub fn HeaderedTable<H>(header: H) -> Table<<H as CellTypeExtractor>::Out>
    where H: CellTypeExtractor,
          Vec<UntypedHeader>: FromHet<H>
{
    let as_headers = FromHet::from_het(header);
    Table {
        header: as_headers,
        rows: vec![],
        style: None,
    }
}

impl<R> Table<R> {
    /// Start off a new table without headers
    pub fn new() -> Table<R> {
        Table {
            header: vec![],
            rows: vec![],
            style: None,
        }
    }

    pub fn style(&mut self, style: tableau::TableStyle) {
        self.style = Some(style);
    }

    /// Adds a typed row to our table
    pub fn add_row<NewR>(&mut self, new_row: NewR)
        where R: From<NewR>
    {
        let as_r = R::from(new_row);
        self.rows.push(as_r);
    }

    /// Returns an untyped Tableau table from our typed table
    pub fn into_untyped(self) -> tableau::Table
        where Vec<tableau::Cell>: FromHet<R>
    {
        let mut tableau_table = tableau::Table::new();

        if self.header.len() > 0 {
            let mut h_row = tableau_table.add_head_row();
            for h in self.header {
                let c = tableau::Cell::from(h);
                h_row.add_cell(c);
            }
        }

        for r in self.rows {
            let mut u_row = tableau_table.add_row();
            let t_row: Vec<tableau::Cell> = FromHet::from_het(r);
            for ut_cell in t_row {
                u_row.add_cell(ut_cell);
            }
        }

        tableau_table
    }
}

pub struct Cell<C> {
    val: C,
    style: Option<Style>,
    alignment: Option<Alignment>,
}

impl<C> Cell<C> {
    pub fn new(v: C) -> Cell<C> {
        Cell {
            val: v,
            style: None,
            alignment: None,
        }
    }

    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}


impl<V> From<Cell<V>> for tableau::Cell
    where V: Display
{
    fn from(h: Cell<V>) -> Self {
        let styled = match h.style {
            Some(style) => style.apply_to(h.val),
            None => Style::default().apply_to(h.val),
        };
        let mut cell = tableau::Cell::new(styled);
        if let Some(alignment) = h.alignment {
            cell.align(alignment);
        }
        cell
    }
}

/// A typed header
pub struct Header<H> {
    pub name: String,
    pub style: Option<Style>,
    pub alignment: Option<Alignment>,
    tp_holder: PhantomData<H>,
}

impl<H> Header<H> {
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

/// Builds a header with the given type and name
#[allow(non_snake_case)]
pub fn Header<T>(s: String) -> Header<T> {
    Header {
        name: s,
        style: None,
        alignment: None,
        tp_holder: PhantomData,
    }
}

/// Given any type, produces an Out type.
///
/// Used for converting a HList of Header<T> into an HList of
/// Cell<T>
pub trait CellTypeExtractor {
    type Out;
}

impl CellTypeExtractor for HNil {
    type Out = HNil;
}

impl<H, T> CellTypeExtractor for HCons<Header<H>, T>
    where T: CellTypeExtractor
{
    type Out = HCons<Cell<H>, <T as CellTypeExtractor>::Out>;
}

impl From<UntypedHeader> for tableau::Cell {
    fn from(h: UntypedHeader) -> Self {
        let styled = match h.style {
            Some(style) => style.apply_to(h.name),
            None => Style::default().apply_to(h.name),
        };
        let mut cell = tableau::Cell::new(styled);
        if let Some(alignment) = h.alignment {
            cell.align(alignment);
        }
        cell
    }
}

#[doc(hidden)]
pub struct UntypedHeader {
    name: String,
    style: Option<Style>,
    alignment: Option<Alignment>,
}

#[doc(hidden)]
impl<H> From<Header<H>> for UntypedHeader {
    fn from(t: Header<H>) -> Self {
        UntypedHeader {
            name: t.name,
            style: t.style,
            alignment: t.alignment,
        }
    }
}

/// Custom From typeclass so we can implement from HList to Vec
pub trait FromHet<A> {
    fn from_het(a: A) -> Self;
}

#[doc(hidden)]
impl FromHet<HNil> for Vec<UntypedHeader> {
    fn from_het(_: HNil) -> Self {
        vec![]
    }
}

#[doc(hidden)]
impl<H, T> FromHet<HCons<H, T>> for Vec<UntypedHeader>
    where UntypedHeader: From<H>,
          Vec<UntypedHeader>: FromHet<T>
{
    fn from_het(a: HCons<H, T>) -> Self {
        let HCons { head: h, tail: t } = a;
        let h_untyped = UntypedHeader::from(h);
        let mut h_vec: Vec<UntypedHeader> = vec![h_untyped];
        let mut t_vec: Vec<UntypedHeader> = FromHet::from_het(t);
        h_vec.append(&mut t_vec);
        h_vec
    }
}

#[doc(hidden)]
impl FromHet<HNil> for Vec<tableau::Cell> {
    fn from_het(_: HNil) -> Self {
        vec![]
    }
}

#[doc(hidden)]
impl<H, T> FromHet<HCons<H, T>> for Vec<tableau::Cell>
    where Vec<tableau::Cell>: FromHet<T>,
          tableau::Cell: From<H>
{
    fn from_het(s: HCons<H, T>) -> Self {
        let HCons { head: h, tail: t } = s;
        let cell = tableau::Cell::from(h);
        let mut v_h: Vec<tableau::Cell> = vec![cell];
        let mut v_t: Vec<tableau::Cell> = FromHet::from_het(t);
        v_h.append(&mut v_t);
        v_h
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adding_to_Table() {
        let mut t: Table<Hlist![Cell<i32>, Cell<bool>, Cell<f32>]> = Table::new();
        for i in 1..11 {
            t.add_row(hlist![Cell::new(i), Cell::new(i % 2 == 0), Cell::new(i as f32)])
        }
        assert_eq!(t.rows.len(), 10)
    }

    #[test]
    fn adding_to_Table_with_header() {
        let mut t = HeaderedTable(hlist![Header::<String>("Name".to_string()),
                                         Header::<usize>("Age".to_string()),
                                         Header::<bool>("Married".to_string())]);
        for i in 1..6 {
            t.add_row(hlist![Cell::new(format!("Joe {}", i)),
                             Cell::new(i + 10),
                             Cell::new(i % 2 == 0)])
        }
        assert_eq!(t.rows.len(), 5)
    }

    #[test]
    fn into_untyped() {
        let mut t = HeaderedTable(hlist![Header::<String>("Name".to_string()),
                                         Header::<usize>("Age".to_string()),
                                         Header::<bool>("Married".to_string())]);
        for i in 1..6 {
            t.add_row(hlist![Cell::new(format!("Joe {}", i)).align(Alignment::Left),
                             Cell::new(i + 10),
                             Cell::new(i % 2 == 0)])
        }
        t.into_untyped();
    }

}
