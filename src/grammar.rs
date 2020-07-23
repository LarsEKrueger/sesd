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

/// Match token classes during parsing.
///
/// Token classes (e.g. all digits 0-9) can be represented as rules with alternative terminal
/// symbols. This is very inefficient for large character classes (e.g. there are thousands of
/// printable Unicode characters). However, simple ranges are insufficient for some use cases (e.g.
/// printable Unicode characters span a dozen ranges with gaps). Thus, each token type needs to
/// provide a suitable matcher for maximum flexibility and efficiency.
///
/// T is the type of the tokens to match.
pub trait Matcher<T> {
    fn matches(&self, t: T) -> bool;
}

/// Grammar Symbols, terminals and non-terminals.
///
/// The terminal symbols hold matcher instances to match against the input tokens of type `T`. The
/// non-terminals hold their name.
#[derive(Debug)]
pub enum Symbol<M> {
    /// Terminals are of the same type as in the Buffer struct.
    Terminal(M),
    /// Non-terminals are identified by a string, which is later used for debugging and error messages.
    NonTerminal(String),
}

/// A grammar rule or production, e.g. S -> A B c.
#[derive(Debug)]
pub struct Rule<M> {
    /// Name of a non-terminal symbol.
    lhs: String,
    rhs: Vec<Symbol<M>>,
}

/// Grammar builder, textual representation of productions rules: S -> A B C
///
/// When a grammar has been completely defined, it needs to be compiled to be used by the parser.
/// This will create the look-up tables required for efficient parsing.
#[derive(Debug)]
pub struct Grammar<T, M>
where
    M: Matcher<T> + std::fmt::Debug,
    T: std::fmt::Debug,
{
    /// Rule table
    rules: Vec<Rule<M>>,

    /// Non-terminal that
    start: String,

    /// Marker to indicate the T is used indirectly by Matcher
    _marker: PhantomData<T>,
}

/// Symbol IDs are indices into the symbol table. As such, the can be fairly small integers to
/// save space. 16 bit should be sufficient for all purposes. If not, file a feature request.
pub type SymbolId = u16;

/// Number of symbol ids.
const MAX_SYMBOL_ID: SymbolId = std::u16::MAX;

/// ID of the pseudo-non-terminal to represent parsing errors
pub const ERROR_ID: SymbolId = 0;

/// Checked and compacted representation of a grammar.
///
/// Symbols (terminals and non-terminals) are identified by small integers. For debugging and
/// queries, the names of the non-terminals are kept in a table. The matchers of terminals are kept in a
/// separate table.
///
/// A compiled grammar identifies non-terminals by their index into the symbol table. This table is
/// used for debugging and error messages. The terminals cannot be queried from the public API,
/// thus all parameters of type `SymbolId` refer to non-terminal symbols.
///
/// TODO: Make finding rules of NonTerminal more efficient. Sort rules by lhs. Either keep separate table of
/// first fule index or store first rule index in rhs instead of symbol index.
pub struct CompiledGrammar<T, M>
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

/// Decoded symbol right of the dot in a dotted rule.
pub enum CompiledSymbol<M> {
    /// Dot was at the end of the rule. Return the LHS of the rule.
    Completed(SymbolId),
    /// Dot was on a nonterminal symbol.
    NonTerminal(SymbolId),
    /// Dot was on a terminal.
    Terminal(M),
}

/// Dotted rule from Earley Algorithm.
#[derive(PartialEq, Debug, Clone)]
pub struct DottedRule {
    /// Index into rule table
    pub rule: SymbolId,
    /// Index into rhs of rule
    dot: SymbolId,
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

impl<T, M> Grammar<T, M>
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
    pub fn add_rule(&mut self, lhs: String, rhs: Vec<Symbol<M>>) {
        self.rules.push(Rule { lhs, rhs });
    }

    /// Add a rule.
    pub fn add(&mut self, rule: Rule<M>) {
        self.rules.push(rule);
    }

    /// Set the start symbol. This can be overwritten and may contain an unknown symbol until just
    /// before [compile](method.compile) is called.
    pub fn set_start(&mut self, sym: String) {
        self.start = sym;
    }

