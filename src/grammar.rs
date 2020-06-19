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

//! Grammar builder for Parser

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Write;
use std::marker::PhantomData;

use itertools::Itertools;

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

/// Trait to match tokens
///
/// T is the type of the tokens to match
pub trait Matcher<T> {
    fn matches(&self, t: T) -> bool;
}

/// Grammar Symbols
///
/// The terminal symbols hold matcher instances to match against the input tokens of type `T`.
#[derive(Debug)]
pub enum Symbol<M> {
    /// Terminals are of the same type as in the Buffer struct.
    Terminal(M),
    /// Non-terminals are identified by a string, which is later used for debugging and error messages.
    NonTerminal(String),
}

/// A grammar is a set of productions rules: S -> A B C
///
/// When a grammar has been completely defined, it needs to be compiled to be used by the parser.
/// This will create the look-up tables required for efficient parsing.
///
/// Currently, all terminal symbols are enumerated. This assumes a lexer to be run before the
/// parser or a fairly small set of terminal symbols. This needs to be changed later.
///
/// TODO: Cope with character classes/sets natively.
#[derive(Debug)]
pub struct Grammar<T, M>
where
    M: Matcher<T> + std::fmt::Debug,
    T: std::fmt::Debug,
{
    /// Rule table, (lhs, rhs)
    rules: Vec<(String, Vec<Symbol<M>>)>,

    /// Non-terminal that
    start: String,

    /// Marker to indicate the T is used indirectly by Matcher
    _marker: PhantomData<T>,
}

/// Symbol IDs are indices into the symbol table. As such, the can be fairly small integers to
/// same space. 32 bit should be sufficient for all purposed.
pub type SymbolId = u32;

/// Number of symbol ids.
const MAX_SYMBOL_ID: u32 = std::u32::MAX;

/// A compiled grammar identifies non-terminals by their index into the symbol table. This table is
/// used for debugging and error messages.
///
/// TODO: Make finding rules of NonTerminal more efficient. Sort rules by lhs. Either keep separate table of
/// first fule index or store first rule index in rhs instead of symbol index.
pub struct CompiledGrammar<T, M>
where
    M: Matcher<T>,
{
    /// Names of symbols
    nonterminal_table: Vec<String>,

    /// Values of expected terminals
    terminal_table: Vec<M>,

    /// Rules as indices into the symbol tables. If the ID is < nonterminal_table.len(), it's a
    /// non-terminal. Otherwise it's a terminal.
    ///
    /// TODO: Flatten this.
    pub rules: Vec<(SymbolId, Vec<SymbolId>)>,

    /// Index of start symbol
    pub start: SymbolId,

    /// Marker to indicate the T is used indirectly by Matcher
    _marker: std::marker::PhantomData<T>,
}

/// Decode the rule indices into symbol index and terminal
pub enum CompiledSymbol<M> {
    /// Dot was at the end of the rule. Return the LHS
    Completed(SymbolId),
    /// Dot was on a nonterminal symbol
    NonTerminal(SymbolId),
    /// Dot was on a terminal
    Terminal(M),
}

/// Dotted Rule from Earley Algo
#[derive(PartialEq, Debug, Clone)]
pub struct DottedRule {
    /// Index into rule table
    rule: SymbolId,
    /// Index into rhs of rule
    dot: SymbolId,
}

impl<T> Matcher<T> for T
where
    T: PartialEq,
{
    fn matches(&self, t: T) -> bool {
        *self == t
    }
}

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

