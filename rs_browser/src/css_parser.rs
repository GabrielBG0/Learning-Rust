use css::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unity, Value};
use std::iter::Peekable;
use std::str::Chars;

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CssParser {}
