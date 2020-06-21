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

//! Compiled-in data for Cargo.toml files
//!
//! This is based on https://github.com/toml-lang/toml/blob/master/toml.abnf, which is
//! MIT licensed.

use sesd::{CharMatcher, CompiledGrammar, Grammar, Symbol};

use super::style_sheet::{Style, StyleSheet, SymbolMatcher};

/// Build the grammar for TOML files
pub fn grammar() -> CompiledGrammar<char, CharMatcher> {
    let mut grammar = grammar_nostart();

    grammar.set_start("toml".to_string());

    grammar
        .compile()
        .expect("compiling built-in grammar should not fail")
}

// Style Builder
struct SB {
    pub s: Style,
}

impl SB {
    fn new() -> Self {
        Self { s: Style::none() }
    }

    fn b(mut self) -> Self {
        self.s.attr.set_bold(true);
        self
    }

    fn i(mut self) -> Self {
        self.s.attr.set_italic(true);
        self
    }

    fn u(mut self) -> Self {
        self.s.attr.set_underline(true);
        self
    }

    fn cp(mut self, c: pancurses::ColorPair) -> Self {
        self.s.attr.set_color_pair(c);
        self
    }
}

/// Build the style sheet for Cargo.toml files
pub fn style_sheet(grammar: &CompiledGrammar<char, CharMatcher>) -> StyleSheet {
    let mut sheet = StyleSheet::new(Style::none());

    // Table headers, underlined
    sheet.add(
        vec![
            SymbolMatcher::Exact(grammar.nt_id("toml")),
            SymbolMatcher::Star(grammar.nt_id("expressions")),
            SymbolMatcher::Exact(grammar.nt_id("expression")),
            SymbolMatcher::Exact(grammar.nt_id("table")),
        ],
        SB::new().u().s,
    );

    // Comments, italic
    sheet.add(
        vec![
            SymbolMatcher::Exact(grammar.nt_id("toml")),
            SymbolMatcher::Star(grammar.nt_id("expressions")),
            SymbolMatcher::Exact(grammar.nt_id("expression")),
            SymbolMatcher::Exact(grammar.nt_id("maybe_comment")),
            SymbolMatcher::Exact(grammar.nt_id("comment")),
        ],
        SB::new().i().s,
    );

    // Keys, cyan on black
    sheet.add(
        vec![
            SymbolMatcher::Exact(grammar.nt_id("toml")),
            SymbolMatcher::Star(grammar.nt_id("expressions")),
            SymbolMatcher::Exact(grammar.nt_id("expression")),
            SymbolMatcher::Exact(grammar.nt_id("keyval")),
            SymbolMatcher::Exact(grammar.nt_id("key")),
        ],
        SB::new().cp(pancurses::ColorPair(0o60)).s,
    );

    // String values, magenta on black
    sheet.add(
        vec![
            SymbolMatcher::Exact(grammar.nt_id("toml")),
            SymbolMatcher::Star(grammar.nt_id("expressions")),
            SymbolMatcher::Exact(grammar.nt_id("expression")),
            SymbolMatcher::Exact(grammar.nt_id("keyval")),
            SymbolMatcher::Exact(grammar.nt_id("val")),
            SymbolMatcher::Exact(grammar.nt_id("string")),
        ],
        SB::new().cp(pancurses::ColorPair(0o50)).s,
    );

    // Array values, magenta on black, underline
    sheet.add(
        vec![
            SymbolMatcher::Exact(grammar.nt_id("toml")),
            SymbolMatcher::Star(grammar.nt_id("expressions")),
            SymbolMatcher::Exact(grammar.nt_id("expression")),
            SymbolMatcher::Exact(grammar.nt_id("keyval")),
            SymbolMatcher::Exact(grammar.nt_id("val")),
            SymbolMatcher::Exact(grammar.nt_id("array")),
        ],
        SB::new().cp(pancurses::ColorPair(0o50)).u().s,
    );

    // Struct values, magenta on black, italic
    sheet.add(
        vec![
            SymbolMatcher::Exact(grammar.nt_id("toml")),
            SymbolMatcher::Star(grammar.nt_id("expressions")),
            SymbolMatcher::Exact(grammar.nt_id("expression")),
            SymbolMatcher::Exact(grammar.nt_id("keyval")),
            SymbolMatcher::Exact(grammar.nt_id("val")),
            SymbolMatcher::Exact(grammar.nt_id("inline-table")),
        ],
        SB::new().cp(pancurses::ColorPair(0o50)).i().s,
    );

    sheet
}

