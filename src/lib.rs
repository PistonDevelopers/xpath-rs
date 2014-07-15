#![deny(missing_doc)]
//! The XPath module from libxml2

extern crate libc;
pub use document::Document;
pub use context::Context;
pub use xpath_object::XPathObject;

mod ffi;
mod document;
mod context;
mod xpath_object;

#[cfg(test)]
mod test;
