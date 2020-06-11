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

//! SESD public API

mod buffer;
mod char;
mod grammar;
mod parser;

pub use self::char::CharMatcher;
use buffer::Buffer;
pub use grammar::{CompiledGrammar, Grammar, Matcher, Symbol};
use parser::Parser;

/// Editor Block with Synchronous Parsing
pub struct SyncBlock<T, M>
where
    M: Matcher<T>,
{
    buffer: Buffer<T>,
    parser: Parser<T, M>,
}

impl<T, M> SyncBlock<T, M>
where
    T: Clone,
    M: Matcher<T> + Clone,
{
    pub fn new(grammar: CompiledGrammar<T, M>) -> Self {
        Self {
            buffer: Buffer::new(),
            parser: Parser::new(grammar),
        }
    }

    fn buffer_changed(&mut self) {
        self.parser.buffer_changed(self.buffer.cursor());
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer_changed();
    }

    pub fn enter(&mut self, token: T) {
        let c = self.buffer.cursor();
        self.buffer.enter(token.clone());
        self.parser.update(c, token);
    }

    pub fn append_iter<I>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
    {
        for t in iter {
            self.enter(t);
        }
    }

    pub fn move_start(&mut self) {
        self.buffer.move_start();
    }
}
