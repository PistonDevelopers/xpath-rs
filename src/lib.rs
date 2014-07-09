#![crate_name = "xpath"]
#![deny(missing_doc)]
//! The XPath module from libxml2

extern crate libc;

mod ffi;

#[cfg(test)]
mod test;

/// The XML document
pub struct Document {
    xmlDocument: *const ffi::Document
}

struct Context {
    xmlXPathContext: *const ffi::Context
}

/// The XPath Object itself
pub struct XPathObject {
    xpathObject: *const ffi::XPathObject
}

impl Document {

    /// Creates a new Document from a string representation
    pub fn from_str(body: &str) -> Option<Document> {
        unsafe {
            let doc = body.with_c_str(|cstr| ffi::xmlParseDoc(cstr));
            raw_doc_to_option(doc)
        }
    }

    /// Creates a new Document from a file
    pub fn from_file(filename: &str) -> Option<Document> {
        unsafe {
            let doc = filename.with_c_str(|cstr| ffi::xmlParseFile(cstr));
            raw_doc_to_option(doc)
        }
    }

    /// Creates a new Context to be used for reaching nodes
    fn new_context(&self) -> Option<Context> {
        unsafe {
            let context = ffi::xmlXPathNewContext(self.xmlDocument);
            if context.is_null() {
                None
            } else {
                Some(Context { xmlXPathContext: context })
            }
        }
    }

    /// Gets a nodeset from a Document
    pub fn get_node_set(&self, xpath: &str) -> Option<XPathObject> {
        match self.new_context() {
            Some(context) => {
                let result =
                    ffi::XPathObject::from_context(context.xmlXPathContext, xpath);
                if result.is_null() {
                    None
                } else {
                    Some(XPathObject { xpathObject: result })
                }
            },
            None => None
        }
    }
}

impl Drop for XPathObject {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeObject(self.xpathObject);
        }
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlFreeDoc(self.xmlDocument);
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeContext(self.xmlXPathContext);
        }
    }
}

unsafe fn raw_doc_to_option(doc: *const ffi::Document) -> Option<Document> {
    if doc.is_null() {
        None
    } else {
        Some(Document { xmlDocument: doc })
    }
}
