// -*- flycheck-rust-crate-root: "lib.rs" -*-
use libc::{c_char, c_int, c_double, c_void, c_ushort};
use std::ptr;

pub struct Document;
pub struct Context;
pub struct XPathObject {
    xpath_type: c_ushort,
    node_set: *const NodeSet,
    bool_val: c_int,
    float_val: c_double,
    string_val: *const c_char,
    user: *const c_void,
    index: c_int,
    user2: *const c_void,
    index2: c_int
}
pub struct NodeSet {
    node_nr: c_int,
    node_max: c_int,
    node_tab: *const Node
}
pub struct Node {
    private: *const c_void,
    element_type: c_ushort,
    name: *const c_char,
    children: *const Node,
    last: *const Node,
    parent: *const Node,
    next: *const Node,
    prev: *const Node,
    doc: *const Document,
    namespace: *const Namespace,
    content: *const c_char,
    properties: *const Attribute,
    namespace_def: *const Namespace,
    psvi: *const c_void,
    line: c_ushort,
    extra: c_ushort
}
pub struct Namespace;
pub struct Attribute;

#[link(name = "xml2")]
extern {
    pub fn xmlParseDoc(body: *const c_char) -> *const Document;
    pub fn xmlParseFile(filename: *const c_char) -> *const Document;
    pub fn xmlFreeDoc(doc: *const Document);

    pub fn xmlXPathNewContext(doc: *const Document) -> *const Context;
    pub fn xmlXPathFreeContext(context: *const Context);

    pub fn xmlXPathEvalExpression(xpath: *const c_char, context: *const Context)
                                  -> *const XPathObject;
    pub fn xmlXPathFreeObject(object: *const XPathObject);
}

#[inline]
fn xml_xpath_node_set_is_empty(ns: *const NodeSet) -> bool {
    unsafe {
        ns.is_null() || (*ns).node_nr==0 || (*ns).node_tab.is_null()
    }
}

impl XPathObject {
    pub fn from_context(context: *const Context,
                        xpath: &str) -> *const XPathObject
    {
        unsafe {
            let result = xmlXPathEvalExpression(xpath.to_c_str().as_ptr(),
                                                context);
            if result.is_null() {
                ptr::null()
            } else if xml_xpath_node_set_is_empty((*result).node_set) {
                xmlXPathFreeObject(result);
                ptr::null()
            } else {
                result
            }
        }
    }
}
