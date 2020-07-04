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
pub mod char;
mod grammar;
mod parser;

use buffer::Buffer;
pub use grammar::{CompiledGrammar, DottedRule, Grammar, Matcher, Symbol, SymbolId, ERROR_ID};
use parser::Parser;
pub use parser::{CstIter, CstIterItem, CstIterItemNode, CstPath};

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
        self.reparse(c);
    }

    pub fn delete(&mut self, n: usize) {
        self.buffer.delete(n);
        let c = self.buffer.cursor();
        self.reparse(c);
    }

    pub fn reparse(&mut self, start: usize) {
        for (i, t) in self.buffer.token_from_iter(start) {
            self.parser.update(i, t.clone());
        }
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

    pub fn cst_iter(&self) -> CstIter<T, M> {
        self.parser.cst_iter()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn parser<'a>(&'a self) -> &Parser<T, M> {
        &self.parser
    }

    pub fn grammar<'a>(&'a self) -> &CompiledGrammar<T, M> {
        self.parser.grammar()
    }

    pub fn move_forward(&mut self, steps: usize) {
        self.buffer.move_forward(steps)
    }

    pub fn move_backward(&mut self, steps: usize) -> bool {
        self.buffer.move_backward(steps)
    }

    pub fn cursor(&self) -> usize {
        self.buffer.cursor()
    }

    /// Set the cursor to the given index, if valid
    pub fn set_cursor(&mut self, index: usize) {
        self.buffer.set_cursor(index)
    }

    /// Search from the given position forward through the tokens until the predicate becomes true.
    ///
    /// If the given position is invalid, None will be returned.
    ///
    /// Return None, if the index wasn't found. Otherwise, return the index at which the predicate
    /// became true.
    pub fn search_forward<F>(&self, start: usize, mut until: F) -> Option<usize>
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.search_forward(start, until)
    }

    /// Search from the given position backward through the tokens until the predicate becomes true.
    ///
    /// If the given position is invalid, None will be returned.
    ///
    /// Return None, if the index wasn't found. Otherwise, return the index at which the predicate
    /// became true.
    pub fn search_backward<F>(&self, start: usize, mut until: F) -> Option<usize>
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.search_backward(start, until)
    }

    /// Move the cursor forward until the predicate becomes true
    pub fn skip_forward<F>(&mut self, until: F)
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.skip_forward(until)
    }

    /// Move the cursor backward until the predicate becomes true
    pub fn skip_backward<F>(&mut self, until: F)
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.skip_backward(until)
    }
}

impl<M> SyncBlock<char, M>
where
    M: Matcher<char>,
{
    pub fn span_string(&self, start: usize, end: usize) -> String {
        use std::iter::FromIterator;
        String::from_iter(self.buffer.span(start, end).iter())
    }
}
