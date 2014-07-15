use ffi = super::ffi;
use super::xpath_object::XPathObject;

/// A context to get XPathObjects
pub struct Context {
    xmlXPathContext: *const ffi::Context
}

pub fn from_raw_document(document: *const ffi::Document) -> Option<Context> {
    unsafe {
        let context = ffi::xmlXPathNewContext(document);
        if context.is_null() {
            None
        } else {
            Some(Context { xmlXPathContext: context })
        }
    }
}

impl Context {
    /// Instances a new XPathObject to access its nodesets
    pub fn new_xpath_object(&self, xpath: &str) -> Option<XPathObject> {
        super::xpath_object::from_raw_context(self.xmlXPathContext, xpath)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeContext(self.xmlXPathContext);
        }
    }
}
