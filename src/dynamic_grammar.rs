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

//! Build a grammar at runtime

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;

use itertools::Itertools;

use crate::grammar::{CompiledGrammar, Matcher, SymbolId, ERROR_ID};

/// Number of symbol ids.
const MAX_SYMBOL_ID: SymbolId = std::u16::MAX;

/// List of errors when processing grammars
#[derive(Debug)]
pub enum Error {
    /// Too many entries to compile
    TooLarge(String),
    /// Non-terminal used in a rule without a rule for it
    NoRule(String),
    /// No start symbol set
    EmptyStart,
    /// Empty string used in a rule
    EmptySymbol,
    /// Empty right hand side of a rule
    EmptyRhs,
}

/// Type alias for Results with Errors
type Result<T> = std::result::Result<T, Error>;

/// Grammar symbols, terminals and non-terminals, in textual representation.
///
/// The terminal symbols hold matcher instances to match against the input tokens of type `T`. The
/// non-terminals hold their name.
#[derive(Debug)]
pub enum TextSymbol<M> {
    /// Terminals are of the same type as in the Buffer struct.
    Terminal(M),
    /// Non-terminals are identified by a string, which is later used for debugging and error messages.
    NonTerminal(String),
}

/// A grammar rule or production, e.g. S -> A B c, in textual representation.
#[derive(Debug)]
pub struct TextRule<M> {
    /// Name of a non-terminal symbol.
    lhs: String,
    rhs: Vec<TextSymbol<M>>,
}

/// Grammar builder, textual representation of productions rules: S -> A B C
///
/// When a grammar has been completely defined, it needs to be compiled to be used by the parser.
/// This will create the look-up tables required for efficient parsing.
#[derive(Debug)]
pub struct TextGrammar<T, M>
where
    M: Matcher<T> + std::fmt::Debug,
    T: std::fmt::Debug,
{
    /// Rule table
    rules: Vec<TextRule<M>>,

    /// Non-terminal that
    start: String,

    /// Marker to indicate the T is used indirectly by Matcher
    _marker: PhantomData<T>,
}

/// Machine readable representation of a grammar, dynamically built from e.g. a TextGrammar.
pub struct DynamicGrammar<T, M>
where
    M: Matcher<T>,
{
    /// Names of symbols. Index corresponds to value in rhs of rules. Rules that have empty right
    /// hand sides come first.
    nonterminal_table: Vec<String>,

    /// Values of expected terminals. Index is value from rhs of rule - nonterminal_table.len().
    terminal_table: Vec<M>,

    /// Rules as indices into the symbol tables. If the ID is < nonterminal_table.len(), it's a
    /// non-terminal. Otherwise it's a terminal.
    ///
    /// TODO: Flatten this.
    rules: Vec<(SymbolId, Vec<SymbolId>)>,

    /// Index of start symbol
    start: SymbolId,

    /// Number of symbols that have empty right hand sides.
    empty_rules: SymbolId,

    /// Marker to indicate the T is used indirectly by Matcher
    _marker: std::marker::PhantomData<T>,
}

impl<T> Matcher<T> for T
where
    T: PartialEq,
{
    /// If non-terminal matchers are tokens, accept them only if they are identical.
    ///
    /// Default implementation for simple grammar without token classes.
    fn matches(&self, t: T) -> bool {
        *self == t
    }
}

/// Update the symbol table during grammar compilation.
fn update_symbol(
    map: &mut HashMap<String, (bool, usize)>,
    key: String,
    is_rule: bool,
    next_id: &mut usize,
) {
    if let Some((has_rule, _)) = map.get_mut(&key) {
        *has_rule = *has_rule | is_rule;
    } else {
        map.insert(key, (is_rule, *next_id));
        *next_id += 1;
    }
}

