use Capture;

rusty_regex! { literal_re = "hi" }

#[test]
fn literal() {
    assert_eq!(literal_re("hi").unwrap()[0].end, 2);
    assert!(literal_re("ho").is_none());
}

rusty_regex! { literal_star_re = "hi"* }

#[test]
fn literal_star() {
    assert!(literal_star_re("hihihi").unwrap()[0].end == 6);
    assert!(literal_star_re("hihiho").unwrap()[0].end == 4);
    assert!(literal_star_re("hihohi").unwrap()[0].end == 2);
    assert!(literal_star_re("hohihi").unwrap()[0].end == 0);
}

rusty_regex! { literal_plus_re = "hi"+ }

#[test]
fn literal_plus() {
    assert!(literal_plus_re("hihihi").unwrap()[0].end == 6);
    assert!(literal_plus_re("hihiho").unwrap()[0].end == 4);
    assert!(literal_plus_re("hihohi").unwrap()[0].end == 2);
    assert!(literal_plus_re("hohihi").is_none());
}

rusty_regex! { literal_literal_re = "hi" "ho"}

#[test]
fn literal_literal() {
    assert!(literal_literal_re("hihihi").is_none());
    assert!(literal_literal_re("hihiho").is_none());
    assert!(literal_literal_re("hihohi").unwrap()[0].end == 4);
    assert!(literal_literal_re("hohihi").is_none());
}

rusty_regex! { literal_star_literal_re = ("hi"*) "ho"}

#[test]
fn literal_star_literal() {
    assert!(literal_star_literal_re("hihihi").is_none());
    assert!(literal_star_literal_re("hihiho").is_some());
    assert!(literal_star_literal_re("hihiho").unwrap()[1].end == 4);
}

rusty_regex! { star_plus_re = ("hi"*) ("hi"+) }

#[test]
fn star_plus() {
    assert_eq!(star_plus_re("hihihi").unwrap(), vec![
        Capture { text: "hihihi", start: 0, end: 6 },
        Capture { text: "hihihi", start: 0, end: 4 },
        Capture { text: "hihihi", start: 4, end: 6 }
        ]);

    assert_eq!(star_plus_re("hi").unwrap(), vec![
        Capture { text: "hi", start: 0, end: 2 },
        Capture { text: "hi", start: 0, end: 0 },
        Capture { text: "hi", start: 0, end: 2 }
        ]);
}

rusty_regex! { plus_plus_re = ("hi"+) ("hi"+) }

#[test]
fn plus_plus() {
    assert_eq!(plus_plus_re("hihihi").unwrap(), vec![
        Capture { text: "hihihi", start: 0, end: 6 },
        Capture { text: "hihihi", start: 0, end: 4 },
        Capture { text: "hihihi", start: 4, end: 6 }
        ]);

    assert!(plus_plus_re("hi").is_none());
}

