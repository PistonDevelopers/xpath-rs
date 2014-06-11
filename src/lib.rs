#![crate_id = "xpath"]
#![deny(missing_doc)]
//! The XPath module from libxml2

extern crate libc;

mod ffi;
pub mod document;
