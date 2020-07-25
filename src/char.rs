/*
    MIT License

    Copyright (c) 2020 Lars Krueger <lars_e_krueger@gmx.de>

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

//! Matcher trait implementation for token `char`.
//!
//! Provides exact and range matches.

use super::grammar::Matcher;

/// Matches single characters or ranges
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum CharMatcher {
    /// Match a single char
    Exact(char),

    /// Match a range [from, to], i.e. both limits of the interval match.
    Range(char, char),

    /// Match all but the characters in the string
    NoneOf(Vec<char>),
}

impl Matcher<char> for CharMatcher {
    fn matches(&self, t: char) -> bool {
        match self {
            CharMatcher::Exact(c) => t == *c,
            CharMatcher::Range(from, to) => (*from <= t) && (t <= *to),
            CharMatcher::NoneOf(cs) => {
                for c in cs {
                    if *c == t {
                        return false;
                    }
                }
                true
            }
        }
    }
}

/// Check if the character before the buffer position is a newline.
///
/// Predicate for skip_backward.
pub fn start_of_line(buffer: &Vec<char>, position: usize) -> bool {
    if position == 0 {
        return true;
    }
    buffer[position - 1] == '\n'
}

/// Check if the character at the buffer position is a newline
///
/// Predicate for skip_forward
pub fn end_of_line(buffer: &Vec<char>, position: usize) -> bool {
    if position == buffer.len() {
        true
    } else {
        buffer[position] == '\n'
    }
}
