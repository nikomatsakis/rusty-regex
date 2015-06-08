rusty_regex! { literal_re = "hi" }

#[test]
fn literal() {
    assert_eq!(literal_re("hi").unwrap(), 2);
    assert!(literal_re("ho").is_none());
}
