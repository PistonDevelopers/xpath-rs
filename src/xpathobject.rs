use ffi = super::ffi;

pub struct XPathObject {
    xpathObject: *const ffi::XPathObject
}

impl XPathObject {
    pub fn from_raw_context(context: *const ffi::Context,
                            xpath: &str) -> Option<XPathObject> {
        let result = ffi::XPathObject::from_context(context, xpath);
        if result.is_null() {
            None
        } else {
            Some(XPathObject { xpathObject: result })
        }
    }
}
