// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~




use pyo3::prelude::*;
mod bond; 
mod cox_ingersoll_ross;
mod hull_white;
mod vasicek;


#[pymodule]
fn bonds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<bond::PyZeroCouponBond>()?;
    m.add_class::<cox_ingersoll_ross::PyCoxIngersollRoss>()?;
    m.add_class::<hull_white::PyHullWhite>()?;
    m.add_class::<vasicek::PyVasicek>()?;
    Ok(())
}

