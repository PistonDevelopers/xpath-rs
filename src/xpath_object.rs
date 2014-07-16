use ffi = super::ffi;

use std::kinds::marker::ContravariantLifetime;

/// The object that will be used for node manipulations
pub struct XPathObject<'a> {
    xpathObject: *const ffi::XPathObject,
    lt: ContravariantLifetime<'a>
}

pub fn from_raw_context<'a>(context: *const ffi::Context,
                        xpath: &str) -> Option<XPathObject<'a>> {
    let result =
        ffi::XPathObject::from_context(context, xpath);
    if result.is_null() {
        None
    } else {
        Some(XPathObject {
            xpathObject: result,
            lt: ContravariantLifetime
        })
    }
}

#[unsafe_destructor]
impl<'a> Drop for XPathObject<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeObject(self.xpathObject);
        }
    }
}
