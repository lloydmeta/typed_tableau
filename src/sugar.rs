//! Holds syntatic sugar ops for working with typed tables
//!
//! The point of this is to allow users to not need to use the hlist macros
//! since they already need to wrap their types in `cell` or `column` anyways.

use frunk_core::hlist::{HCons, HNil};
use table::{Cell, cell, Column, column};
use std::ops::Add;

/// Sugar for appending anything to an HList of Cells
///
/// # Example
/// ```
/// # use typed_tableau::*;
/// let cell = cell(1).cell(42f32).cell("hi");
/// ```
pub trait AppendToCells<With> {
    type Out;
    fn cell(self, o: With) -> Self::Out;
}

impl<H, T, With> AppendToCells<With> for HCons<H, T>
    where HCons<H, T>: Add<HCons<Cell<With>, HNil>>
{
    type Out = <HCons<H, T> as Add<HCons<Cell<With>, HNil>>>::Output;

    fn cell(self, o: With) -> Self::Out {
        self +
        HCons {
            head: cell(o),
            tail: HNil,
        }
    }
}

impl<H, With> AppendToCells<With> for Cell<H> {
    type Out = HCons<Cell<H>, HCons<Cell<With>, HNil>>;

    fn cell(self, o: With) -> Self::Out {
        HCons {
            head: self,
            tail: HCons {
                head: cell(o),
                tail: HNil,
            },
        }
    }
}

/// Sugar for appending anything to an HList of Columns
///
/// # Example
/// ```
/// use typed_tableau::*;
/// let columns = column::<&str>("name").column::<usize>("age").column::<bool>("is_admin");
/// ```
pub trait AppendToColumns {
    fn column<ColType>(self, name: &str) -> <Self as Add<HCons<Column<ColType>, HNil>>>::Output
        where Self: Add<HCons<Column<ColType>, HNil>>;
}

#[doc(hidden)]
impl<H, W> Add<HCons<Column<W>, HNil>> for Column<H> {
    type Output = HCons<Self, HCons<Column<W>, HNil>>;

    fn add(self, rhs: HCons<Column<W>, HNil>) -> Self::Output {
        HCons {
            head: self,
            tail: rhs,
        }
    }
}

impl<H> AppendToColumns for Column<H> {
    fn column<ColType>(self, name: &str) -> <Self as Add<HCons<Column<ColType>, HNil>>>::Output
        where Self: Add<HCons<Column<ColType>, HNil>>
    {
        self +
        HCons {
            head: column(name),
            tail: HNil,
        }

    }
}

impl<H, T> AppendToColumns for HCons<H, T> {
    fn column<ColType>(self, name: &str) -> <Self as Add<HCons<Column<ColType>, HNil>>>::Output
        where Self: Add<HCons<Column<ColType>, HNil>>
    {
        self +
        HCons {
            head: column(name),
            tail: HNil,
        }
    }
}

/// Sugar for appending something that is already contained onto an HList
///
/// # Example
///
/// ```
/// # extern crate typed_tableau;
/// # extern crate console;
/// # fn main () {
/// use typed_tableau::*;
/// use console::{Alignment, Style};
///
/// let columns =
///     column::<&str>("name").
///     column::<f32>("weight").with(
///     column::<bool>("is_admin").style(Style::new().green()));
///
/// let cells =
///     cell("joe").with(
///     cell(42f32).align(Alignment::Right)).
///     cell(true);
/// # }
/// ```
pub trait AppendWith<With> {
    type Out;

    fn with(self, o: With) -> Self::Out;
}

impl<H, W> AppendWith<Cell<W>> for Cell<H> {
    type Out = HCons<Self, HCons<Cell<W>, HNil>>;

    fn with(self, o: Cell<W>) -> Self::Out {
        HCons {
            head: self,
            tail: HCons {
                head: o,
                tail: HNil,
            },
        }
    }
}

impl<H, T, CellType> AppendWith<Cell<CellType>> for HCons<H, T>
    where Self: Add<HCons<Cell<CellType>, HNil>>
{
    type Out = <Self as Add<HCons<Cell<CellType>, HNil>>>::Output;

    fn with(self, o: Cell<CellType>) -> Self::Out {
        self +
        HCons {
            head: o,
            tail: HNil,
        }
    }
}

impl<H, W> AppendWith<Column<W>> for Column<H> {
    type Out = HCons<Self, HCons<Column<W>, HNil>>;

    fn with(self, o: Column<W>) -> Self::Out {
        HCons {
            head: self,
            tail: HCons {
                head: o,
                tail: HNil,
            },
        }
    }
}

impl<H, T, ColType> AppendWith<Column<ColType>> for HCons<H, T>
    where Self: Add<HCons<Column<ColType>, HNil>>
{
    type Out = <Self as Add<HCons<Column<ColType>, HNil>>>::Output;

    fn with(self, o: Column<ColType>) -> Self::Out {
        self +
        HCons {
            head: o,
            tail: HNil,
        }
    }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn appending_cell_to_cell() {
        let _: Hlist![Cell<i32>, Cell<f32>, Cell<&str>] = cell(1).cell(42f32).cell("hi");
    }

    #[test]
    fn appending_column_to_column() {

        let _: Hlist![Column<&str>, Column<usize>, Column<bool>] = column::<&str>("name").column::<usize>("age").column::<bool>("is_admin");
    }


}
