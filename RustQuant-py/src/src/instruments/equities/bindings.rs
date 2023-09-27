use pyo3::prelude::*;

mod commonstock;



#[pymodule]
fn equities(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<commonstock::PyCommonStock>()?;
    Ok(())
}


