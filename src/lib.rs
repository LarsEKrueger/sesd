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

//! Structured Editing of Stream Data
//!
//! Experimental library to combine a parser and a (text) editor buffer. While the primary use is a
//! text editor, the library is supposed to be flexible enough to operate on any kind of symbol.
//!
//! The parser is based on the [Earley Parser](https://en.wikipedia.org/wiki/Earley_parser). The
//! editor buffer is a vector holding tokens of an arbitrary type. The tokens have very few
//! requirements regarding the traits they need to implement, primarily `Clone` and `PartialEq`.
//!
//! The parser can rebuild its parsing chart on the fly, even partially. This allows it to be in
//! interactive applications. At the moment, only the synchronous reparsing is implemented (i.e.
//! reparsing after each change of the input buffer).
//!
//! The parser generates a [Concrete Syntax Tree](https://en.wikipedia.org/wiki/Parse_tree)
//! or parse tree on the fly too. The tree is ambiguous or a forest until the start symbol has been
//! accepted by the parser. The Earley Parser has been selected because it predicts which terminal
//! and non-terminal symbols can come next for a successful parse.
//!
//! The error handling uses that property to recover from incorrect terminal by marking the chart
//! entries for the incorrect token position as erroneous and then pretend that all possible
//! non-terminals matched. This, together with the handling of ambiguous grammar, will eventually
//! find a position from which the parse can be continued until acceptance.
//!
//! # Examples
//!
//! The following example uses a very simple grammar due to size. Check the unit tests for
//! bigger examples.
//!
//! ```
//! use sesd::{char::CharMatcher, TextGrammar, DynamicGrammar, Parser, TextRule, Verdict};
//!
//! use CharMatcher::*;
//! let mut grammar: TextGrammar<char, CharMatcher> = TextGrammar::new();
//!
//! grammar.set_start("S".to_string());
//! // S ::= Noun ' ' Noun
//! grammar.add( TextRule::new( "S").nt("Noun").t(Exact(' ')).nt("Noun"));
//! // Noun ::= 'j' 'o' 'h' 'n'
//! grammar.add( TextRule::new( "Noun").
//!         t(Exact('j')).
//!         t(Exact('o')).
//!         t(Exact('h')).
//!         t(Exact('n')));
//!
//! let compiled_grammar = grammar.compile().expect("compilation should have worked");
//!
//! let mut parser = Parser::<char, CharMatcher, DynamicGrammar<char,CharMatcher>>::new(compiled_grammar);
//! let mut position = 0;
//! for (i, c) in "john joh".chars().enumerate() {
//!     let res = parser.update(i, c);
//!     assert_eq!(res, Verdict::More);
//!     position = i;
//! }
//! let res = parser.update(position+1, 'n');
//! assert_eq!(res, Verdict::Accept);
//! ```

#[macro_use]
extern crate log;

use std::marker::PhantomData;

mod buffer;
pub mod char;
mod dynamic_grammar;
mod grammar;
mod parser;
pub mod style_sheet;

use buffer::Buffer;
pub use dynamic_grammar::{DynamicGrammar, Error, TextGrammar, TextRule, TextSymbol};
pub use grammar::{CompiledGrammar, Matcher, SymbolId, ERROR_ID};
pub use parser::{CstIter, CstIterItem, CstIterItemNode, CstPath, Parser, Verdict};

/// Editor with synchronous parsing.
///
/// Provides a buffer for tokens and a parser. Edit operation trigger a re-parse of the changed
/// part of the buffer.
///
/// The grammar is not meant to be changed on the fly, but there is no technical limitations
/// against that. File a feature request if you need it.
pub struct SynchronousEditor<T, M, G>
where
    M: Matcher<T>,
    G: CompiledGrammar<T, M>,
{
    /// Token buffer
    buffer: Buffer<T>,
    /// Parser
    parser: Parser<T, M, G>,

    /// Phantom data to make compiler happy
    _marker: PhantomData<M>,
}

