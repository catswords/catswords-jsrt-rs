use crate::error::{ok, Result};
use crate::guard::Guard;
use crate::runtime::Runtime;
use chakracore_sys as sys;

pub struct Context {
    pub(crate) raw: sys::JsContextRef,
}

impl Context {
    pub fn new(runtime: &Runtime) -> Result<Self> {
        let mut cx: sys::JsContextRef = std::ptr::null_mut();
        unsafe { ok(sys::JsCreateContext(runtime.raw, &mut cx))?; }
        Ok(Self { raw: cx })
    }

    pub fn make_current(&self) -> Result<Guard<'_>> {
        let mut prev: sys::JsContextRef = std::ptr::null_mut();
        unsafe {
            ok(sys::JsGetCurrentContext(&mut prev))?;
            ok(sys::JsSetCurrentContext(self.raw))?;
        }
        Ok(Guard {
            prev,
            current: self.raw,
            _marker: std::marker::PhantomData,
        })
    }
}
