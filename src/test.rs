use Capture;

///////////////////////////////////////////////////////////////////////////

rusty_regex! { literal_re = ^ "hi" }

#[test]
fn literal() {
    assert_eq!(literal_re("hi").unwrap()[0].end, 2);
    assert!(literal_re("ho").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { literal_star_re = ^ "hi"* }

#[test]
fn literal_star() {
    assert!(literal_star_re("hihihi").unwrap()[0].end == 6);
    assert!(literal_star_re("hihiho").unwrap()[0].end == 4);
    assert!(literal_star_re("hihohi").unwrap()[0].end == 2);
    assert!(literal_star_re("hohihi").unwrap()[0].end == 0);
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { literal_plus_re = ^ "hi"+ }

#[test]
fn literal_plus() {
    assert!(literal_plus_re("hihihi").unwrap()[0].end == 6);
    assert!(literal_plus_re("hihiho").unwrap()[0].end == 4);
    assert!(literal_plus_re("hihohi").unwrap()[0].end == 2);
    assert!(literal_plus_re("hohihi").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { literal_literal_re = ^ "hi" "ho"}

#[test]
fn literal_literal() {
    assert!(literal_literal_re("hihihi").is_none());
    assert!(literal_literal_re("hihiho").is_none());
    assert!(literal_literal_re("hihohi").unwrap()[0].end == 4);
    assert!(literal_literal_re("hohihi").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { literal_star_literal_re = ^ ("hi"*) "ho"}

#[test]
fn literal_star_literal() {
    assert!(literal_star_literal_re("hihihi").is_none());
    assert!(literal_star_literal_re("hihiho").is_some());
    assert!(literal_star_literal_re("hihiho").unwrap()[1].end == 4);
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { star_plus_re = ^ ("hi"*) ("hi"+) }

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

///////////////////////////////////////////////////////////////////////////

rusty_regex! { star_min_plus_re = ^ ("hi"*?) ("hi"+) }

#[test]
fn star_min_plus() {
    assert_eq!(star_min_plus_re("hihihi").unwrap(), vec![
        Capture { text: "hihihi", start: 0, end: 6 },
        Capture { text: "hihihi", start: 0, end: 0 },
        Capture { text: "hihihi", start: 0, end: 6 }
        ]);

    assert_eq!(star_min_plus_re("hi").unwrap(), vec![
        Capture { text: "hi", start: 0, end: 2 },
        Capture { text: "hi", start: 0, end: 0 },
        Capture { text: "hi", start: 0, end: 2 }
        ]);
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { star_min_hi_plus_ho_end_re = ^ (['a' 'c']*?) (['b' 'c']+) END }

#[test]
fn star_min_hi_plus_ho() {
    assert_eq!(star_min_hi_plus_ho_end_re("aacbbc").unwrap(), vec![
        Capture { text: "aacbbc", start: 0, end: 6 },
        Capture { text: "aacbbc", start: 0, end: 2 },
        Capture { text: "aacbbc", start: 2, end: 6 },
        ]);

    assert_eq!(star_min_hi_plus_ho_end_re("aacabbc").unwrap(), vec![
        Capture { text: "aacabbc", start: 0, end: 7 },
        Capture { text: "aacabbc", start: 0, end: 4 },
        Capture { text: "aacabbc", start: 4, end: 7 },
        ]);
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { plus_plus_re = ^ ("hi"+) ("hi"+) }

#[test]
fn plus_plus() {
    assert_eq!(plus_plus_re("hihihi").unwrap(), vec![
        Capture { text: "hihihi", start: 0, end: 6 },
        Capture { text: "hihihi", start: 0, end: 4 },
        Capture { text: "hihihi", start: 4, end: 6 }
        ]);

    assert!(plus_plus_re("hi").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { paren_no_cap_re = ^ (?:"hi" "hi")+ ("hi"+) }

#[test]
fn paren_no_cap() {
    assert_eq!(paren_no_cap_re("hihihi").unwrap(), vec![
        Capture { text: "hihihi", start: 0, end: 6 },
        Capture { text: "hihihi", start: 4, end: 6 }
        ]);
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { char_range_re = ^ ['a' 'b' 'c']+ }

#[test]
fn char_range() {
    assert!(char_range_re("abc").is_some());
    assert!(char_range_re("def").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { inv_char_range_re = ^ [^ 'a' 'b' 'c']+ }

#[test]
fn inv_char_range() {
    assert!(inv_char_range_re("abc").is_none());
    assert!(inv_char_range_re("def").is_some());
}


///////////////////////////////////////////////////////////////////////////

rusty_regex! { end_re = ^ ['a' 'b' 'c']* END }

#[test]
fn end() {
    assert!(end_re("").is_some());
    assert!(end_re("abcabc").is_some());
    assert!(end_re("abcabcd").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { digit_re = ^ [:digit:]* END }

#[test]
fn digit() {
    assert!(digit_re("0123").is_some());
    assert!(digit_re("0 123").is_none());
    assert!(digit_re("abc").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { digit_or_space_re = ^ [:digit: :space:]* END }

#[test]
fn digit_or_space() {
    assert!(digit_or_space_re("0123").is_some());
    assert!(digit_or_space_re("0 123").is_some());
    assert!(digit_or_space_re("abc").is_none());
}

///////////////////////////////////////////////////////////////////////////

rusty_regex! { dot_re = ^ .* END }

#[test]
fn dot() {
    assert!(dot_re("0123").is_some());
    assert!(dot_re("0 123").is_some());
    assert!(dot_re("abc").is_some());
    assert!(dot_re("").is_some());
}


///////////////////////////////////////////////////////////////////////////

rusty_regex! { missing_anchor_re = "a"+ }

#[test]
fn missing_anchor() {
    // contains an 'a':
    assert!(missing_anchor_re("gjoijqpavadsaf").is_some());

    // does not:
    assert!(missing_anchor_re("gjoijqpvdsf").is_none());
}