impl<T, M, G> SynchronousEditor<T, M, G>
where
    T: Clone,
    M: Matcher<T> + Clone,
    G: CompiledGrammar<T, M>,
{
    /// Create a new parser with an empty buffer.
    pub fn new(grammar: G) -> Self {
        Self {
            buffer: Buffer::new(),
            parser: Parser::new(grammar),
            _marker: PhantomData,
        }
    }

    fn buffer_changed(&mut self) {
        self.parser.buffer_changed(self.buffer.cursor());
    }

    /// Remove all content from the token buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer_changed();
    }

    /// Insert a single token at the cursor position, then advance the cursor by one token.
    ///
    /// Triggers a re-parse.
    pub fn enter(&mut self, token: T) {
        let c = self.buffer.cursor();
        self.buffer.enter(token.clone());
        self.reparse(c);
    }

    /// Delete n tokens to the right of the current cursor position.
    ///
    /// Triggers a re-parse.
    pub fn delete(&mut self, n: usize) {
        self.buffer.delete(n);
        let c = self.buffer.cursor();
        self.reparse(c);
    }

    /// Trigger a re-parse.
    ///
    /// Parse errors are silently ignored and inserted into the CST.
    fn reparse(&mut self, start: usize) {
        // Mark the buffer as changed at start, even if the rest has been deleted
        self.parser.buffer_changed(start);
        for (i, t) in self.buffer.token_from_iter(start) {
            self.parser.update(i, t.clone());
        }
    }

    /// Enter tokens as long as an iterator can provide them
    ///
    /// Triggers a re-parse at the end of the iterator.
    pub fn enter_iter<I>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
    {
        let c = self.buffer.cursor();
        for t in iter {
            self.buffer.enter(t);
        }
        self.reparse(c);
    }

    /// Move the cursor to the start of the buffer.
    pub fn move_start(&mut self) {
        self.buffer.move_start();
    }

    /// Create a new iterator to traverse the parse tree in pre-order.
    pub fn cst_iter(&self) -> CstIter<T, M, G> {
        self.parser.cst_iter()
    }

    /// Number of tokens in the buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Borrow the parser for reading.
    pub fn parser<'a>(&'a self) -> &Parser<T, M, G> {
        &self.parser
    }

    /// Borrow the compiled grammar from inside the parser
    pub fn grammar<'a>(&'a self) -> &G {
        self.parser.grammar()
    }

    /// Move the cursor a number of positions towards the end of the buffer.
    pub fn move_forward(&mut self, steps: usize) {
        self.buffer.move_forward(steps)
    }

    /// Move the cursor a number of positions towards the beginning of the buffer.
    ///
    /// Returns true if the cursor was moved.
    pub fn move_backward(&mut self, steps: usize) -> bool {
        self.buffer.move_backward(steps)
    }

    /// Return the cursor position in the buffer
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
    pub fn search_forward<F>(&self, start: usize, until: F) -> Option<usize>
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
    pub fn search_backward<F>(&self, start: usize, until: F) -> Option<usize>
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.search_backward(start, until)
    }

    /// Move the cursor towards the end of the buffer until the predicate becomes true
    pub fn skip_forward<F>(&mut self, until: F)
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.skip_forward(until)
    }

    /// Move the cursor towards the beginning of the buffer until the predicate becomes true
    pub fn skip_backward<F>(&mut self, until: F)
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        self.buffer.skip_backward(until)
    }

    /// List of symbols predicted at the cursor position
    pub fn predictions_at_cursor(&self) -> Vec<SymbolId> {
        self.parser.predictions(self.buffer.cursor())
    }

    /// Replace a section of the buffer by new tokens
    ///
    /// Place the cursor at the end of the inserted text and reparse from start.
    pub fn replace<I>(&mut self, start: usize, end: usize, iter: I)
    where
        I: Iterator<Item = T>,
    {
        self.buffer.delete_range(start, end);
        self.buffer.set_cursor(start);
        self.enter_iter(iter);
    }
}

impl<M, G> SynchronousEditor<char, M, G>
where
    M: Matcher<char>,
    G: CompiledGrammar<char, M>,
{
    /// For an editor holding tokens of type `char`s, return a string of the tokens beginning at
    /// position `start` and including the token before at position `end`.
    pub fn span_string(&self, start: usize, end: usize) -> String {
        use std::iter::FromIterator;
        String::from_iter(self.buffer.span(start, end).iter())
    }

    /// Copy the whole buffer into a string.
    pub fn as_string(&self) -> String {
        self.buffer.as_string()
    }
}
