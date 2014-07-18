// -*- flycheck-rust-crate-root: "lib.rs" -*-
use ffi = super::ffi;

use std::kinds::marker::ContravariantLifetime;

/// The object that will be used for node manipulations
pub struct XPathObject<'a> {
    wrapper: XPathObjectWrapper,
    lt: ContravariantLifetime<'a>
}

struct XPathObjectWrapper {
    xpath_object: *const ffi::XPathObject,
}

pub fn from_raw_context<'a>(context: *const ffi::Context,
                        xpath: &str) -> Option<XPathObject<'a>> {
    let result =
        ffi::XPathObject::from_context(context, xpath);
    if result.is_null() {
        None
    } else {
        Some(XPathObject {
            wrapper: XPathObjectWrapper {
                xpath_object: result
            },
            lt: ContravariantLifetime
        })
    }
}

impl<'a> Drop for XPathObjectWrapper {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeObject(self.xpath_object);
        }
    }
}
