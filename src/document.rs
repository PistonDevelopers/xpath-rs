// -*- flycheck-rust-crate-root: "lib.rs" -*-
use ffi = super::ffi;
use super::context::Context;

/// The XML document
pub struct Document {
    xmlDocument: *const ffi::Document
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

    /// Creates a new Context
    pub fn new_context<'a>(&'a self) -> Option<Context<'a>> {
        super::context::from_raw_document(self.xmlDocument)
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlFreeDoc(self.xmlDocument);
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
