// NOTE: The only macro intentionally exported is `rusty_peg`. All
// other macros should be marked `#[doc(hidden)]` and considered
// internal implementation details.

#[macro_export]
macro_rules! rusty_regex {
    ($name:ident = $($tokens:tt)+) => {
        pub fn $name<'text>(text: &'text str) -> Option<Vec<$crate::Capture<'text>>> {
            let mut captures = vec![];
            let regex = $crate::util::CaptureRe(rusty_regex_parse_tokens!($($tokens,)*));
            println!("regex={:?}", regex);
            $crate::RegexThen::match_then(&regex, text, 0, &mut captures, &$crate::util::Accept)
                .map(|_| captures)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! rusty_regex_parse_tokens {
    () => {
        $crate::util::Accept
    };

    ($token:tt, *, ?, $($tokens:tt,)*) => {
        ($crate::util::RepeatMin(rusty_regex_parse_token!($token), 0),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, *, $($tokens:tt,)*) => {
        ($crate::util::StarMax(rusty_regex_parse_token!($token)),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, +, ?, $($tokens:tt,)*) => {
        ($crate::util::RepeatMin(rusty_regex_parse_token!($token), 1),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, +, $($tokens:tt,)*) => {
        ($crate::util::PlusMax(rusty_regex_parse_token!($token)),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, ?, $($tokens:tt,)*) => {
        ($crate::util::Question(rusty_regex_parse_token!($token)),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, $($tokens:tt,)*) => {
        (rusty_regex_parse_token!($token),
         rusty_regex_parse_tokens!($($tokens,)*))
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! rusty_regex_parse_token {
    (($($token:tt)*)) => {
        $crate::util::CaptureRe(rusty_regex_parse_tokens!($($token,)*))
    };

    ([:$i:ident:]) => {
        $crate::util::named_choices::$i
    };

    ([$($token:tt)+]) => {
        $crate::util::Choice(rusty_regex_parse_choices!($($token,)+, ()))
    };

    ($literal:expr) => {
        $crate::util::Literal($literal)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! rusty_regex_parse_choices {
    (()) => {
        NoChoice
    };

    (^, $($tokens:tt),+) => {
        NotChoice(rusty_regex_parse_choices!($($tokens),+))
    };

    ($start:expr, -, $end:expr, $($tokens:tt),+) => {
        OrChoice(RangeChoice($start, $end), rusty_regex_parse_choices!($($tokens),+))
    };

    ($c:expr, $($tokens:tt),+) => {
        OrChoice(RangeChoice($start, $end), rusty_regex_parse_choices!($($tokens),+))
    };
}
