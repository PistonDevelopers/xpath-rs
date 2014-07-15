use ffi = super::ffi;

/// The object that will be used for node manipulations
pub struct XPathObject {
    xpathObject: *const ffi::XPathObject
}

pub fn from_raw_context(context: *const ffi::Context,
                        xpath: &str) -> Option<XPathObject> {
    let result =
        ffi::XPathObject::from_context(context, xpath);
    if result.is_null() {
        None
    } else {
        Some(XPathObject { xpathObject: result })
    }
}

impl Drop for XPathObject {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeObject(self.xpathObject);
        }
    }
}
