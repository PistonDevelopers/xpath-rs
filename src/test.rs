// -*- flycheck-rust-crate-root: "lib.rs" -*-
use super::Document;

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
    assert!(Document::from_file("test_assets/foo.xml").is_some());
}

#[test]
fn return_valid_xpath_object_for_extent_path() {
    let document = Document::from_file("test_assets/foo.xml").unwrap();
    let context = document.new_context().unwrap();
    let xpath_object = context.new_xpath_object("/foo");
    assert!(xpath_object.is_some());
    let xpath_object = xpath_object.unwrap();
    assert!(xpath_object.get_node_set().is_some());
 }

#[test]
fn return_right_amount_of_nodes() {
    let document = Document::from_file("test_assets/foo.xml").unwrap();
    let context = document.new_context().unwrap();
    let xpath_object = context.new_xpath_object("/foo/bar").unwrap();
    assert!(xpath_object.get_node_set().unwrap().get_nodes().len()==2);
}

#[test]
fn return_none_for_invalid_path() {
    assert!(Document::from_file("test_assets/foo.xml").unwrap()
            .new_context().unwrap()
            .new_xpath_object("/bar").is_none());
}
