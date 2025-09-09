use pyo3::prelude::*;
use pyo3::types::PyModule;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let code = r#"
def predict(x):
    return [i**2 for i in x]
"#;

        let module = PyModule::from_code(py, code, "ai_module.py", "ai_module")?;
        let predict = module.getattr("predict")?;
        let result = predict.call1((vec![1, 2, 3, 4],))?;

        println!("Predicted: {:?}", result);
        Ok(())
    })
}
