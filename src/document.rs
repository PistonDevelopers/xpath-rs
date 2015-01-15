// -*- flycheck-rust-crate-root: "lib.rs" -*-
use super::ffi as ffi;
use super::context::Context;
use std::ffi::CString;

/// The XML document
pub struct Document {
    xml_document: *const ffi::Document
}

impl Document {
    /// Creates a new Document from a string representation
    pub fn from_str(body: &str) -> Option<Document> {
        unsafe {
            let doc = ffi::xmlParseDoc(CString::from_slice(body.as_bytes()).as_ptr());
            raw_doc_to_option(doc)
        }
    }

    /// Creates a new Document from a file
    pub fn from_file(filename: &str) -> Option<Document> {
        unsafe {
            let doc = ffi::xmlParseFile(CString::from_slice(filename.as_bytes()).as_ptr());
            raw_doc_to_option(doc)
        }
    }

    /// Creates a new Context
    pub fn new_context<'a>(&'a self) -> Option<Context<'a>> {
        super::context::from_raw_document(self.xml_document)
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlFreeDoc(self.xml_document);
        }
    }
}

unsafe fn raw_doc_to_option(doc: *const ffi::Document) -> Option<Document> {
    if doc.is_null() {
        None
    } else {
        Some(Document { xml_document: doc })
    }
}
