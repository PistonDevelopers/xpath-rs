//! Handling documents
use ffi = super::ffi;
use xpathobject::XPathObject;

/// The XML document
pub struct Document {
    xmlDocument: *const ffi::Document
}

struct Context {
    xmlXPathContext: *const ffi::Context
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
            Some(context) => XPathObject::from_raw_context(context.xmlXPathContext,
                                                           xpath),
            None => None
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

#[cfg(test)]
mod test {
    use super::Document;
    #[test]
    fn return_none_for_badly_formatted_xml() {
        assert!(Document::from_str("foo").is_none());
    }

    #[test]
    fn return_some_for_well_formatted_xml() {
        assert!(Document::from_str("<foo></foo>").is_some());
    }

    #[test]
    fn return_none_for_non_existent_file() {
        assert!(Document::from_file("foo").is_none());
    }

    #[test]
    fn return_some_for_existent_file() {
        assert!(Document::from_file("test_assets/foo.xml").is_some());
    }

    #[test]
    fn return_valid_xpath_object_for_extent_path() {
        assert!(Document::from_file("test_assets/foo.xml").unwrap()
                .get_node_set("/foo").is_some());
    }
}
