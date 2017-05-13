//! This module holds logic associated with constructing typed tables
use std::fmt::Display;
use tableau;
use console::{Style, Alignment};
use std::marker::PhantomData;
use frunk_core::hlist::{HCons, HNil};

/// A table with typed columns
pub struct Table<R> {
    #[doc(hidden)]
    columns: Vec<UntypedColumn>,
    rows: Vec<R>,
    style: Option<tableau::TableStyle>,
}

/// Creates a table with the given column columns.
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
/// let t = table(column::<String>("Name").column::<usize>("Age").column::<bool>("Married"));
/// # }
/// ```
pub fn table<H>(columns: H) -> Table<<H as CellTypeExtractor>::Out>
    where H: CellTypeExtractor,
          Vec<UntypedColumn>: FromHet<H>
{
    let as_columns = FromHet::from_het(columns);
    Table {
        columns: as_columns,
        rows: vec![],
        style: None,
    }
}

impl<R> Table<R> {
    /// Useful for when you just want to create a table by type
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate frunk_core;
    /// # extern crate typed_tableau;
    /// # use typed_tableau::*;
    /// # fn main() {
    /// type Columns = Hlist![Cell<i32>, Cell<bool>, Cell<f32>];
    /// let mut t: Table<Columns> = Table::typed();
    /// # }
    /// ```
    pub fn typed() -> Table<R> {
        Table {
            columns: vec![],
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

        if self.columns.len() > 0 {
            let mut h_row = tableau_table.add_head();
            for h in self.columns {
                let c = tableau::Cell::from(h);
                h_row.add_cell(c);
            }
        }

        for r in self.rows {
            let mut u_row = tableau_table.add();
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

/// Creates a new cell
pub fn cell<C>(v: C) -> Cell<C> {
    Cell {
        val: v,
        style: None,
        alignment: None,
    }
}

impl<C> Cell<C> {
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

/// A typed column
pub struct Column<H> {
    pub name: String,
    pub style: Option<Style>,
    pub alignment: Option<Alignment>,
    tp_holder: PhantomData<H>,
}

impl<H> Column<H> {
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

/// Builds a column with the given type and name
pub fn column<T>(s: &str) -> Column<T> {
    Column {
        name: String::from(s),
        style: None,
        alignment: None,
        tp_holder: PhantomData,
    }
}

/// Given any type, produces an Out type.
///
/// Used for converting a HList of Column<T> into an HList of
/// Cell<T>
pub trait CellTypeExtractor {
    type Out;
}

impl CellTypeExtractor for HNil {
    type Out = HNil;
}

impl<H, T> CellTypeExtractor for HCons<Column<H>, T>
    where T: CellTypeExtractor
{
    type Out = HCons<Cell<H>, <T as CellTypeExtractor>::Out>;
}

impl From<UntypedColumn> for tableau::Cell {
    fn from(h: UntypedColumn) -> Self {
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
pub struct UntypedColumn {
    name: String,
    style: Option<Style>,
    alignment: Option<Alignment>,
}

#[doc(hidden)]
impl<H> From<Column<H>> for UntypedColumn {
    fn from(t: Column<H>) -> Self {
        UntypedColumn {
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
impl FromHet<HNil> for Vec<UntypedColumn> {
    fn from_het(_: HNil) -> Self {
        vec![]
    }
}

#[doc(hidden)]
impl<H, T> FromHet<HCons<H, T>> for Vec<UntypedColumn>
    where UntypedColumn: From<H>,
          Vec<UntypedColumn>: FromHet<T>
{
    fn from_het(a: HCons<H, T>) -> Self {
        let HCons { head: h, tail: t } = a;
        let h_untyped = UntypedColumn::from(h);
        let mut h_vec: Vec<UntypedColumn> = vec![h_untyped];
        let mut t_vec: Vec<UntypedColumn> = FromHet::from_het(t);
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

    use sugar::*;
    use super::*;

    #[test]
    fn adding_to_Table() {
        type Columns = Hlist![Cell<i32>, Cell<bool>, Cell<f32>];
        let mut t: Table<Columns> = Table::typed();
        for i in 1..11 {
            t.add_row(cell(i).cell(i % 2 == 0).cell(i as f32))
        }
        assert_eq!(t.rows.len(), 10)
    }

    #[test]
    fn adding_to_Table_with_header() {
        let mut t = table(column::<String>("Name").column::<usize>("Age").column::<bool>("Married"));
        for i in 1..6 {
            t.add_row(cell(format!("Joe {}", i))   .cell(i + 10)         .cell(i % 2 == 0))
        }
        assert_eq!(t.rows.len(), 5)
    }

    #[test]
    fn into_untyped() {
        let mut t = table(column::<String>("Name").
                          column::<usize>("Age").
                          column::<bool>("Married"));
        for i in 1..6 {
            t.add_row(cell(format!("Joe {}", i)).align(Alignment::Left).
                      cell(i + 10).
                      cell(i % 2 == 0))
        }
        t.into_untyped();
    }

}
