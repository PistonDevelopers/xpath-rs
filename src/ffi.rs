// -*- flycheck-rust-crate-root: "lib.rs" -*-
use libc::{c_char, c_int, c_double, c_void, c_ushort};
use std::ptr;
use std::c_str::ToCStr;

#[repr(C)]
pub struct Document;
#[repr(C)]
pub struct Context;
#[allow(dead_code)]
#[repr(C)]
pub struct XPathObject {
    xpath_type: c_ushort,
    pub node_set: *const NodeSet,
    bool_val: c_int,
    float_val: c_double,
    string_val: *const c_char,
    user: *const c_void,
    index: c_int,
    user2: *const c_void,
    index2: c_int
}
/// The nodeset
#[allow(dead_code)]
#[repr(C)]
pub struct NodeSet {
    node_nr: c_int,
    node_max: c_int,
    node_tab: *mut Node
}

/// A node from the nodeset
#[allow(dead_code)]
#[repr(C)]
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
#[repr(C)]
pub struct Namespace;
#[repr(C)]
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

impl Copy for Node {}

impl Copy for NodeSet {}

impl<'a> NodeSet {
    /// Gets a vector of nodes from the NodeSet
    pub fn get_nodes(&'a self) -> Vec<&'a Node> {
        unsafe {
            let node_ptr = self.node_tab;
            if node_ptr.is_null() {
                vec![]
            } else {
                range(0, self.node_nr).map(
                    |off| &*self.node_tab.clone().offset(off as int)).collect()
            }
        }
    }
}