/// Internal function to support testing
///
/// No start symbol is set, thus sub-rules can be tested.
fn grammar_nostart() -> Grammar<char, CharMatcher> {
    let mut grammar = Grammar::<char, CharMatcher>::new();

    use CharMatcher::*;
    use Symbol::*;

    grammar.add_rule("ALPHA".to_string(), vec![Terminal(Range('A', 'Z'))]);
    grammar.add_rule("ALPHA".to_string(), vec![Terminal(Range('a', 'z'))]);
    grammar.add_rule("DIGIT".to_string(), vec![Terminal(Range('0', '9'))]);
    grammar.add_rule("HEXDIG".to_string(), vec![NonTerminal("DIGIT".to_string())]);
    grammar.add_rule("HEXDIG".to_string(), vec![Terminal(Range('A', 'F'))]);
    grammar.add_rule("HEXDIG".to_string(), vec![Terminal(Range('a', 'f'))]);
    grammar.add_rule(
        "4HEXDIG".to_string(),
        vec![
            NonTerminal("HEXDIG".to_string()),
            NonTerminal("HEXDIG".to_string()),
            NonTerminal("HEXDIG".to_string()),
            NonTerminal("HEXDIG".to_string()),
        ],
    );
    grammar.add_rule(
        "8HEXDIG".to_string(),
        vec![
            NonTerminal("4HEXDIG".to_string()),
            NonTerminal("4HEXDIG".to_string()),
        ],
    );

    grammar.add_rule(
        "ws".to_string(),
        vec![
            NonTerminal("wschar".to_string()),
            NonTerminal("ws".to_string()),
        ],
    );
    grammar.add_rule("ws".to_string(), vec![]);
    grammar.add_rule("wschar".to_string(), vec![Terminal(Exact(' '))]);
    grammar.add_rule("wschar".to_string(), vec![Terminal(Exact('\t'))]);
    grammar.add_rule("newline".to_string(), vec![Terminal(Exact('\x0A'))]);
    grammar.add_rule(
        "newline".to_string(),
        vec![Terminal(Exact('\x0D')), Terminal(Exact('\x0A'))],
    );

    grammar.add_rule(
        "comment-start-symbol".to_string(),
        vec![Terminal(Exact('#'))],
    );
    grammar.add_rule(
        "non-ascii".to_string(),
        vec![Terminal(Range('\u{80}', '\u{D7FF}'))],
    );
    grammar.add_rule(
        "non-ascii".to_string(),
        vec![Terminal(Range('\u{E000}', '\u{10FFFF}'))],
    );
    grammar.add_rule("non-eol".to_string(), vec![Terminal(Exact('\t'))]);
    grammar.add_rule("non-eol".to_string(), vec![Terminal(Range('\x20', '\x7F'))]);
    grammar.add_rule(
        "non-eol".to_string(),
        vec![NonTerminal("non-ascii".to_string())],
    );
    grammar.add_rule(
        "comment".to_string(),
        vec![
            NonTerminal("comment-start-symbol".to_string()),
            NonTerminal("non-eols".to_string()),
        ],
    );
    grammar.add_rule(
        "non-eols".to_string(),
        vec![
            NonTerminal("non-eol".to_string()),
            NonTerminal("non-eols".to_string()),
        ],
    );
    grammar.add_rule("non-eols".to_string(), vec![]);
    grammar.add_rule(
        "maybe_comment".to_string(),
        vec![NonTerminal("comment".to_string())],
    );
    grammar.add_rule("maybe_comment".to_string(), vec![]);

    grammar.add_rule(
        "table".to_string(),
        vec![NonTerminal("std-table".to_string())],
    );
    grammar.add_rule(
        "table".to_string(),
        vec![NonTerminal("array-table".to_string())],
    );
    grammar.add_rule(
        "std-table".to_string(),
        vec![
            NonTerminal("std-table-open".to_string()),
            NonTerminal("key".to_string()),
            NonTerminal("std-table-close".to_string()),
        ],
    );
    grammar.add_rule(
        "std-table-open".to_string(),
        vec![Terminal(Exact('[')), NonTerminal("ws".to_string())],
    );
    grammar.add_rule(
        "std-table-close".to_string(),
        vec![NonTerminal("ws".to_string()), Terminal(Exact(']'))],
    );
    grammar.add_rule(
        "inline-table".to_string(),
        vec![
            NonTerminal("inline-table-open".to_string()),
            NonTerminal("[inline-table-keyvals]".to_string()),
            NonTerminal("inline-table-close".to_string()),
        ],
    );
    grammar.add_rule(
        "inline-table-open".to_string(),
        vec![Terminal(Exact('{')), NonTerminal("ws".to_string())],
    );
    grammar.add_rule(
        "inline-table-close".to_string(),
        vec![NonTerminal("ws".to_string()), Terminal(Exact('}'))],
    );
    grammar.add_rule(
        "inline-table-sep".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            Terminal(Exact(',')),
            NonTerminal("ws".to_string()),
        ],
    );
    grammar.add_rule(
        "[inline-table-keyvals]".to_string(),
        vec![NonTerminal("inline-table-keyvals".to_string())],
    );
    grammar.add_rule("[inline-table-keyvals]".to_string(), vec![]);
    grammar.add_rule(
        "inline-table-keyvals".to_string(),
        vec![
            NonTerminal("keyval".to_string()),
            NonTerminal("[inline-table-sepinline-table-keyvals]".to_string()),
        ],
    );
    grammar.add_rule(
        "[inline-table-sepinline-table-keyvals]".to_string(),
        vec![
            NonTerminal("inline-table-sep".to_string()),
            NonTerminal("inline-table-keyvals".to_string()),
        ],
    );
    grammar.add_rule("[inline-table-sepinline-table-keyvals]".to_string(), vec![]);
    grammar.add_rule(
        "array-table".to_string(),
        vec![
            NonTerminal("array-table-open".to_string()),
            NonTerminal("key".to_string()),
            NonTerminal("array-table-close".to_string()),
        ],
    );
    grammar.add_rule(
        "array-table-open".to_string(),
        vec![
            Terminal(Exact('[')),
            Terminal(Exact('[')),
            NonTerminal("ws".to_string()),
        ],
    );
    grammar.add_rule(
        "array-table-close".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            Terminal(Exact(']')),
            Terminal(Exact(']')),
        ],
    );

    grammar.add_rule(
        "array".to_string(),
        vec![
            NonTerminal("array-open".to_string()),
            NonTerminal("[array-values]".to_string()),
            NonTerminal("ws-comment-newline".to_string()),
            NonTerminal("array-close".to_string()),
        ],
    );
    grammar.add_rule(
        "[array-values]".to_string(),
        vec![NonTerminal("array-values".to_string())],
    );
    grammar.add_rule("[array-values]".to_string(), vec![]);
    grammar.add_rule("array-open".to_string(), vec![Terminal(Exact('['))]);
    grammar.add_rule("array-close".to_string(), vec![Terminal(Exact(']'))]);
    grammar.add_rule(
        "array-values".to_string(),
        vec![
            NonTerminal("ws-comment-newline".to_string()),
            NonTerminal("val".to_string()),
            NonTerminal("ws".to_string()),
            NonTerminal("array-sep".to_string()),
            NonTerminal("array-values".to_string()),
        ],
    );
    grammar.add_rule(
        "array-values".to_string(),
        vec![
            NonTerminal("ws-comment-newline".to_string()),
            NonTerminal("val".to_string()),
            NonTerminal("ws".to_string()),
            NonTerminal("[array-sep]".to_string()),
        ],
    );
    grammar.add_rule(
        "[array-sep]".to_string(),
        vec![NonTerminal("array-sep".to_string())],
    );
    grammar.add_rule("[array-sep]".to_string(), vec![]);
    grammar.add_rule("array-sep".to_string(), vec![Terminal(Exact(','))]);
    grammar.add_rule(
        "ws-comment-newline".to_string(),
        vec![
            NonTerminal("wscn".to_string()),
            NonTerminal("ws-comment-newline".to_string()),
        ],
    );
    grammar.add_rule("ws-comment-newline".to_string(), vec![]);
    grammar.add_rule("wscn".to_string(), vec![NonTerminal("wschar".to_string())]);
    grammar.add_rule(
        "wscn".to_string(),
        vec![
            NonTerminal("[comment]".to_string()),
            NonTerminal("newline".to_string()),
        ],
    );
    grammar.add_rule(
        "[comment".to_string(),
        vec![NonTerminal("comment".to_string())],
    );
    grammar.add_rule("[comment]".to_string(), vec![]);

    grammar.add_rule(
        "date-time".to_string(),
        vec![NonTerminal("offset-date-time".to_string())],
    );
    grammar.add_rule(
        "date-time".to_string(),
        vec![NonTerminal("local-date-time".to_string())],
    );
    grammar.add_rule(
        "date-time".to_string(),
        vec![NonTerminal("local-date".to_string())],
    );
    grammar.add_rule(
        "date-time".to_string(),
        vec![NonTerminal("local-time".to_string())],
    );
    grammar.add_rule(
        "date-fullyear".to_string(),
        vec![NonTerminal("4DIGIT".to_string())],
    );
    grammar.add_rule(
        "4DIGIT".to_string(),
        vec![
            NonTerminal("2DIGIT".to_string()),
            NonTerminal("2DIGIT".to_string()),
        ],
    );
    grammar.add_rule(
        "2DIGIT".to_string(),
        vec![
            NonTerminal("DIGIT".to_string()),
            NonTerminal("DIGIT".to_string()),
        ],
    );
    grammar.add_rule(
        "date-month".to_string(),
        vec![NonTerminal("2DIGIT".to_string())],
    );
    grammar.add_rule(
        "date-mday".to_string(),
        vec![NonTerminal("2DIGIT".to_string())],
    );
    grammar.add_rule("time-delim".to_string(), vec![Terminal(Exact('T'))]);
    grammar.add_rule("time-delim".to_string(), vec![Terminal(Exact('t'))]);
    grammar.add_rule("time-delim".to_string(), vec![Terminal(Exact(' '))]);
    grammar.add_rule(
        "time-hour".to_string(),
        vec![NonTerminal("2DIGIT".to_string())],
    );
    grammar.add_rule(
        "time-minute".to_string(),
        vec![NonTerminal("2DIGIT".to_string())],
    );
    grammar.add_rule(
        "time-second".to_string(),
        vec![NonTerminal("2DIGIT".to_string())],
    );
    grammar.add_rule(
        "time-secfrac".to_string(),
        vec![Terminal(Exact('.')), NonTerminal("1*DIGIT".to_string())],
    );
    grammar.add_rule(
        "1*DIGIT".to_string(),
        vec![
            NonTerminal("DIGIT".to_string()),
            NonTerminal("1*DIGIT".to_string()),
        ],
    );
    grammar.add_rule(
        "1*DIGIT".to_string(),
        vec![NonTerminal("DIGIT".to_string())],
    );
    grammar.add_rule(
        "time-numoffset".to_string(),
        vec![
            NonTerminal("sign".to_string()),
            NonTerminal("time-hour".to_string()),
            Terminal(Exact(':')),
            NonTerminal("time-minute".to_string()),
        ],
    );
    grammar.add_rule("time-offset".to_string(), vec![Terminal(Exact('Z'))]);
    grammar.add_rule("time-offset".to_string(), vec![Terminal(Exact('z'))]);
    grammar.add_rule(
        "time-offset".to_string(),
        vec![NonTerminal("time-numoffset".to_string())],
    );
    grammar.add_rule(
        "partial-time".to_string(),
        vec![
            NonTerminal("time-hour".to_string()),
            Terminal(Exact(':')),
            NonTerminal("time-minute".to_string()),
            Terminal(Exact(':')),
            NonTerminal("time-second".to_string()),
            NonTerminal("[time-secfrac]".to_string()),
        ],
    );
    grammar.add_rule(
        "[time-secfrac]".to_string(),
        vec![NonTerminal("time-secfrac".to_string())],
    );
    grammar.add_rule("[time-secfrac]".to_string(), vec![]);
    grammar.add_rule(
        "full-date".to_string(),
        vec![
            NonTerminal("date-fullyear".to_string()),
            Terminal(Exact('-')),
            NonTerminal("date-month".to_string()),
            Terminal(Exact('-')),
            NonTerminal("date-mday".to_string()),
        ],
    );
    grammar.add_rule(
        "full-time".to_string(),
        vec![
            NonTerminal("partial-time".to_string()),
            NonTerminal("time-offset".to_string()),
        ],
    );
    grammar.add_rule(
        "offset-date-time".to_string(),
        vec![
            NonTerminal("full-date".to_string()),
            NonTerminal("time-delim".to_string()),
            NonTerminal("full-time".to_string()),
        ],
    );
    grammar.add_rule(
        "local-date-time".to_string(),
        vec![
            NonTerminal("full-date".to_string()),
            NonTerminal("time-delim".to_string()),
            NonTerminal("partial-time".to_string()),
        ],
    );
    grammar.add_rule(
        "local-date".to_string(),
        vec![NonTerminal("full-date".to_string())],
    );
    grammar.add_rule(
        "local-time".to_string(),
        vec![NonTerminal("partial-time".to_string())],
    );

    grammar.add_rule(
        "integer".to_string(),
        vec![NonTerminal("dec-int".to_string())],
    );
    grammar.add_rule(
        "integer".to_string(),
        vec![NonTerminal("hex-int".to_string())],
    );
    grammar.add_rule(
        "integer".to_string(),
        vec![NonTerminal("oct-int".to_string())],
    );
    grammar.add_rule(
        "integer".to_string(),
        vec![NonTerminal("bin-int".to_string())],
    );
    grammar.add_rule("minus".to_string(), vec![Terminal(Exact('-'))]);
    grammar.add_rule("plus".to_string(), vec![Terminal(Exact('+'))]);
    grammar.add_rule("underscore".to_string(), vec![Terminal(Exact('_'))]);
    grammar.add_rule("digit1-9".to_string(), vec![Terminal(Range('1', '9'))]);
    grammar.add_rule("digit0-7".to_string(), vec![Terminal(Range('0', '7'))]);
    grammar.add_rule("digit0-1".to_string(), vec![Terminal(Range('0', '1'))]);
    grammar.add_rule(
        "hex-prefix".to_string(),
        vec![Terminal(Exact('0')), Terminal(Exact('x'))],
    );
    grammar.add_rule(
        "oct-prefix".to_string(),
        vec![Terminal(Exact('0')), Terminal(Exact('o'))],
    );
    grammar.add_rule(
        "bin-prefix".to_string(),
        vec![Terminal(Exact('0')), Terminal(Exact('b'))],
    );
    grammar.add_rule(
        "dec-int".to_string(),
        vec![
            NonTerminal("sign".to_string()),
            NonTerminal("unsigned-dec-int".to_string()),
        ],
    );
    grammar.add_rule("sign".to_string(), vec![NonTerminal("minus".to_string())]);
    grammar.add_rule("sign".to_string(), vec![NonTerminal("plus".to_string())]);
    grammar.add_rule("sign".to_string(), vec![]);
    grammar.add_rule(
        "unsigned-dec-int".to_string(),
        vec![NonTerminal("DIGIT".to_string())],
    );
    grammar.add_rule(
        "unsigned-dec-int".to_string(),
        vec![
            NonTerminal("digit1-9".to_string()),
            NonTerminal("uns-dec-int-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "uns-dec-int-rest".to_string(),
        vec![
            NonTerminal("DIGIT_".to_string()),
            NonTerminal("uns-dec-int-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "uns-dec-int-rest".to_string(),
        vec![NonTerminal("DIGIT_".to_string())],
    );
    grammar.add_rule("DIGIT_".to_string(), vec![NonTerminal("DIGIT".to_string())]);
    grammar.add_rule(
        "DIGIT_".to_string(),
        vec![
            NonTerminal("underscore".to_string()),
            NonTerminal("DIGIT".to_string()),
        ],
    );
    grammar.add_rule(
        "hex-int".to_string(),
        vec![
            NonTerminal("hex-prefix".to_string()),
            NonTerminal("HEXDIG".to_string()),
            NonTerminal("hex-int-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "hex-int-rest".to_string(),
        vec![
            NonTerminal("HEXDIG_".to_string()),
            NonTerminal("hex-int-rest".to_string()),
        ],
    );
    grammar.add_rule("hex-int-rest".to_string(), vec![]);
    grammar.add_rule(
        "HEXDIG_".to_string(),
        vec![NonTerminal("HEXDIG".to_string())],
    );
    grammar.add_rule(
        "HEXDIG_".to_string(),
        vec![
            NonTerminal("underscore".to_string()),
            NonTerminal("HEXDIG".to_string()),
        ],
    );
    grammar.add_rule(
        "oct-int".to_string(),
        vec![
            NonTerminal("oct-prefix".to_string()),
            NonTerminal("digit0-7".to_string()),
            NonTerminal("oct-int-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "oct-int-rest".to_string(),
        vec![
            NonTerminal("digit0-7_".to_string()),
            NonTerminal("oct-int-rest".to_string()),
        ],
    );
    grammar.add_rule("oct-int-rest".to_string(), vec![]);
    grammar.add_rule(
        "digit0-7_".to_string(),
        vec![NonTerminal("digit0-7".to_string())],
    );
    grammar.add_rule(
        "digit0-7_".to_string(),
        vec![
            NonTerminal("underscore".to_string()),
            NonTerminal("digit0-7".to_string()),
        ],
    );
    grammar.add_rule(
        "bin-int".to_string(),
        vec![
            NonTerminal("bin-prefix".to_string()),
            NonTerminal("digit0-1".to_string()),
            NonTerminal("bin-int-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "bin-int-rest".to_string(),
        vec![
            NonTerminal("digit0-1_".to_string()),
            NonTerminal("bin-int-rest".to_string()),
        ],
    );
    grammar.add_rule("bin-int-rest".to_string(), vec![]);
    grammar.add_rule(
        "digit0-1_".to_string(),
        vec![NonTerminal("digit0-1".to_string())],
    );
    grammar.add_rule(
        "digit0-1_".to_string(),
        vec![
            NonTerminal("underscore".to_string()),
            NonTerminal("digit0-1".to_string()),
        ],
    );
    grammar.add_rule(
        "float".to_string(),
        vec![
            NonTerminal("float-int-part".to_string()),
            NonTerminal("float_rest".to_string()),
        ],
    );
    grammar.add_rule(
        "float".to_string(),
        vec![NonTerminal("special-float".to_string())],
    );
    grammar.add_rule(
        "float_rest".to_string(),
        vec![NonTerminal("exp".to_string())],
    );
    grammar.add_rule(
        "float_rest".to_string(),
        vec![
            NonTerminal("frac".to_string()),
            NonTerminal("[exp]".to_string()),
        ],
    );
    grammar.add_rule("[exp]".to_string(), vec![NonTerminal("exp".to_string())]);
    grammar.add_rule("[exp]".to_string(), vec![]);
    grammar.add_rule(
        "float-int-part".to_string(),
        vec![NonTerminal("dec-int".to_string())],
    );
    grammar.add_rule(
        "frac".to_string(),
        vec![
            NonTerminal("decimal-point".to_string()),
            NonTerminal("zero-prefixable-int".to_string()),
        ],
    );
    grammar.add_rule("decimal-point".to_string(), vec![Terminal(Exact('.'))]);
    grammar.add_rule(
        "zero-prefixable-int".to_string(),
        vec![
            NonTerminal("DIGIT".to_string()),
            NonTerminal("zero-prefixable-int-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "zero-prefixable-int-rest".to_string(),
        vec![
            NonTerminal("DIGIT_".to_string()),
            NonTerminal("zero-prefixable-int-rest".to_string()),
        ],
    );
    grammar.add_rule("zero-prefixable-int-rest".to_string(), vec![]);
    grammar.add_rule(
        "exp".to_string(),
        vec![
            Terminal(Exact('e')),
            NonTerminal("float-exp-part".to_string()),
        ],
    );
    grammar.add_rule(
        "float-exp-part".to_string(),
        vec![
            NonTerminal("sign".to_string()),
            NonTerminal("zero-prefixable-int".to_string()),
        ],
    );
    grammar.add_rule(
        "special-float".to_string(),
        vec![
            NonTerminal("sign".to_string()),
            NonTerminal("inf".to_string()),
        ],
    );
    grammar.add_rule(
        "special-float".to_string(),
        vec![
            NonTerminal("sign".to_string()),
            NonTerminal("nan".to_string()),
        ],
    );
    grammar.add_rule(
        "inf".to_string(),
        vec![
            Terminal(Exact('i')),
            Terminal(Exact('n')),
            Terminal(Exact('f')),
        ],
    );
    grammar.add_rule(
        "nan".to_string(),
        vec![
            Terminal(Exact('n')),
            Terminal(Exact('a')),
            Terminal(Exact('n')),
        ],
    );
    grammar.add_rule("boolean".to_string(), vec![NonTerminal("true".to_string())]);
    grammar.add_rule(
        "boolean".to_string(),
        vec![NonTerminal("false".to_string())],
    );
    grammar.add_rule(
        "true".to_string(),
        vec![
            Terminal(Exact('t')),
            Terminal(Exact('r')),
            Terminal(Exact('u')),
            Terminal(Exact('e')),
        ],
    );
    grammar.add_rule(
        "false".to_string(),
        vec![
            Terminal(Exact('f')),
            Terminal(Exact('a')),
            Terminal(Exact('l')),
            Terminal(Exact('s')),
            Terminal(Exact('e')),
        ],
    );

    grammar.add_rule(
        "string".to_string(),
        vec![NonTerminal("ml-basic-string".to_string())],
    );
    grammar.add_rule(
        "string".to_string(),
        vec![NonTerminal("basic-string".to_string())],
    );
    grammar.add_rule(
        "string".to_string(),
        vec![NonTerminal("ml-literal-string".to_string())],
    );
    grammar.add_rule(
        "string".to_string(),
        vec![NonTerminal("literal-string".to_string())],
    );
    grammar.add_rule(
        "basic-string".to_string(),
        vec![
            NonTerminal("quotation-mark".to_string()),
            NonTerminal("basic-chars".to_string()),
            NonTerminal("quotation-mark".to_string()),
        ],
    );
    grammar.add_rule(
        "basic-chars".to_string(),
        vec![
            NonTerminal("basic-char".to_string()),
            NonTerminal("basic-chars".to_string()),
        ],
    );
    grammar.add_rule("basic-chars".to_string(), vec![]);
    grammar.add_rule("quotation-mark".to_string(), vec![Terminal(Exact('"'))]);
    grammar.add_rule(
        "basic-char".to_string(),
        vec![NonTerminal("basic-unescaped".to_string())],
    );
    grammar.add_rule(
        "basic-char".to_string(),
        vec![NonTerminal("escaped".to_string())],
    );
    grammar.add_rule(
        "basic-unescaped".to_string(),
        vec![NonTerminal("wschar".to_string())],
    );
    grammar.add_rule("basic-unescaped".to_string(), vec![Terminal(Exact('!'))]);
    grammar.add_rule(
        "basic-unescaped".to_string(),
        vec![Terminal(Range('\x23', '\x5B'))],
    );
    grammar.add_rule(
        "basic-unescaped".to_string(),
        vec![Terminal(Range('\x5D', '\x7E'))],
    );
    grammar.add_rule(
        "basic-unescaped".to_string(),
        vec![NonTerminal("non-ascii".to_string())],
    );
    grammar.add_rule(
        "escaped".to_string(),
        vec![
            NonTerminal("escape".to_string()),
            NonTerminal("escape-seq-char".to_string()),
        ],
    );
    grammar.add_rule("escape".to_string(), vec![Terminal(Exact('\\'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x22'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x5C'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x62'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x66'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x6E'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x72'))]);
    grammar.add_rule("escape-seq-char".to_string(), vec![Terminal(Exact('\x74'))]);
    grammar.add_rule(
        "escape-seq-char".to_string(),
        vec![Terminal(Exact('\x75')), NonTerminal("4HEXDIG".to_string())],
    );

    grammar.add_rule(
        "escape-seq-char".to_string(),
        vec![Terminal(Exact('\x55')), NonTerminal("8HEXDIG".to_string())],
    );
    grammar.add_rule(
        "ml-basic-string".to_string(),
        vec![
            NonTerminal("ml-basic-string-delim".to_string()),
            NonTerminal("ml-basic-body".to_string()),
            NonTerminal("ml-basic-string-delim".to_string()),
        ],
    );
    grammar.add_rule(
        "ml-basic-string-delim".to_string(),
        vec![
            NonTerminal("quotation-mark".to_string()),
            NonTerminal("quotation-mark".to_string()),
            NonTerminal("quotation-mark".to_string()),
        ],
    );
    grammar.add_rule(
        "ml-basic-body".to_string(),
        vec![
            NonTerminal("*mlb-content".to_string()),
            NonTerminal("mlb-quotes-content".to_string()),
            NonTerminal("[mlb-quotes]".to_string()),
        ],
    );
    grammar.add_rule(
        "[mlb-quotes]".to_string(),
        vec![NonTerminal("mlb-quotes".to_string())],
    );
    grammar.add_rule("[mlb-quotes]".to_string(), vec![]);
    grammar.add_rule(
        "1*mlb-content".to_string(),
        vec![
            NonTerminal("mlb-content".to_string()),
            NonTerminal("1*mlb-content".to_string()),
        ],
    );
    grammar.add_rule(
        "1*mlb-content".to_string(),
        vec![NonTerminal("mlb-content".to_string())],
    );
    grammar.add_rule(
        "mlb-quotes-content".to_string(),
        vec![
            NonTerminal("mlb-quotes".to_string()),
            NonTerminal("1*mlb-content".to_string()),
            NonTerminal("mlb-quotes-content".to_string()),
        ],
    );
    grammar.add_rule("mlb-quotes-content".to_string(), vec![]);
    grammar.add_rule(
        "*mlb-content".to_string(),
        vec![
            NonTerminal("mlb-content".to_string()),
            NonTerminal("*mlb-content".to_string()),
        ],
    );
    grammar.add_rule("*mlb-content".to_string(), vec![]);
    grammar.add_rule(
        "mlb-content".to_string(),
        vec![NonTerminal("mlb-char".to_string())],
    );
    grammar.add_rule(
        "mlb-content".to_string(),
        vec![NonTerminal("newline".to_string())],
    );
    grammar.add_rule(
        "mlb-content".to_string(),
        vec![NonTerminal("mlb-escaped-nl".to_string())],
    );
    grammar.add_rule(
        "mlb-char".to_string(),
        vec![NonTerminal("mlb-unescaped".to_string())],
    );
    grammar.add_rule(
        "mlb-char".to_string(),
        vec![NonTerminal("escaped".to_string())],
    );
    grammar.add_rule(
        "mlb-quotes".to_string(),
        vec![NonTerminal("1*2quotation-mark".to_string())],
    );
    grammar.add_rule(
        "mlb-unescaped".to_string(),
        vec![NonTerminal("wschar".to_string())],
    );
    grammar.add_rule("mlb-unescaped".to_string(), vec![Terminal(Exact('!'))]);
    grammar.add_rule(
        "mlb-unescaped".to_string(),
        vec![Terminal(Range('\x23', '\x5B'))],
    );
    grammar.add_rule(
        "mlb-unescaped".to_string(),
        vec![Terminal(Range('\x5D', '\x7E'))],
    );
    grammar.add_rule(
        "mlb-unescaped".to_string(),
        vec![NonTerminal("non-ascii".to_string())],
    );
    grammar.add_rule(
        "mlb-escaped-nl".to_string(),
        vec![
            NonTerminal("escape".to_string()),
            NonTerminal("ws".to_string()),
            NonTerminal("newline".to_string()),
            NonTerminal("wschar-nls".to_string()),
        ],
    );
    grammar.add_rule(
        "wschar-nl".to_string(),
        vec![NonTerminal("wschar".to_string())],
    );
    grammar.add_rule(
        "wschar-nl".to_string(),
        vec![NonTerminal("newline".to_string())],
    );
    grammar.add_rule(
        "wschar-nls".to_string(),
        vec![
            NonTerminal("wschar-nl".to_string()),
            NonTerminal("wschar-nls".to_string()),
        ],
    );
    grammar.add_rule("wschar-nls".to_string(), vec![]);
    grammar.add_rule(
        "1*2quotation-mark".to_string(),
        vec![Terminal(Exact('"')), Terminal(Exact('"'))],
    );
    grammar.add_rule("1*2quotation-mark".to_string(), vec![Terminal(Exact('"'))]);
    grammar.add_rule(
        "literal-string".to_string(),
        vec![
            NonTerminal("apostrophe".to_string()),
            NonTerminal("*literal-char".to_string()),
            NonTerminal("apostrophe".to_string()),
        ],
    );
    grammar.add_rule(
        "*literal-char".to_string(),
        vec![
            NonTerminal("literal-char".to_string()),
            NonTerminal("*literal-char".to_string()),
        ],
    );
    grammar.add_rule("*literal-char".to_string(), vec![]);
    grammar.add_rule("apostrophe".to_string(), vec![Terminal(Exact('\''))]);
    grammar.add_rule("literal-char".to_string(), vec![Terminal(Exact('\x09'))]);
    grammar.add_rule(
        "literal-char".to_string(),
        vec![Terminal(Range('\x20', '\x26'))],
    );
    grammar.add_rule(
        "literal-char".to_string(),
        vec![Terminal(Range('\x28', '\x7E'))],
    );
    grammar.add_rule(
        "literal-char".to_string(),
        vec![NonTerminal("non-ascii".to_string())],
    );
    grammar.add_rule(
        "ml-literal-string".to_string(),
        vec![
            NonTerminal("ml-literal-string-delim".to_string()),
            NonTerminal("ml-literal-body".to_string()),
            NonTerminal("ml-literal-string-delim".to_string()),
        ],
    );
    grammar.add_rule(
        "ml-literal-string-delim".to_string(),
        vec![NonTerminal("3apostrophe".to_string())],
    );
    grammar.add_rule(
        "ml-literal-body".to_string(),
        vec![
            NonTerminal("*mll-content".to_string()),
            NonTerminal("some_mll-quotes-content".to_string()),
            NonTerminal("[mll-quotes]".to_string()),
        ],
    );
    grammar.add_rule(
        "3apostrophe".to_string(),
        vec![
            NonTerminal("apostrophe".to_string()),
            NonTerminal("apostrophe".to_string()),
            NonTerminal("apostrophe".to_string()),
        ],
    );
    grammar.add_rule(
        "*mll-content".to_string(),
        vec![
            NonTerminal("mll-content".to_string()),
            NonTerminal("*mll-content".to_string()),
        ],
    );
    grammar.add_rule("*mll-content".to_string(), vec![]);
    grammar.add_rule(
        "1*mll-content".to_string(),
        vec![
            NonTerminal("mll-content".to_string()),
            NonTerminal("1*mll-content".to_string()),
        ],
    );
    grammar.add_rule(
        "1*mll-content".to_string(),
        vec![NonTerminal("mll-content".to_string())],
    );
    grammar.add_rule(
        "[mll-quotes]".to_string(),
        vec![NonTerminal("mll-quotes".to_string())],
    );
    grammar.add_rule("[mll-quotes]".to_string(), vec![]);
    grammar.add_rule(
        "some_mll-quotes-content".to_string(),
        vec![
            NonTerminal("mll-quotes".to_string()),
            NonTerminal("1*mll-content".to_string()),
            NonTerminal("some_mll-quotes-content".to_string()),
        ],
    );
    grammar.add_rule("some_mll-quotes-content".to_string(), vec![]);
    grammar.add_rule(
        "mll-content".to_string(),
        vec![NonTerminal("mll-char".to_string())],
    );
    grammar.add_rule(
        "mll-content".to_string(),
        vec![NonTerminal("newline".to_string())],
    );
    grammar.add_rule("mll-char".to_string(), vec![Terminal(Exact('\x09'))]);
    grammar.add_rule(
        "mll-char".to_string(),
        vec![Terminal(Range('\x20', '\x26'))],
    );
    grammar.add_rule(
        "mll-char".to_string(),
        vec![Terminal(Range('\x28', '\x7E'))],
    );
    grammar.add_rule(
        "mll-char".to_string(),
        vec![NonTerminal("non-ascii".to_string())],
    );
    grammar.add_rule(
        "mll-quotes".to_string(),
        vec![NonTerminal("apostrophe".to_string())],
    );
    grammar.add_rule(
        "mll-quotes".to_string(),
        vec![
            NonTerminal("apostrophe".to_string()),
            NonTerminal("apostrophe".to_string()),
        ],
    );

    grammar.add_rule(
        "toml".to_string(),
        vec![NonTerminal("expression".to_string())],
    );
    grammar.add_rule(
        "toml".to_string(),
        vec![
            NonTerminal("expression".to_string()),
            NonTerminal("expressions".to_string()),
        ],
    );
    grammar.add_rule(
        "expressions".to_string(),
        vec![
            NonTerminal("newline".to_string()),
            NonTerminal("expression".to_string()),
            NonTerminal("expressions".to_string()),
        ],
    );
    grammar.add_rule(
        "expressions".to_string(),
        vec![NonTerminal("newline".to_string())],
    );
    grammar.add_rule(
        "expression".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            NonTerminal("maybe_comment".to_string()),
        ],
    );
    grammar.add_rule(
        "expression".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            NonTerminal("keyval".to_string()),
            NonTerminal("ws".to_string()),
            NonTerminal("maybe_comment".to_string()),
        ],
    );
    grammar.add_rule(
        "expression".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            NonTerminal("table".to_string()),
            NonTerminal("ws".to_string()),
            NonTerminal("maybe_comment".to_string()),
        ],
    );
    grammar.add_rule(
        "keyval".to_string(),
        vec![
            NonTerminal("key".to_string()),
            NonTerminal("keyval-sep".to_string()),
            NonTerminal("val".to_string()),
        ],
    );
    grammar.add_rule(
        "key".to_string(),
        vec![NonTerminal("simple-key".to_string())],
    );
    grammar.add_rule(
        "key".to_string(),
        vec![NonTerminal("dotted-key".to_string())],
    );
    grammar.add_rule(
        "simple-key".to_string(),
        vec![NonTerminal("quoted-key".to_string())],
    );
    grammar.add_rule(
        "simple-key".to_string(),
        vec![NonTerminal("unquoted-key".to_string())],
    );
    grammar.add_rule(
        "unquoted-key".to_string(),
        vec![
            NonTerminal("unquoted-key-char".to_string()),
            NonTerminal("unquoted-key".to_string()),
        ],
    );
    grammar.add_rule(
        "unquoted-key".to_string(),
        vec![NonTerminal("unquoted-key-char".to_string())],
    );
    grammar.add_rule(
        "unquoted-key-char".to_string(),
        vec![NonTerminal("ALPHA".to_string())],
    );
    grammar.add_rule(
        "unquoted-key-char".to_string(),
        vec![NonTerminal("DIGIT".to_string())],
    );
    grammar.add_rule(
        "unquoted-key-char".to_string(),
        vec![Terminal(Exact('\x2D'))],
    );
    grammar.add_rule(
        "unquoted-key-char".to_string(),
        vec![Terminal(Exact('\x5F'))],
    );
    grammar.add_rule(
        "quoted-key".to_string(),
        vec![NonTerminal("basic-string".to_string())],
    );
    grammar.add_rule(
        "quoted-key".to_string(),
        vec![NonTerminal("literal-string".to_string())],
    );
    grammar.add_rule(
        "dotted-key".to_string(),
        vec![
            NonTerminal("simple-key".to_string()),
            NonTerminal("dotted-key-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "dotted-key-rest".to_string(),
        vec![
            NonTerminal("dot-sep".to_string()),
            NonTerminal("simple-key".to_string()),
            NonTerminal("dotted-key-rest".to_string()),
        ],
    );
    grammar.add_rule(
        "dotted-key-rest".to_string(),
        vec![
            NonTerminal("dot-sep".to_string()),
            NonTerminal("simple-key".to_string()),
        ],
    );
    grammar.add_rule(
        "dot-sep".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            Terminal(Exact('.')),
            NonTerminal("ws".to_string()),
        ],
    );
    grammar.add_rule(
        "keyval-sep".to_string(),
        vec![
            NonTerminal("ws".to_string()),
            Terminal(Exact('=')),
            NonTerminal("ws".to_string()),
        ],
    );
    grammar.add_rule("val".to_string(), vec![NonTerminal("string".to_string())]);
    grammar.add_rule("val".to_string(), vec![NonTerminal("boolean".to_string())]);
    grammar.add_rule("val".to_string(), vec![NonTerminal("array".to_string())]);
    grammar.add_rule(
        "val".to_string(),
        vec![NonTerminal("inline-table".to_string())],
    );
    grammar.add_rule(
        "val".to_string(),
        vec![NonTerminal("date-time".to_string())],
    );
    grammar.add_rule("val".to_string(), vec![NonTerminal("float".to_string())]);
    grammar.add_rule("val".to_string(), vec![NonTerminal("integer".to_string())]);

    grammar
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn comment() {
        let mut grammar = grammar_nostart();

        grammar.set_start("comment".to_string());
        let grammar = grammar.compile();
        assert!(grammar.is_ok());

        let grammar = grammar.unwrap();
    }

}
