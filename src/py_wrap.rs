use crate::{ParseResult, WritingChar};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "try_read")]
pub fn try_read_py<'a, 'b>(
    hiragana: &'a str,
    romaji: &'b str,
) -> PyResult<Option<(&'a str, &'b str, (String, String))>> {
    Ok(crate::try_read(hiragana, romaji))
}

#[pyfunction]
#[pyo3(name = "parse_hiragana")]
pub fn parse_hiragana_py(mut hiragana: &str) -> PyResult<Vec<(String, String)>> {
    match crate::parse_hiragana(hiragana) {
        Some(v) => Ok(v),
        None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Failed to parse hiragana",
        )),
    }
}

pub type PyParseResult = (
    String,
    Vec<(String, String)>,
    (String, String, String),
    Vec<(String, String)>,
);

#[pyfunction]
#[pyo3(name = "parse_hiragana_with_buf")]
pub fn parse_hiragana_with_buf_py(mut hiragana: &str, mut romaji: &str) -> PyResult<PyParseResult> {
    match crate::parse_hiragana_with_buf(hiragana, romaji) {
        Some(ParseResult::Writing(completed, writing, rest)) => Ok((
            "Writing".to_string(),
            completed,
            (writing.hiragana, writing.romaji, writing.cur_buf_string),
            rest,
        )),
        Some(ParseResult::Completed(completed)) => Ok((
            "Completed".to_string(),
            completed,
            ("".to_string(), "".to_string(), "".to_string()),
            vec![],
        )),
        None => Err(pyo3::exceptions::PyValueError::new_err(
            "Failed to parse hiragana with buf",
        )),
    }
}

#[pymodule]
pub fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(try_read_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_hiragana_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_hiragana_with_buf_py, m)?)?;
    Ok(())
}
