#![cfg(feature = "derive")]

use document_validator::Document;

#[derive(Document)]
#[document(validator = "my_validation_logic")]
pub struct MyDocument(Box<str>);

fn my_validation_logic(document: &str) -> bool {
    document.len() == 5
}

#[test]
fn validation() {
    assert!(MyDocument::validate("12345"));
    assert!(!MyDocument::validate(""));
}

#[test]
fn construction() {
    assert!(MyDocument::new("12345").is_some());
    assert!(MyDocument::new("").is_none());
}

#[test]
fn as_str() {
    let document = MyDocument::new("12345").unwrap();
    assert_eq!(document.as_str(), "12345")
}