impl<T, M> TextGrammar<T, M>
where
    M: Matcher<T> + Hash + Ord + Clone + std::fmt::Debug,
    T: std::fmt::Debug,
{
    /// Return a new grammar builder.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            start: String::new(),
            _marker: PhantomData,
        }
    }

    /// Add a rule with the name of the left hand side symbol and the expansion of the right hand
    /// side.
    ///
    /// Obsolete interface. Use [add](#method.add).
    pub fn add_rule(&mut self, lhs: String, rhs: Vec<TextSymbol<M>>) {
        self.rules.push(TextRule { lhs, rhs });
    }

    /// Add a rule.
    pub fn add(&mut self, rule: TextRule<M>) {
        self.rules.push(rule);
    }

    /// Set the start symbol. This can be overwritten and may contain an unknown symbol until just
    /// before [compile](#method.compile) is called.
    pub fn set_start(&mut self, sym: String) {
        self.start = sym;
    }

    /// Compile the grammar for efficient use.
    ///
    /// If the given grammar is incorrect or inconsistent, return an error.
    pub fn compile(self) -> Result<DynamicGrammar<T, M>> {
        // Build symbol table. Remember for each symbol if it has been seen on the lhs and assign a
        // symbol ID.
        let mut symbol_set = HashMap::new();
        // Add a dummy non-terminal symbol for errors. An empty string cannot be added otherwise.
        symbol_set.insert(String::new(), (true, ERROR_ID as usize));
        let mut next_symbol_id = (ERROR_ID as usize) + 1;

        let mut terminal_set = HashSet::new();

        // Find empty rules first to give their lhs symbols low numbers
        for r in self.rules.iter() {
            let lhs = &r.lhs;
            if lhs.is_empty() {
                return Err(Error::EmptySymbol);
            }
            // The index into the rhs can grow to the full length (i.e. past the last entry).
            if r.rhs.len() >= (MAX_SYMBOL_ID as usize) {
                return Err(Error::TooLarge(lhs.clone()));
            }

            if r.rhs.len() == 0 {
                update_symbol(&mut symbol_set, lhs.clone(), true, &mut next_symbol_id);
            }
        }

        let empty_rules = next_symbol_id;
        if empty_rules > (MAX_SYMBOL_ID as usize) {
            return Err(Error::TooLarge("Empty Rules".to_string()));
        }

        for r in self.rules.iter() {
            let lhs = &r.lhs;
            update_symbol(&mut symbol_set, lhs.clone(), true, &mut next_symbol_id);
            // TODO?: Reject if left recursive rule
            for s in r.rhs.iter() {
                match s {
                    TextSymbol::Terminal(t) => {
                        terminal_set.insert(t);
                    }
                    TextSymbol::NonTerminal(nt) => {
                        if nt.is_empty() {
                            return Err(Error::EmptySymbol);
                        }
                        update_symbol(&mut symbol_set, nt.clone(), false, &mut next_symbol_id);
                    }
                }
            }
        }

        // Don't forget the start symbol. It counts as used on the RHS
        if self.start.is_empty() {
            return Err(Error::EmptyStart);
        }

        update_symbol(
            &mut symbol_set,
            self.start.clone(),
            false,
            &mut next_symbol_id,
        );

        // Check if all symbols have rules
        for (sym, (has_rule, _)) in symbol_set.iter() {
            if !has_rule {
                return Err(Error::NoRule(sym.clone()));
            }
        }

        // Build the non-terminal table by sorting the key-value pairs by id.
        let mut nonterminal_table: Vec<String> = symbol_set
            .iter()
            .map(|(k, (_, v))| (k, v))
            .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .map(|x| x.0.clone())
            .collect();
        if nonterminal_table.len() > (MAX_SYMBOL_ID as usize) {
            return Err(Error::TooLarge("Terminals".to_string()));
        }
        // Overwrite the error pseudo-non-terminal with a descriptive name
        nonterminal_table[0] = "~~~ERROR~~~".to_string();

        // Build the terminal table
        let terminal_table: Vec<M> = terminal_set
            .iter()
            .sorted_by(|a, b| Ord::cmp(a, b))
            .map(|x| (*x).clone())
            .collect();
        if terminal_table.len() + nonterminal_table.len() > (MAX_SYMBOL_ID as usize) {
            return Err(Error::TooLarge(
                "Terminals and NonTerminals together".to_string(),
            ));
        }

        // Build the rules
        let mut rules: Vec<(SymbolId, Vec<SymbolId>)> = Vec::new();

        // The first rule (id = 0) is a pseudo-rule for error handling.
        rules.push((ERROR_ID, Vec::new()));
        for rule in self.rules.iter() {
            let lhs_id = symbol_set
                .get(&rule.lhs)
                .expect("lhs symbol should be known")
                .1;

            let rhs_id: Vec<SymbolId> = rule
                .rhs
                .iter()
                .map(|it| match it {
                    TextSymbol::Terminal(t) => {
                        let t_id = terminal_table
                            .binary_search(t)
                            .expect("rhs terminal should be known");
                        (t_id + nonterminal_table.len()) as SymbolId
                    }
                    TextSymbol::NonTerminal(nt) => {
                        let nt_id = symbol_set.get(nt).expect("rhs symbol should be known").1;
                        nt_id as SymbolId
                    }
                })
                .collect();

            rules.push((lhs_id as SymbolId, rhs_id))
        }

        // Get the start id
        let start = symbol_set
            .get(&self.start)
            .expect("start symbol should be known")
            .1;
        let start = start as SymbolId;

        Ok(DynamicGrammar {
            nonterminal_table,
            terminal_table,
            rules,
            start,
            empty_rules: empty_rules as SymbolId,
            _marker: PhantomData,
        })
    }
}

