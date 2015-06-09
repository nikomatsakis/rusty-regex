#[macro_use]
pub mod macros;

pub mod util;

#[cfg(test)]
mod test;

pub trait RegexThen {
    fn match_then<'text,C>(&self,
                           text: &'text str,
                           position: usize,
                           captures: &mut Vec<Capture<'text>>,
                           continuation: &C)
                           -> Option<usize>
        where C: RegexContinuation;
}

pub trait RegexContinuation {
    fn match_continue<'text>(&self,
                             text: &'text str,
                             position: usize,
                             captures: &mut Vec<Capture<'text>>)
                             -> Option<usize>;
}

pub trait CharRange {
    fn test(&self, c: char) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Capture<'text> {
    text: &'text str,
    start: usize,
    end: usize,
}

impl<'text> Capture<'text> {
    fn to_str(self) -> &'text str {
        &self.text[self.start..self.end]
    }
}
