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
//! The parser can rebuild its parsing chart on the fly, even partially. This allows it to be used
//! in interactive applications. At the moment, only the synchronous reparsing is implemented (i.e.
//! reparsing after each change of the input buffer).
//!
//! The parser generates a [Concrete Syntax Tree](https://en.wikipedia.org/wiki/Parse_tree)
//! or parse tree on the fly too. The tree is ambiguous or a forest until the start symbol has been
//! accepted by the parser. The Earley Parser has been selected because it predicts which terminal
//! and non-terminal symbols can come next for a successful parse.
//!
//! The error handling uses that property to recover from incorrect terminals by marking the chart
//! entries for the incorrect token position as erroneous and then pretend that all possible
//! non-terminals matched. This, together with the handling of ambiguous grammars, will eventually
//! find a position from which the parse can be continued until acceptance.
//!
//! Grammars are be defined by types that implement the `CompiledGrammar` trait. Currently, there
//! are two ways: at compile time using the [`grammar!` macro](macro.grammar.html) and at run time using the
//! [`DynamicGrammar` struct](struct.DynamicGrammar.html).
//!
//! There is a distinction between tokens and token matchers in the parser. Tokens are passed to
//! the parser, who asks the token matchers if they accept this token. In the simplest case token
//! and matcher are of the same type. The default implementation will check for identity to accept
//! a token. For `char` tokens, a matcher is provided that accepts exact matches and character from
//! ranges. This will simplify the implementation of the most common case.
//!
//! # Examples
//!
//! The following examples uses a very simple grammar due to size. Check the unit tests for
//! bigger examples.
//!
//! Both examples implement the grammar
//!
//! ```text
//! S := NOUN ' ' NOUN
//!
//! NOUN := 'j' 'o' 'h' 'n'
//!      |
//! ```
//!
//! using `char` for tokens and `char::CharMatcher` for matchers.
//!
//! This grammar accepts the strings
//! ```text
//!   " "
//!   "john "
//!   " john"
//!   "john john"
//! ```
//!
//! If your project only requires a few grammars, it is most efficient to define them at compile
//! time, like this:
//!
//! ```
//! #[macro_use] extern crate sesd;
//!
//! grammar!{
//!   // Define the name of the mod in which the grammar is enclosed.
//!   example1,
//!   // Make the Matcher variants available inside the grammar. Any definitions made in this
//!   // section will not be visible outside the grammar.
//!   //
//!   // The braces are mandatory.
//!   {
//!     use sesd::char::CharMatcher::*;
//!   },
//!   // Type of the tokens
//!   char,
//!   // Type of the token matcher
//!   sesd::char::CharMatcher,
//!   // Name of start symbol. Must be a valid identifier for a constant.
//!   //
//!   // The start symbol needs to be declared below as either empty or non-empty.
//!   S,
//!   // List of non-terminals that match the empty set.
//!   //
//!   // The constants associated with the symbols are public.
//!   //
//!   // Empty rules for non-terminals do not need to be added in the rules below.
//!   [NOUN],
//!   // List of all non-terminals that do not match the empty set.
//!   //
//!   // Even if you add a rule to match the empty set, but list the symbol in the following list,
//!   // the empty rule will be ignored.
//!   //
//!   // The constants associated with the symbols are public.
//!   [S],
//!   // List of matchers. Each matcher must be given a symbol and constant value.
//!   //
//!   // The empty rule for NOUN has been left out because NOUN has been declared as empty rule
//!   // before.
//!   [
//!       T_J = Exact('j'),
//!       T_O = Exact('o'),
//!       T_H = Exact('h'),
//!       T_N = Exact('n'),
//!       T_SPACE = Exact(' ')
//!   ],
//!   // List of rules. Non-terminals and matchers can be mixed freely.
//!   [
//!       S = NOUN T_SPACE NOUN,
//!       NOUN = T_J T_O T_H T_N
//!   ]
//! }
//!
//! // Create an instance of the grammar. For compile-time grammars, it allocates nothing.
//! let grammar = example1::grammar();
//!
//! // Instantiate a parser
//! use sesd::Parser;
//! use sesd::Verdict::*;
//! let mut parser = Parser::<char, sesd::char::CharMatcher, example1::Grammar>::new(grammar);
//!
//! // Parse 'john john'
//! for (i, (c, v)) in [
//!    ('j', More),
//!    ('o', More),
//!    ('h', More),
//!    ('n', More),
//!    // This token is accepted as a complete parse due to the empty rule.
//!    // In practice, you'd need to check the CST to identify which case has caused this acceptance.
//!    (' ', Accept),
//!    ('j', More),
//!    ('o', More),
//!    ('h', More),
//!    ('n', Accept),
//!    ].iter().enumerate() {
//!     let res = parser.update(i, *c);
//!     assert_eq!(res, *v);
//! }
//! // Print the parse chart at the end. Inspect it to learn how an Earley parser works.
//! parser.print_chart();
//! ```
//!
//! If your project deals with user-defined grammars, or you need to make the grammar configurable,
//! the following example will illustrate the process. For ease of use, the definition of the
//! grammar is done by representing the non-terminals as strings. Identical strings refer to the
//! same non-terminal. When the textual representation of the grammar has been built, it needs to
//! be compiled to a more efficient representation, an instance of `DynamicGrammar`. This can be
//! used to constuct a parser as before.
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
//! // Noun ::=
//! // In contrast to before, the empty rule needs to be added.
//! grammar.add( TextRule::new( "Noun"));
//!
//! // Compile the textual representation to a machine-readable format. In practice, the error
//! // needs to be handled correctly.
//! let compiled_grammar = grammar.compile().expect("compilation should have worked");
//!
//! // Construct the parser.
//! use sesd::Verdict::*;
//! let mut parser = Parser::<char, CharMatcher, DynamicGrammar<char,CharMatcher>>::new(compiled_grammar);
//!
//! // Parse 'john john'
//! for (i, (c, v)) in [
//!    ('j', More),
//!    ('o', More),
//!    ('h', More),
//!    ('n', More),
//!    // This token is accepted as a complete parse due to the empty rule.
//!    // In practice, you'd need to check the CST to identify which case has caused this acceptance.
//!    (' ', Accept),
//!    ('j', More),
//!    ('o', More),
//!    ('h', More),
//!    ('n', Accept),
//!    ].iter().enumerate() {
//!     let res = parser.update(i, *c);
//!     assert_eq!(res, *v);
//! }
//! // Print the parse chart at the end. Inspect it to learn how an Earley parser works. You will
//! // see the empty rules being predicted, but never matched. This is considered an inefficiency,
//! // not a bug.
//! parser.print_chart();
//!
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