    /// Compile the grammar for efficient use.
    ///
    /// If the given grammar is incorrect or inconsistent, return an error.
    pub fn compile(self) -> Result<CompiledGrammar<T, M>> {
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

            rules.push((lhs_id as SymbolId, rhs_id))
        }

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
            empty_rules: empty_rules as SymbolId,
            _marker: PhantomData,
        })
    }
}

impl<M> Rule<M> {
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
        self.rhs.push(Symbol::NonTerminal(nt.to_string()));
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
        self.rhs.push(Symbol::Terminal(t));
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
            self.rhs.push(Symbol::Terminal(t));
        }
        self
    }
}

impl<T, M> CompiledGrammar<T, M>
where
    M: Matcher<T> + Clone,
{
    /// Number of rules in the grammqr
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Check if rule with index `i` has the start symbol as lhs symbol.
    pub fn is_start_rule(&self, i: usize) -> bool {
        self.rules[i].0 == self.start
    }

    /// Check if the given symbol is the start symbol.
    pub fn is_start_symbol(&self, sym: SymbolId) -> bool {
        self.start == sym
    }

    /// Check if the rule with index `i` as the given symbol as lhs.
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

    /// Return symbol after the dot or indicate which lhs had been completed if dot is at the end
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

    /// Borrow the name of a non-terminal given its ID.
    ///
    /// Passing an invalid SymbolId results in a panic.
    pub fn nt_name<'a>(&'a self, sym: SymbolId) -> &'a str {
        &self.nonterminal_table[sym as usize]
    }

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

    /// Convert a list of non-terminal names to SymbolIds.
    ///
    /// Unknown names are returned as MAX_SYMBOL_ID.
    ///
    /// This function is slow and should not be used for mass queries.
    pub fn nt_ids(&self, names: &[&str]) -> Vec<SymbolId> {
        names.iter().map(|n| self.nt_id(n)).collect()
    }

    /// Get the lhs of rule with index `i`
    pub fn lhs(&self, i: usize) -> SymbolId {
        self.rules[i as usize].0
    }

    /// Check if the non-terminal symbol has empty rules
    pub fn nt_with_empty_rule(&self, sym: SymbolId) -> bool {
        sym < self.empty_rules
    }

}

impl<T, M> CompiledGrammar<T, M>
where
    M: Matcher<T> + Clone + std::fmt::Debug,
{
    /// Write a reabale form of a dotted rule to the given Writer instance.
    ///
    /// Debug function. Creates unicode characters that might not display correctly on old
    /// terminals.
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

    /// Convert a dotted rule to a string if possible.
    ///
    /// Debug function. Creates unicode characters that might not display correctly on old
    /// terminals.
    pub fn dotted_rule_to_string(&self, dotted_rule: &DottedRule) -> std::io::Result<String> {
        let mut line = Vec::new();
        self.write_dotted_rule(&mut line, dotted_rule)?;
        Ok(String::from_utf8_lossy(&line).into_owned())
    }

    /// Print a dotted rule to stdout if possible.
    ///
    /// Debug function. Creates unicode characters that might not display correctly on old
    /// terminals.
    pub fn print_dotted_rule(&self, dotted_rule: &DottedRule) -> std::io::Result<()> {
        self.write_dotted_rule(&mut std::io::stdout(), dotted_rule)
    }

    /// Log the tables as debug
    pub fn debug_tables(&self) {
        debug!("Non terminal table");
        for (i,n) in self.nonterminal_table.iter().enumerate() {
            debug!("  {:6}: {}", i, n);
        }
        for (i,n) in self.terminal_table.iter().enumerate() {
            debug!("  {:6}: {:?}", i+self.nonterminal_table.len(), n);
        }
    }
}

impl DottedRule {
    /// Create a dotted rule for the rule with index `rule_id` and the dot on the left of the first
    /// symbol on the rhs.
    pub fn new(rule_id: usize) -> Self {
        Self {
            rule: rule_id as SymbolId,
            dot: 0,
        }
    }

    /// Return a new dotted rule where the dot was moved one symbol to the right.
    pub fn advance_dot(&self) -> Self {
        Self {
            rule: self.rule,
            dot: self.dot + 1,
        }
    }

    /// Return true if the dot is on the left of the first symbol on the rhs.
    pub fn is_first(&self) -> bool {
        self.dot == 0
    }
}

impl<M> CompiledSymbol<M> {
    /// Return true if the symbol represents a completed rule.
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