impl<T, M> Grammar<T, M>
where
    M: Matcher<T> + Hash + Ord + Clone + std::fmt::Debug,
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            start: String::new(),
            _marker: PhantomData,
        }
    }

    /// Add a rule with the name of the left hand side symbol and the expansion of the right hand
    /// side.
    pub fn add_rule(&mut self, lhs: String, rhs: Vec<Symbol<M>>) {
        self.rules.push((lhs, rhs));
    }

    /// Set the start symbol. This can be overwritten and may contain unknown an symbol until just
    /// before `compile` is called.
    pub fn set_start(&mut self, sym: String) {
        self.start = sym;
    }

    pub fn compile(self) -> Result<CompiledGrammar<T, M>> {
        // Build symbol table. Remember for each symbol if it has been seen on the lhs and assign a
        // symbol ID.
        let mut symbol_set = HashMap::new();
        if self.start.is_empty() {
            return Err(Error::EmptyStart);
        }
        let mut next_symbol_id = 0;
        // Don't forget the start symbol. It counts as used on the RHS
        update_symbol(
            &mut symbol_set,
            self.start.clone(),
            false,
            &mut next_symbol_id,
        );

        let mut terminal_set = HashSet::new();

        for r in self.rules.iter() {
            let lhs = &r.0;
            if lhs.is_empty() {
                return Err(Error::EmptySymbol);
            }
            update_symbol(&mut symbol_set, lhs.clone(), true, &mut next_symbol_id);
            // The index into the rhs can grow to the full length (i.e. past the last entry).
            if r.1.len() >= (MAX_SYMBOL_ID as usize) {
                return Err(Error::TooLarge(lhs.clone()));
            }
            // TODO: Reject if left recursive rule
            for s in r.1.iter() {
                match s {
                    Symbol::Terminal(t) => {
                        terminal_set.insert(t);
                    }
                    Symbol::NonTerminal(nt) => {
                        if nt.is_empty() {
                            return Err(Error::EmptySymbol);
                        }
                        update_symbol(&mut symbol_set, nt.clone(), false, &mut next_symbol_id);
                    }
                }
            }
        }

        // Check if all symbols have rules
        for (sym, (has_rule, _)) in symbol_set.iter() {
            if !has_rule {
                return Err(Error::NoRule(sym.clone()));
            }
        }

        // Build the non-terminal table by sorting the key-value pairs by id.
        let nonterminal_table: Vec<String> = symbol_set
            .iter()
            .map(|(k, (_, v))| (k, v))
            .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .map(|x| x.0.clone())
            .collect();
        if nonterminal_table.len() > (MAX_SYMBOL_ID as usize) {
            return Err(Error::TooLarge("Terminals".to_string()));
        }

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
        let rules: Vec<(SymbolId, Vec<SymbolId>)> = self
            .rules
            .iter()
            .map(|(lhs, rhs)| {
                let lhs_id = symbol_set.get(lhs).expect("lhs symbol should be known").1;

                let rhs_id: Vec<SymbolId> = rhs
                    .iter()
                    .map(|it| match it {
                        Symbol::Terminal(t) => {
                            let t_id = terminal_table
                                .binary_search(t)
                                .expect("rhs terminal should be known");
                            (t_id + nonterminal_table.len()) as SymbolId
                        }
                        Symbol::NonTerminal(nt) => {
                            let nt_id = symbol_set.get(nt).expect("rhs symbol should be known").1;
                            nt_id as SymbolId
                        }
                    })
                    .collect();

                (lhs_id as SymbolId, rhs_id)
            })
            .collect();

        // Get the start id
        let start = symbol_set
            .get(&self.start)
            .expect("start symbol should be known")
            .1;
        let start = start as SymbolId;

        Ok(CompiledGrammar {
            nonterminal_table,
            terminal_table,
            rules,
            start,
            _marker: PhantomData,
        })
    }
}

impl<T, M> CompiledGrammar<T, M>
where
    M: Matcher<T> + Clone,
{
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn is_start_rule(&self, i: usize) -> bool {
        self.rules[i].0 == self.start
    }

    pub fn is_start_symbol(&self, sym: SymbolId) -> bool {
        self.start == sym
    }

    pub fn lhs_is(&self, i: usize, sym: SymbolId) -> bool {
        self.rules[i].0 == sym
    }

    /// Return true if dotted rule indicates a completely parsed start symbol, i.e. a successful
    /// parse.
    pub fn dotted_is_completed_start(&self, dotted_rule: &DottedRule) -> bool {
        let rule_index = dotted_rule.rule as usize;
        let dot_index = dotted_rule.dot as usize;
        let rule = &self.rules[rule_index];
        dot_index >= rule.1.len() && self.is_start_rule(rule_index)
    }

    /// Return symbol after the dot or None if dot is at the end
    pub fn dotted_symbol(&self, dotted_rule: &DottedRule) -> CompiledSymbol<M> {
        let rule_index = dotted_rule.rule as usize;
        let dot_index = dotted_rule.dot as usize;
        let rule = &self.rules[rule_index];
        if dot_index < rule.1.len() {
            let sym = rule.1[dot_index];
            if (sym as usize) < self.nonterminal_table.len() {
                return CompiledSymbol::NonTerminal(sym);
            } else {
                let t_ind = (sym as usize) - self.nonterminal_table.len();
                return CompiledSymbol::Terminal(self.terminal_table[t_ind].clone());
            }
        }
        CompiledSymbol::Completed(rule.0)
    }

    pub fn nt_name<'a>(&'a self, sym: SymbolId) -> &'a str {
        &self.nonterminal_table[sym as usize]
    }
}