impl<M> TextRule<M> {
    /// Create a new rule for the given symbol.
    ///
    /// ```ignore
    /// Rule::new("left")
    /// ```
    ///
    /// corresponds to
    /// ```ignore
    /// <left> ::=
    /// ```
    ///
    /// in [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).
    pub fn new(lhs: &str) -> Self {
        Self {
            lhs: lhs.to_string(),
            rhs: Vec::new(),
        }
    }

    /// Append a non-terminal to a rule.
    ///
    /// ```ignore
    /// Rule::new("left").nt("first")
    /// ```
    ///
    /// corresponds to
    /// ```ignore
    /// <left> ::= <first>
    /// ```
    ///
    /// in [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).
    pub fn nt(mut self, nt: &str) -> Self {
        self.rhs.push(TextSymbol::NonTerminal(nt.to_string()));
        self
    }

    /// Append a matcher for terminal to a rule.
    ///
    /// ```ignore
    /// Rule::new("left").nt("first").t('x')
    /// ```
    ///
    /// corresponds to
    /// ```ignore
    /// <left> ::= <first> "x"
    /// ```
    ///
    /// in [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).
    pub fn t(mut self, t: M) -> Self {
        self.rhs.push(TextSymbol::Terminal(t));
        self
    }

    /// Append a sequence of terminals to the rule.
    ///
    /// ```ignore
    /// Rule::new("left").ts("abc".chars())
    /// ```
    ///
    /// corresponds to
    ///
    /// ```ignore
    /// <left> ::= 'a' 'b' 'c'
    /// ```
    ///
    /// in [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).
    pub fn ts(mut self, iter: impl Iterator<Item = M>) -> Self {
        for t in iter {
            self.rhs.push(TextSymbol::Terminal(t));
        }
        self
    }
}

impl<T, M> CompiledGrammar<T, M> for DynamicGrammar<T, M>
where
    M: Matcher<T> + Clone,
{
    fn start_symbol(&self) -> SymbolId {
        self.start
    }

    fn rules_count(&self) -> usize {
        self.rules.len()
    }

    fn lhs(&self, rule: usize) -> SymbolId {
        self.rules[rule].0
    }

    fn rhs(&self, rule: usize) -> &[SymbolId] {
        &self.rules[rule].1
    }

    fn nt_name(&self, nt: SymbolId) -> &str {
        &self.nonterminal_table[nt as usize]
    }

    fn nt_count(&self) -> SymbolId {
        self.nonterminal_table.len() as SymbolId
    }

    fn t_count(&self) -> SymbolId {
        self.terminal_table.len() as SymbolId
    }

    fn nt_empty_count(&self) -> SymbolId {
        self.empty_rules
    }

    fn matcher(&self, term: SymbolId) -> M {
        self.terminal_table[term as usize].clone()
    }
}

