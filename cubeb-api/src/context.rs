use std::ffi::CString;
use {Context, Result};

/// Initialize a new cubeb [`Context`]
///
/// See the documentation for [`Context::init`] for more info
pub fn init<T: Into<Vec<u8>>>(name: T) -> Result<Context> {
    let name = CString::new(name)?;

    Context::init(Some(name.as_c_str()), None)
}