impl<T, M> CompiledGrammar<T, M>
where
    M: Matcher<T> + Clone + std::fmt::Debug,
{
    pub fn write_dotted_rule(
        &self,
        writer: &mut dyn Write,
        dotted_rule: &DottedRule,
    ) -> std::io::Result<()> {
        let rule_index = dotted_rule.rule as usize;
        let dot_index = dotted_rule.dot as usize;
        let rule = &self.rules[rule_index];
        write!(writer, "{} → ", self.nonterminal_table[rule.0 as usize])?;
        for i in 0..rule.1.len() {
            if i == dot_index {
                write!(writer, "• ")?;
            }
            let sym = rule.1[i];
            if (sym as usize) < self.nonterminal_table.len() {
                write!(writer, "{} ", self.nonterminal_table[sym as usize])?;
            } else {
                let t_ind = (sym as usize) - self.nonterminal_table.len();
                write!(writer, "'{:?}' ", self.terminal_table[t_ind])?;
            }
        }
        if dot_index == rule.1.len() {
            write!(writer, "• ")?;
        }
        Ok(())
    }

    pub fn dotted_rule_to_string(&self, dotted_rule: &DottedRule) -> std::io::Result<String> {
        let mut line = Vec::new();
        self.write_dotted_rule(&mut line, dotted_rule)?;
        Ok(String::from_utf8_lossy(&line).into_owned())
    }

    pub fn print_dotted_rule(&self, dotted_rule: &DottedRule) -> std::io::Result<()> {
        self.write_dotted_rule(&mut std::io::stdout(), dotted_rule)
    }
}

impl DottedRule {
    pub fn new(rule_id: usize) -> Self {
        Self {
            rule: rule_id as SymbolId,
            dot: 0,
        }
    }

    pub fn advance_dot(&self) -> Self {
        Self {
            rule: self.rule,
            dot: self.dot + 1,
        }
    }

    pub fn is_first(&self) -> bool {
        self.dot == 0
    }
}

impl<M> CompiledSymbol<M> {
    pub fn is_complete(&self) -> bool {
        match *self {
            Self::Completed(_) => true,
            _ => false,
        }
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
    pub fn define_grammar() -> Grammar<char, CharMatcher> {
        let mut grammar: Grammar<char, CharMatcher> = Grammar::new();

        use CharMatcher::*;
        use Symbol::*;
        grammar.set_start("S".to_string());
        grammar.add_rule(
            "S".to_string(),
            vec![NonTerminal("NP".to_string()), NonTerminal("VP".to_string())],
        );
        grammar.add_rule(
            "NP".to_string(),
            vec![NonTerminal("NP".to_string()), NonTerminal("PP".to_string())],
        );
        grammar.add_rule("NP".to_string(), vec![NonTerminal("Noun".to_string())]);
        grammar.add_rule(
            "VP".to_string(),
            vec![
                NonTerminal("Verb".to_string()),
                NonTerminal("NP".to_string()),
            ],
        );
        grammar.add_rule(
            "VP".to_string(),
            vec![NonTerminal("VP".to_string()), NonTerminal("PP".to_string())],
        );
        grammar.add_rule(
            "PP".to_string(),
            vec![
                NonTerminal("Prep".to_string()),
                NonTerminal("NP".to_string()),
            ],
        );
        grammar.add_rule(
            "Noun".to_string(),
            vec![
                Terminal(Exact('j')),
                Terminal(Exact('o')),
                Terminal(Exact('h')),
                Terminal(Exact('n')),
                Terminal(Exact(' ')),
            ],
        );
        grammar.add_rule(
            "Noun".to_string(),
            vec![
                Terminal(Exact('m')),
                Terminal(Exact('a')),
                Terminal(Exact('r')),
                Terminal(Exact('y')),
                Terminal(Exact(' ')),
            ],
        );
        grammar.add_rule(
            "Noun".to_string(),
            vec![
                Terminal(Exact('d')),
                Terminal(Exact('e')),
                Terminal(Exact('n')),
                Terminal(Exact('v')),
                Terminal(Exact('e')),
                Terminal(Exact('r')),
                Terminal(Exact(' ')),
            ],
        );
        grammar.add_rule(
            "Verb".to_string(),
            vec![
                Terminal(Exact('c')),
                Terminal(Exact('a')),
                Terminal(Exact('l')),
                Terminal(Exact('l')),
                Terminal(Exact('e')),
                Terminal(Exact('d')),
                Terminal(Exact(' ')),
            ],
        );
        grammar.add_rule(
            "Prep".to_string(),
            vec![
                Terminal(Exact('f')),
                Terminal(Exact('r')),
                Terminal(Exact('o')),
                Terminal(Exact('m')),
                Terminal(Exact(' ')),
            ],
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
