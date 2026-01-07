use crate::error::{ok, Result};
use crate::guard::Guard;
use chakracore_sys as sys;

#[derive(Clone, Copy)]
pub struct Value {
    pub(crate) raw: sys::JsValueRef,
}

impl Value {
    pub fn raw(&self) -> sys::JsValueRef {
        self.raw
    }

    // Recommended: return Result instead of silently swallowing conversion errors.
    pub fn to_integer(&self, _guard: &Guard<'_>) -> Result<i32> {
        let mut out: i32 = 0;
        unsafe {
            ok(sys::JsNumberToInt(self.raw, &mut out))?;
        }
        Ok(out)
    }

    pub fn undefined(_guard: &Guard<'_>) -> Result<Self> {
        let mut v: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok(sys::JsGetUndefinedValue(&mut v))?; }
        Ok(Self { raw: v })
    }

    pub fn null(_guard: &Guard<'_>) -> Result<Self> {
        let mut v: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok(sys::JsGetNullValue(&mut v))?; }
        Ok(Self { raw: v })
    }
}
