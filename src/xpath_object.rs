// -*- flycheck-rust-crate-root: "lib.rs" -*-
use ffi = super::ffi;

use std::kinds::marker::ContravariantLifetime;

/// The object that will be used for node manipulations
pub struct XPathObject<'a> {
    wrapper: XPathObjectWrapper,
    lt: ContravariantLifetime<'a>
}

struct XPathObjectWrapper {
    xpath_object: *const ffi::XPathObject,
}

/// The nodeset
pub struct NodeSet<'a> {
    node_set: &'a ffi::NodeSet,
}

/// A node from the document
pub struct Node<'a> {
    node: &'a ffi::Node,
}

impl<'a> XPathObject<'a> {
    /// Gets a nodeset from the xpathobject
    pub fn get_node_set(&'a self) -> Option<NodeSet<'a>> {
        unsafe {
            let node_set_ptr = (*self.wrapper.xpath_object).node_set;
            if node_set_ptr.is_null() {
                None
            } else {
                Some(NodeSet { node_set: &*node_set_ptr })
            }
        }
    }
}

impl<'a> NodeSet<'a> {
    /// Gets a vector of nodes from the NodeSet
    pub fn get_nodes(&'a self) -> Vec<Node<'a>> {
        unsafe {
            let node_ptr = self.node_set.node_tab;
            if node_ptr.is_null() {
                vec![]
            } else {
                range(0, self.node_set.node_nr).map(
                    |off| Node {
                        node: &*self.node_set.node_tab.clone().offset(off as int)
                    }).collect()
            }
        }
    }
}

pub fn from_raw_context<'a>(context: *const ffi::Context,
                        xpath: &str) -> Option<XPathObject<'a>> {
    let result =
        ffi::XPathObject::from_context(context, xpath);
    if result.is_null() {
        None
    } else {
        Some(XPathObject {
            wrapper: XPathObjectWrapper {
                xpath_object: result
            },
            lt: ContravariantLifetime
        })
    }
}

impl<'a> Drop for XPathObjectWrapper {
    fn drop(&mut self) {
        unsafe {
            ffi::xmlXPathFreeObject(self.xpath_object);
        }
    }
}
