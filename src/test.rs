rusty_regex! { literal_re = "hi" }

#[test]
fn literal() {
    assert_eq!(literal_re("hi").unwrap(), 2);
    assert!(literal_re("ho").is_none());
}

rusty_regex! { literal_star_re = "hi"* }

#[test]
fn literal_star() {
    assert!(literal_star_re("hihihi").unwrap() == 6);
    assert!(literal_star_re("hihiho").unwrap() == 4);
    assert!(literal_star_re("hihohi").unwrap() == 2);
    assert!(literal_star_re("hohihi").unwrap() == 0);
}

rusty_regex! { literal_plus_re = "hi"+ }

#[test]
fn literal_plus() {
    assert!(literal_plus_re("hihihi").unwrap() == 6);
    assert!(literal_plus_re("hihiho").unwrap() == 4);
    assert!(literal_plus_re("hihohi").unwrap() == 2);
    assert!(literal_plus_re("hohihi").is_none());
}

rusty_regex! { literal_literal_re = "hi" "ho"}

#[test]
fn literal_literal() {
    assert!(literal_literal_re("hihihi").is_none());
    assert!(literal_literal_re("hihiho").is_none());
    assert!(literal_literal_re("hihohi").unwrap() == 4);
    assert!(literal_literal_re("hohihi").is_none());
}