impl<T, M> DynamicGrammar<T, M>
where
    M: Matcher<T>,
{
    /// Convert the name of non-terminal to its SymbolId.
    ///
    /// Unknown names are returned as MAX_SYMBOL_ID.
    ///
    /// This function is slow and should not be used for mass queries.
    pub fn nt_id(&self, name: &str) -> SymbolId {
        for i in 0..self.nonterminal_table.len() {
            if name == self.nonterminal_table[i] {
                return i as SymbolId;
            }
        }
        MAX_SYMBOL_ID
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::char::CharMatcher;
    use super::*;

    /// Define the grammar from: https://www.cs.unm.edu/~luger/ai-final2/CH9_Dynamic%20Programming%20and%20the%20Earley%20Parser.pdf
    ///
    /// S
    /// S → NP VP
    /// NP → NP PP
    /// NP → Noun
    /// VP → Verb NP
    /// VP → VP PP
    /// PP → Prep NP
    /// Noun → “john”
    /// Noun → “mary”
    /// Noun → “denver”
    /// Verb → “called”
    /// Prep → “from”
    pub fn define_grammar() -> TextGrammar<char, CharMatcher> {
        let mut grammar: TextGrammar<char, CharMatcher> = TextGrammar::new();

        use CharMatcher::*;
        grammar.set_start("S".to_string());
        grammar.add(TextRule::new("S").nt("NP").nt("VP"));
        grammar.add(TextRule::new("NP").nt("NP").nt("PP"));
        grammar.add(TextRule::new("NP").nt("Noun"));
        grammar.add(TextRule::new("VP").nt("Verb").nt("NP"));
        grammar.add(TextRule::new("VP").nt("VP").nt("PP"));
        grammar.add(TextRule::new("PP").nt("Prep").nt("NP"));
        grammar.add(
            TextRule::new("Noun")
                .t(Exact('j'))
                .t(Exact('o'))
                .t(Exact('h'))
                .t(Exact('n'))
                .t(Exact(' ')),
        );
        grammar.add(
            TextRule::new("Noun")
                .t(Exact('m'))
                .t(Exact('a'))
                .t(Exact('r'))
                .t(Exact('y'))
                .t(Exact(' ')),
        );
        grammar.add(
            TextRule::new("Noun")
                .t(Exact('d'))
                .t(Exact('e'))
                .t(Exact('n'))
                .t(Exact('v'))
                .t(Exact('e'))
                .t(Exact('r'))
                .t(Exact(' ')),
        );
        grammar.add(
            TextRule::new("Verb")
                .t(Exact('c'))
                .t(Exact('a'))
                .t(Exact('l'))
                .t(Exact('l'))
                .t(Exact('e'))
                .t(Exact('d'))
                .t(Exact(' ')),
        );
        grammar.add(
            TextRule::new("Prep")
                .t(Exact('f'))
                .t(Exact('r'))
                .t(Exact('o'))
                .t(Exact('m'))
                .t(Exact(' ')),
        );

        grammar
    }

    #[test]
    fn compile_grammar() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        // Test that the grammar has been compiled correctly

        // Get start symbol
        let start = compiled_grammar.start as usize;
        assert!(start < compiled_grammar.nonterminal_table.len());
        assert_eq!(compiled_grammar.nonterminal_table[start], "S");

        let terminal_base = compiled_grammar.nonterminal_table.len();

        use CharMatcher::*;

        let mut pp_found = false;
        let mut mary_found = false;
        for rule in compiled_grammar.rules {
            let lhs = rule.0 as usize;
            assert!(lhs < compiled_grammar.nonterminal_table.len());
            // Find the rule with "PP" as lhs
            if compiled_grammar.nonterminal_table[lhs] == "PP" {
                assert_eq!(pp_found, false);
                pp_found = true;
                assert_eq!(rule.1.len(), 2);
                let prep = rule.1[0] as usize;
                let np = rule.1[1] as usize;
                assert!(prep < compiled_grammar.nonterminal_table.len());
                assert!(np < compiled_grammar.nonterminal_table.len());
                assert_eq!(compiled_grammar.nonterminal_table[prep], "Prep");
                assert_eq!(compiled_grammar.nonterminal_table[np], "NP");
            } else if compiled_grammar.nonterminal_table[lhs] == "Noun" {
                // Find mary
                if rule.1.len() == 5 {
                    let m = rule.1[0] as usize;
                    assert!(m >= terminal_base);
                    let m = m - terminal_base;
                    assert!(m < compiled_grammar.terminal_table.len());
                    if compiled_grammar.terminal_table[m] == Exact('m') {
                        let a = rule.1[1] as usize;
                        assert!(a >= terminal_base);
                        let a = a - terminal_base;
                        assert!(a < compiled_grammar.terminal_table.len());
                        assert_eq!(compiled_grammar.terminal_table[a], Exact('a'));

                        let r = rule.1[2] as usize;
                        assert!(r >= terminal_base);
                        let r = r - terminal_base;
                        assert!(r < compiled_grammar.terminal_table.len());
                        assert_eq!(compiled_grammar.terminal_table[r], Exact('r'));

                        let y = rule.1[3] as usize;
                        assert!(y >= terminal_base);
                        let y = y - terminal_base;
                        assert!(y < compiled_grammar.terminal_table.len());
                        assert_eq!(compiled_grammar.terminal_table[y], Exact('y'));

                        let sp = rule.1[4] as usize;
                        assert!(sp >= terminal_base);
                        let sp = sp - terminal_base;
                        assert!(sp < compiled_grammar.terminal_table.len());
                        assert_eq!(compiled_grammar.terminal_table[sp], Exact(' '));

                        assert_eq!(mary_found, false);
                        mary_found = true;
                    }
                }
            }
        }
        assert!(pp_found);
        assert!(mary_found);
    }
}
