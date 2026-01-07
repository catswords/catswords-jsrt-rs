use crate::error::{ok, Result};
use chakracore_sys as sys;

pub struct Runtime {
    pub(crate) raw: sys::JsRuntimeHandle,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let mut rt: sys::JsRuntimeHandle = std::ptr::null_mut();
        unsafe {
            ok(sys::JsCreateRuntime(sys::JsRuntimeAttributes::None, std::ptr::null_mut(), &mut rt))?;
        }
        Ok(Self { raw: rt })
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                let _ = sys::JsDisposeRuntime(self.raw);
            }
            self.raw = std::ptr::null_mut();
        }
    }
}
