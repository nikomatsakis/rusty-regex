// NOTE: The only macro intentionally exported is `rusty_peg`. All
// other macros should be marked `#[doc(hidden)]` and considered
// internal implementation details.

#[macro_export]
macro_rules! rusty_regex {
    ($name:ident = ^ $($tokens:tt)+) => {
        pub fn $name<'text>(text: &'text str) -> Option<Vec<$crate::Capture<'text>>> {
            let mut captures = vec![];
            let regex = $crate::util::CaptureRe(rusty_regex_parse_tokens!($($tokens,)*));
            $crate::RegexThen::match_then(&regex, text, 0, &mut captures, &$crate::util::Accept)
                .map(|_| captures)
        }
    };

    // if no leading `^` is provided, insert an implicit `.*?`
    ($name:ident = $($tokens:tt)+) => {
        pub fn $name<'text>(text: &'text str) -> Option<Vec<$crate::Capture<'text>>> {
            let mut captures = vec![];
            let regex =
                ($crate::util::StarMin($crate::util::Choice($crate::util::YesChoice)),
                 $crate::util::CaptureRe(rusty_regex_parse_tokens!($($tokens,)*)));
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
        ($crate::util::StarMin(rusty_regex_parse_token!($token)),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, *, $($tokens:tt,)*) => {
        ($crate::util::StarMax(rusty_regex_parse_token!($token)),
         rusty_regex_parse_tokens!($($tokens,)*))
    };

    ($token:tt, +, ?, $($tokens:tt,)*) => {
        ($crate::util::PlusMin(rusty_regex_parse_token!($token)),
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
    ((? : $($token:tt)*)) => {
        rusty_regex_parse_tokens!($($token,)*)
    };

    (($($token:tt)*)) => {
        $crate::util::CaptureRe(rusty_regex_parse_tokens!($($token,)*))
    };

    ([$($token:tt)+]) => {
        $crate::util::Choice(rusty_regex_parse_choices!($($token,)+))
    };

    (.) => {
        $crate::util::Choice($crate::util::YesChoice)
    };

    (END) => {
        $crate::util::End
    };

    ($literal:expr) => {
        $crate::util::Literal($literal)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! rusty_regex_parse_choices {
    () => {
        $crate::util::NoChoice
    };

    (:, $i:ident, :, $($tokens:tt,)*) => {
        $crate::util::OrChoice(
            $crate::util::named_choices::$i,
            rusty_regex_parse_choices!($($tokens,)*))
    };

    (^, $($tokens:tt,)*) => {
        $crate::util::NotChoice(rusty_regex_parse_choices!($($tokens,)*))
    };

    ($start:expr, -, $end:expr, $($tokens:tt,)*) => {
        $crate::util::OrChoice(
            $crate::util::RangeChoice($start, $end),
            rusty_regex_parse_choices!($($tokens,)*))
    };

    ($c:expr, $($tokens:tt,)*) => {
        $crate::util::OrChoice(
            $crate::util::CharChoice($c),
            rusty_regex_parse_choices!($($tokens,)*))
    };
}
