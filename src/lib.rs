mod captcha;

use pyo3::prelude::*;

#[pymodule]
fn captcha_generator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<captcha::CaptchaData>()?;
    m.add_class::<captcha::CaptchaGenerator>()?;

    Ok(())
}
