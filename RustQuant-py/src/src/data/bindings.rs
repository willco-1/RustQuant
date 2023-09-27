// bindings.rs

use pyo3::prelude::*;
use polars::prelude::*;

#[pyfunction]
fn read_data(format: &str, path: &str) -> PyResult<Data> {
    // Create and return a Data object (your Rust struct)
    Ok(Data::new(format.parse()?, path.to_string()))
}

#[pyfunction]
fn write_data(data: &mut Data, path: &str) -> PyResult<()> {
    // Call the write method on your Data struct
    data.path = path.to_string();
    data.write().map_err(|e| PyErr::new::<pyo3::exceptions::IOError, _>(format!("{}", e)))
}

#[pyfunction]
fn scan_data(data: &mut Data) -> PyResult<LazyFrame> {
    // Call the scan method on your Data struct
    data.scan().map_err(|e| PyErr::new::<pyo3::exceptions::IOError, _>(format!("{}", e)))
}

// Define a Python module to encapsulate your functions
#[pymodule]
fn io(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_data))?;
    m.add_function(wrap_pyfunction!(write_data))?;
    m.add_function(wrap_pyfunction!(scan_data))?;
    
    Ok(())
}


use pyo3::prelude::*;
use polars::prelude::*;
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;

// Import your Rust modules
use RustQuant::YahooFinanceData;
use RustQuant::ReturnsType;

#[pyclass]
pub struct YahooFinanceDataWrapper {
    inner: YahooFinanceData,
}

#[pymethods]
impl YahooFinanceDataWrapper {
    #[new]
    fn new(ticker: &str) -> Self {
        YahooFinanceDataWrapper {
            inner: YahooFinanceData::new(ticker.to_string()),
        }
    }

    fn set_start_date(&mut self, start: i64) {
        self.inner.set_start_date(OffsetDateTime::from_unix_timestamp(start));
    }

    fn set_end_date(&mut self, end: i64) {
        self.inner.set_end_date(OffsetDateTime::from_unix_timestamp(end));
    }

    fn compute_returns(&mut self, returns_type: ReturnsType) {
        self.inner.compute_returns(returns_type);
    }

   

    fn get_price_history(&mut self) {
        self.inner.get_price_history();
    }

    fn get_options_chain(&mut self) {
        self.inner.get_options_chain();
    }
}

#[pymodule]
fn yahoo(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<YahooFinanceDataWrapper>()?;
    Ok(())
}


