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

//! Parser to work on Buffer

use super::grammar::{CompiledGrammar, CompiledSymbol, SymbolId};

/// Parser error codes
#[derive(Debug)]
pub enum Error {
    /// Invalid index was passed to update
    InvalidIndex,
}

/// Type alias for Results with Errors
type Result<T> = std::result::Result<T, Error>;

/// Dotted Rule from Earley Algo
struct DottedRule {
    /// Index into rule table
    rule: SymbolId,
    /// Index into rhs of rule
    dot: SymbolId,
}

/// Start and dot position in the input buffer
///
/// Both indices are usize as to not limit the length of the input buffer.
///
/// TODO: Limit the size of the input buffer.
struct StartDot {
    start: usize,
    dot: usize,
}

/// Incrementally parse the input buffer.
pub struct Parser<T> {
    grammar: CompiledGrammar<T>,

    /// Parsing chart.
    ///
    /// Outer dimension index corresponds to buffer index. Inner dimensions are the possible rules that
    /// apply at this buffer index. In particular, chart[i] contain the rules that apply after
    /// buffer[i] has been processed.
    chart: Vec<Vec<(DottedRule, StartDot)>>,

    /// Number of buffer entries (from the beginning) where the parse is valid.
    ///
    /// This value will be decreased when the buffer is changed and increased when the parser is
    /// updated.
    valid_entries: usize,

    /// Set of rules that derive from the start symbol.
    start_set: Vec<(DottedRule, StartDot)>,
}

impl DottedRule {
    fn new(rule_id: usize) -> Self {
        Self {
            rule: rule_id as SymbolId,
            dot: 0,
        }
    }
}

impl StartDot {
    fn new(index: usize) -> Self {
        Self {
            start: index,
            dot: index,
        }
    }
}

impl<T> Parser<T> 
where T: Clone
{
    pub fn new(grammar: CompiledGrammar<T>) -> Self {

        // Populate the start set with the rules that have the start symbol as lhs.
        // This will be automatically extended to all the indirect rules when the first token is
        // processed. Since chart only grows, this will do useless checking every time the parser
        // gets reset to an empty buffer. However, compared to the rest of the work to be done,
        // this is negligible.
        let mut start_set = Vec::new();
        for i in 0..grammar.rule_count() {
            if grammar.is_start_rule(i) {
                start_set.push((DottedRule::new(i), StartDot::new(0)));
            }
        }

        Self {
            grammar,
            chart: Vec::new(),
            valid_entries: 0,
            start_set,
        }
    }

    /// The buffer has changed at index. All parse entries are invalid beginning with the given
    /// index.
    ///
    /// The chart will not be changed to keep the function small and fast.
    pub fn buffer_changed(&mut self, index: usize) {
        if index < self.valid_entries {
            self.valid_entries = index;
        }
    }

    /// Return index of first invalid buffer index.
    ///
    /// Helper function for parser update function
    pub fn parse_start(&self) -> usize {
        self.valid_entries
    }

    /// Process one entry in the buffer. To support lexers/character class mappers, this function
    /// does not take the buffer directly, but just one token. The caller respondible to ensure
    /// the token is extracted correctly and consistently.
    ///
    /// If the index is inside the already-parsed section, the valid part will be reset.
    ///
    /// If the index is inside the unparsed section, an error will be returned.
    ///
    /// If the index is at the first unparsed position, the token will be processed.
    ///
    /// The function returns true if the parse has been accepted.
    pub fn update(&mut self, index: usize, token: T) -> Result<bool> {
        self.buffer_changed(index);
        if index > self.valid_entries {
            return Err(Error::InvalidIndex);
        }

        // Index is valid. Get the state list. Extend the chart if necessary.
        if self.chart.len() <= index {
            // Should only need to add one state list
            self.chart.push(Vec::new());
            assert!(index < self.chart.len());
        }
        let state_list = &mut self.chart[index];

        // Remove stale entries.
        state_list.clear();

        // Get the state set to read from
        let last_state_list = if index == 0 {
            // First token ever, the previous state set is the start set.
            &self.start_set
        } else {
            &self.chart[index - 1]
        };

        // Process the state list while extending it.
        for last_state in last_state_list {
            let dr = &last_state.0;
            match self.grammar.dotted_symbol(dr.rule, dr.dot) {
                CompiledSymbol::NonTerminal(nt) => {
                    // Predict
                }
                CompiledSymbol::Terminal(t) => {
                    // Scan

                }
                CompiledSymbol::None => {
                    // Complete
                }
            }
        }

        Ok(false)
    }
}
