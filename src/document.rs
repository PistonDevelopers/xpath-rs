//! Handling documents
use ffi = super::ffi;

/// The XML document
pub struct Document {
    xmlDocument: *ffi::Document
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
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlFreeDoc(self.xmlDocument);
        }
    }
}

unsafe fn raw_doc_to_option(doc: *ffi::Document) -> Option<Document> {
    if doc.is_null() {
        None
    } else {
        Some(Document { xmlDocument: doc })
    }
}


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
    assert!(Document::from_file("foo.xml").is_some());
}
