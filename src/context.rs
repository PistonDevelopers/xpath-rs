// -*- flycheck-rust-crate-root: "lib.rs" -*-
use ffi = super::ffi;
use super::xpath_object::XPathObject;

use std::kinds::marker::ContravariantLifetime;

/// A context to get XPathObjects
pub struct Context<'a> {
    wrapper: ContextWrapper,
    lt: ContravariantLifetime<'a>
}

struct ContextWrapper {
    xml_xpath_context: *const ffi::Context
}

pub fn from_raw_document<'a>(document: *const ffi::Document)
                             -> Option<Context<'a>> {
    unsafe {
        let context = ffi::xmlXPathNewContext(document);
        if context.is_null() {
            None
        } else {
            Some(Context {
                wrapper: ContextWrapper {
                    xml_xpath_context: context,
                },
                lt: ContravariantLifetime
            })
        }
    }
}

impl<'a> Context<'a> {
    /// Instances a new XPathObject to access its nodesets
    pub fn new_xpath_object(&'a self, xpath: &str) -> Option<XPathObject<'a>> {
        super::xpath_object::from_raw_context(self.wrapper.xml_xpath_context,
                                              xpath)
    }
}

impl Drop for ContextWrapper {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeContext(self.xml_xpath_context);
        }
    }
}
