use std::iter::zip;
use pyo3::prelude::*;
mod base;

use pyo3::types::{PyDict, PyNone};
//use pyo3::conversion::IntoPyObject::into_pyobject;
//use pyo3::PyTypeInfo;

// #[pyclass()]
// struct MMCIF;


// #[pymethods]
// impl MMCIF {
//     #[new]
//     #[pyo3(signature = (filepath))]
//     fn __new__(
//         filepath: &Bound<'_, PyString>
//     ) -> PyResult<Self> {
//         Ok(Self)
//     }

//     #[pyo3(signature = (filepath))]
//     fn __init__(
//         slf: &Bound<'_, Self>,
//         filepath: &Bound<'_, PyString>
//     ) -> PyResult<()> {
//         Ok(())
//     }
// }


#[pyfunction]
#[pyo3(name = "read_mmcif")]
#[pyo3(signature = (filepath, subset=vec![], exclude=vec![], as_dict=false, verbose=false))]
fn read_mmcif<'py>(py: Python<'py>, filepath:String, subset:Vec<String>, exclude:Vec<String>, as_dict:bool, verbose:bool) -> PyResult<Bound<'py, PyDict>> {
    let data = base::parser::parse(filepath.clone(), subset.clone(), exclude.clone(), verbose);
    let fields = data.fields;
    let dict = PyDict::new(py);
    for field in fields{
        match field.mode.as_str() {
            "dict"|"loop" => {
                //let key: Bound<'py, PyString> = PyString::new(py, &field.key);
                let mut items: Vec<Bound<'_, PyDict>> = vec![];
                //let obj = Bound<'py, PyObject> = PyObject::from((key, items));
                for item in field.subitems{
                    let subdict = PyDict::new(py);
                    for (k, v) in zip(field.subkeys.iter(), item.values.iter()){
                        match v.chars().next() {
                            Some(cc) => {
                                match cc {
                                    '.' | '?' => {
                                         let _ = subdict.set_item(k.to_string(), PyNone::get(py));
                                         continue
                                    },
                                    _ => {}
                                }
                            },
                            None => {
                                let _ = subdict.set_item(k.to_string(), PyNone::get(py));
                                continue
                            }

                        }
                        match v.parse::<i32>() {
                            Ok(vv) => {
                                let _ = subdict.set_item(k.to_string(), vv);
                                continue
                            },
                            Err(..) =>{
                                match v.parse::<f32>() {
                                    Ok(vv) => {
                                        let _ = subdict.set_item(k.to_string(), vv);
                                        continue
                                    },
                                    Err(..) =>{
                                        match v.parse::<bool>() {
                                            Ok(vv) => {
                                                let _ = subdict.set_item(k.to_string(), vv);
                                                continue
                                            },
                                            Err(..) =>{let _ = subdict.set_item(k.to_string(), v);}
                                        }
                                    }
                                }
                            }
                        }

                    }
                    items.push(subdict);
                }
                let _ = dict.set_item::<&String, Vec<Bound<'_, PyDict>>>(&field.key, items);
            },
            _ => {}
        }

    }
    //let dict = PyDict::new(PyString, PyDict);
    if as_dict {
       return Ok(dict)
    }
    todo!("Return MMCIF")


}

/// A Python module implemented in Rust.
#[pymodule]
fn mmcif_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_mmcif, m)?)?;
    //m.add_class::<MMCIF>()?;
    Ok(())
}