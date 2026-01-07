use chakracore_sys as sys;

pub struct Guard<'a> {
    pub(crate) prev: sys::JsContextRef,
    pub(crate) current: sys::JsContextRef,
    pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Guard<'a> {
    pub fn context_raw(&self) -> sys::JsContextRef {
        self.current
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = sys::JsSetCurrentContext(self.prev);
        }
    }
}
