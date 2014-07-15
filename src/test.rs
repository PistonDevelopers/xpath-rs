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
    assert!(Document::from_file("test_assets/foo.xml").unwrap()
            .new_context().unwrap()
            .new_xpath_object("/foo").is_some());
}

#[test]
fn return_none_for_invalid_path() {
    assert!(Document::from_file("test_assets/foo.xml").unwrap()
            .new_context().unwrap()
            .new_xpath_object("/bar").is_none());
}
