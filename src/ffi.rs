use libc::c_char;

pub struct Document;

#[link(name = "xml2")]
extern {
    pub fn xmlParseDoc(body: *c_char) -> *Document;
    pub fn xmlParseFile(filename: *c_char) -> *Document;
    pub fn xmlFreeDoc(doc: *Document);
    pub fn xmlCleanupParser();
}
