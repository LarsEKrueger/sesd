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

use itertools::Itertools;

/// List of errors when processing grammars
pub enum Error {
    /// Too many entries to compile
    TooLarge,
    /// Non-terminal used in a rule without a rule for it
    NoRule,
    /// No start symbol set
    EmptyStart,
    /// Empty string used in a rule
    EmptySymbol,
    /// Empty right hand side of a rule
    EmptyRhs,
}

/// Type alias for Results with Errors
type Result<T> = std::result::Result<T, Error>;

/// Grammar Symbols
pub enum Symbol<T> {
    /// Terminals are of the same type as in the Buffer struct.
    Terminal(T),
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
pub struct Grammar<T> {
    /// Rule table, (lhs, rhs)
    rules: Vec<(String, Vec<Symbol<T>>)>,

    /// Non-terminal that
    start: String,
}

/// Symbol IDs are indices into the symbol table. As such, the can be fairly small integers to
/// same space. 32 bit should be sufficient for all purposed.
type SymbolId = u32;


/// Number of symbol ids.
const MAX_SYMBOL_ID : u32 = std::u32::MAX;

/// A compiled grammar identifies non-terminals by their index into the symbol table. This table is
/// used for debugging and error messages.
pub struct CompiledGrammar<T> {
    /// Names of symbols
    nonterminal_table: Vec<String>,

    /// Values of expected terminals
    terminal_table: Vec<T>,

    /// Rules as indices into the symbol table. If the ID is < nonterminal_table.len(), it's a
    /// non-terminal. Otherwise it's a terminal.
    ///
    /// TODO: Flatten this.
    rules: Vec<(SymbolId, Vec<SymbolId>)>,

    /// Index of start symbol
    start: SymbolId,
}

fn update_symbol(map: &mut HashMap<String, (bool, usize)>, key: String, is_rule: bool, next_id:&mut usize) {
    if let Some((has_rule,_)) = map.get_mut(&key) {
        *has_rule = *has_rule | is_rule;
    } else {
        map.insert(key, (is_rule, *next_id));
        *next_id += 1;
    }
}

impl<T> Grammar<T>
where
    T: Eq + Hash + Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            start: String::new(),
        }
    }

    /// Add a rule with the name of the left hand side symbol and the expansion of the right hand
    /// side.
    pub fn add_rule(&mut self, lhs: String, rhs: Vec<Symbol<T>>) {
        self.rules.push((lhs, rhs));
    }

    /// Set the start symbol. This can be overwritten and may contain unknown an symbol until just
    /// before `compile` is called.
    pub fn set_start(&mut self, sym: String) {
        self.start = sym;
    }

    pub fn compile(self) -> Result<CompiledGrammar<T>> {
        // Build symbol table. Remember for each symbol if it has been seen on the lhs and assign a
        // symbol ID.
        let mut symbol_set = HashMap::new();
        if self.start.is_empty() {
            return Err(Error::EmptyStart);
        }
        let mut next_symbol_id = 0;
        // Don't forget the start symbol. It counts as used on the RHS
        update_symbol(&mut symbol_set, self.start.clone(), false, &mut next_symbol_id);

        let mut terminal_set = HashSet::new();

        for r in self.rules.iter() {
            let lhs = &r.0;
            if lhs.is_empty() {
                return Err(Error::EmptySymbol);
            }
            update_symbol(&mut symbol_set, lhs.clone(), true, &mut next_symbol_id);
            if r.1.is_empty() {
                return Err(Error::EmptyRhs);
            }
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
        for (has_rule,_) in symbol_set.values() {
            if !has_rule {
                return Err(Error::NoRule);
            }
        }

        // Build the non-terminal table by sorting the key-value pairs by id.
        let nonterminal_table : Vec<String>= symbol_set.iter().map( |(k,(_,v))| (k,v)).sorted_by( |a,b| Ord::cmp( a.1, b.1)).map( |x| x.0.clone()).collect();
        if nonterminal_table.len() > (MAX_SYMBOL_ID as usize) { return Err(Error::TooLarge); }

        // Build the terminal table
        let terminal_table : Vec<T> = terminal_set.iter().sorted_by( |a,b| Ord::cmp( a, b)).map( |x| (*x).clone()).collect();
        if terminal_table.len() + nonterminal_table.len() > (MAX_SYMBOL_ID as usize) { return Err(Error::TooLarge); }

        // Build the rules
        let rules : Vec<(SymbolId,Vec<SymbolId>)> = self.rules.iter().map( |(lhs,rhs)| {
            let lhs_id = symbol_set.get(lhs).expect( "lhs symbol should be known").1;

            let rhs_id : Vec<SymbolId>= rhs.iter().map( |it| match it {
                Symbol::Terminal(t) => {
                    let t_id = terminal_table.binary_search( t).expect( "rhs terminal should be known");
                    (t_id + nonterminal_table.len()) as SymbolId
                }
                Symbol::NonTerminal(nt) => {
                    let nt_id = symbol_set.get(nt).expect("rhs symbol should be known").1;
                    nt_id as SymbolId
                }
            }).collect();

            (lhs_id as SymbolId, rhs_id)
        }).collect();

        // Get the start id
        let start = symbol_set.get(&self.start).expect("start symbol should be known").1;
        let start = start  as SymbolId;

        Ok( CompiledGrammar{ 
            nonterminal_table,
            terminal_table,
            rules,
            start})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Define the following grammar
    ///
    /// S
    /// S -> 
    fn define_grammar() -> Grammar<char> {
        let mut grammar : Grammar<char> = Grammar::new();
grammar
    }

    #[test]
    fn grammar() {
        let grammar = define_grammar();

        // Test that the grammar has been defined correctly

    }

    #[test]
    fn compile_grammar() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile();

        // Test that the grammar has been compiled correctly
    }
}
