use libc::{c_char, c_int, c_double, c_void, c_ushort};

pub struct Document;
pub struct Context;
pub struct XPathObject {
    xpath_type: c_ushort,
    node_set: *NodeSet,
    bool_val: c_int,
    float_val: c_double,
    string_val: *c_char,
    user: *c_void,
    index: c_int,
    user2: *c_void,
    index2: c_int
}
pub struct NodeSet {
    node_nr: c_int,
    node_max: c_int,
    node_tab: *Node
}
pub struct Node {
    private: *c_void,
    element_type: c_ushort,
    name: *c_char,
    children: *Node,
    last: *Node,
    parent: *Node,
    next: *Node,
    prev: *Node,
    doc: *Document,
    namespace: *Namespace,
    content: *c_char,
    properties: *Attribute,
    namespace_def: *Namespace,
    psvi: *c_void,
    line: c_ushort,
    extra: c_ushort
}
pub struct Namespace;
pub struct Attribute;

#[link(name = "xml2")]
extern {
    pub fn xmlParseDoc(body: *c_char) -> *Document;
    pub fn xmlParseFile(filename: *c_char) -> *Document;
    pub fn xmlFreeDoc(doc: *Document);

    pub fn xmlXPathNewContext(doc: *Document) -> *Context;
    pub fn xmlXPathFreeContext(context: *Context);

    pub fn xmlXPathEvalExpression(xpath: *c_char, context: *Context) -> *XPathObject;
    pub fn xmlXPathFreeObject(object: *XPathObject);
}
