use pyo3::prelude::*;

mod option; 
mod asian;
mod binonial;
mod greeks; 
mod rainbow; 


#[pymodule]
fn options(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<option::PyOption>()?;
 
    m.add_submodule()?;
