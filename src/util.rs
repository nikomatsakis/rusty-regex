use super::{CharRange, Capture, RegexThen, RegexContinuation};

#[derive(Clone, Debug)]
pub struct Accept;

impl RegexThen for Accept {
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        continuation.match_continue(text, position, captures)
    }
}

impl RegexContinuation for Accept {
    fn match_continue<'text>(&self,
                             _text: &'text str,
                             position: usize,
                             _captures: &mut Vec<Capture<'text>>)
                             -> Option<usize>
    {
        Some(position)
    }
}

#[derive(Clone, Debug)]
pub struct End;

impl RegexThen for End {
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        if position == text.len() {
            continuation.match_continue(text, position, captures)
        } else {
            None
        }
    }
}

impl<R,U> RegexThen for (R,U)
    where R: RegexThen, U: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        let state = SeqMidState { next: &self.1, continuation: continuation };
        self.0.match_then(text, position, captures, &state)
    }
}

struct SeqMidState<'r,R:'r,C:'r> {
    next: &'r R,
    continuation: &'r C,
}

impl<'r,R,C> RegexContinuation for SeqMidState<'r,R,C>
    where R: RegexThen, C: RegexContinuation
{
    fn match_continue<'text>(&self,
                             text: &'text str,
                             position: usize,
                             captures: &mut Vec<Capture<'text>>)
                             -> Option<usize>
    {
        self.next.match_then(text, position, captures, self.continuation)
    }
}

#[derive(Clone, Debug)]
pub struct StarMax<R>(pub R);

impl<R> RegexThen for StarMax<R>
    where R: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        let state = StarMaxState { repeat: &self.0, continuation: continuation };
        state.match_continue(text, position, captures)
    }
}

#[derive(Clone, Debug)]
pub struct PlusMax<R>(pub R);

impl<R> RegexThen for PlusMax<R>
    where R: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        let state = StarMaxState { repeat: &self.0, continuation: continuation };
        self.0.match_then(text, position, captures, &state)
    }
}

struct StarMaxState<'a,R:'a,C:'a> {
    repeat: &'a R,
    continuation: &'a C,
}

impl<'a,R,C> RegexContinuation for StarMaxState<'a,R,C>
    where R: RegexThen, C: RegexContinuation
{
    fn match_continue<'text>(&self,
                             text: &'text str,
                             start: usize,
                             captures: &mut Vec<Capture<'text>>)
                             -> Option<usize>
    {
        // You may be wondering "where is the loop?" The answer is
        // that the loop occurs by passing `self` as the
        // *continuation* for `self.repeat`. This means that after we
        // match the repeating part, we will resume in the same state,
        // ready to try again, but with a different start point.  Only
        // once we fail will we fallback to `self.continuation`.
        let captures_len = captures.len();
        match self.repeat.match_then(text, start, captures, self) {
            Some(end) => Some(end),
            None => {
                captures.truncate(captures_len);
                self.continuation.match_continue(text, start, captures)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StarMin<R>(pub R);

impl<R> RegexThen for StarMin<R>
    where R: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        let state = StarMinState { repeat: &self.0, continuation: continuation };
        state.match_continue(text, position, captures)
    }
}

#[derive(Clone, Debug)]
pub struct PlusMin<R>(pub R);

impl<R> RegexThen for PlusMin<R>
    where R: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        let state = StarMinState { repeat: &self.0, continuation: continuation };
        self.0.match_then(text, position, captures, &state)
    }
}

struct StarMinState<'a,R:'a,C:'a> {
    repeat: &'a R,
    continuation: &'a C,
}

impl<'a,R,C> RegexContinuation for StarMinState<'a,R,C>
    where R: RegexThen, C: RegexContinuation
{
    fn match_continue<'text>(&self,
                             text: &'text str,
                             start: usize,
                             captures: &mut Vec<Capture<'text>>)
                             -> Option<usize>
    {
        // First try what comes after us:
        let captures_len = captures.len();
        match self.continuation.match_continue(text, start, captures) {
            Some(end) => Some(end),
            None => {
                // If that fails, then try the repeat and come back to this point:
                captures.truncate(captures_len);
                self.repeat.match_then(text, start, captures, self)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Question<R>(pub R);

impl<R> RegexThen for Question<R>
    where R: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        match self.0.match_then(text, position, captures, continuation) {
            Some(end) => Some(end),
            None => continuation.match_continue(text, position, captures),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CaptureRe<R>(pub R);

impl<R> RegexThen for CaptureRe<R>
    where R: RegexThen
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        let index = captures.len();

        let post_capture = PostCaptureRe {
            index: index,
            continuation: continuation,
        };

        captures.push(Capture { text: text, start: position, end: position });

        self.0.match_then(text, position, captures, &post_capture)
    }
}

struct PostCaptureRe<'r, C:'r> {
    index: usize,
    continuation: &'r C
}

impl<'r, C> RegexContinuation for PostCaptureRe<'r, C>
    where C: RegexContinuation
{
    fn match_continue<'text>(&self,
                             text: &'text str,
                             position: usize,
                             captures: &mut Vec<Capture<'text>>)
                             -> Option<usize>
    {
        captures[self.index].end = position;
        self.continuation.match_continue(text, position, captures)
    }
}

#[derive(Clone, Debug)]
pub struct Literal(pub &'static str);

impl RegexThen for Literal {
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        if text[position..].starts_with(self.0) {
            let len = self.0.len();
            continuation.match_continue(text, position + len, captures)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Choice<CR:CharRange>(pub CR);

impl<CR> RegexThen for Choice<CR>
    where CR: CharRange
{
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation
    {
        if let Some(c) = text[position..].chars().next() {
            if self.0.test(c) {
                let l = c.len_utf8();
                return continuation.match_continue(text, position + l, captures);
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct CharChoice(pub char);

impl CharRange for CharChoice {
    fn test(&self, c: char) -> bool {
        self.0 == c
    }
}

#[derive(Clone, Debug)]
pub struct RangeChoice(char, char);

impl CharRange for RangeChoice {
    fn test(&self, c: char) -> bool {
        let lo = self.0 as u32;
        let hi = self.1 as u32;
        let c = c as u32;
        (lo <= c) && (c <= hi)
    }
}

#[derive(Clone, Debug)]
pub struct NotChoice<CR>(pub CR);

impl<CR:CharRange> CharRange for NotChoice<CR> {
    fn test(&self, c: char) -> bool {
        !self.0.test(c)
    }
}

#[derive(Clone, Debug)]
pub struct OrChoice<CR1,CR2>(pub CR1, pub CR2);

impl<CR1:CharRange,CR2:CharRange> CharRange for OrChoice<CR1,CR2> {
    fn test(&self, c: char) -> bool {
        self.0.test(c) || self.1.test(c)
    }
}

#[derive(Clone, Debug)]
pub struct NoChoice;

impl CharRange for NoChoice {
    fn test(&self, _: char) -> bool {
        false
    }
}

pub mod named_choices {
    #![allow(non_camel_case_types)]
    use CharRange;

    pub struct alpha;

    impl CharRange for alpha {
        fn test(&self, c: char) -> bool {
            char::is_alphabetic(c)
        }
    }

    pub struct digit;

    impl CharRange for digit {
        fn test(&self, c: char) -> bool {
            c >= '0' && c <= '9'
        }
    }

    pub struct space;

    impl CharRange for space {
        fn test(&self, c: char) -> bool {
            char::is_whitespace(c)
        }
    }
}


