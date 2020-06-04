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

use super::grammar::{CompiledGrammar, CompiledSymbol, SymbolId, DottedRule};

/// Parser error codes
#[derive(Debug)]
pub enum Error {
    /// Invalid index was passed to update
    InvalidIndex,
}

/// Type alias for Results with Errors
type Result<T> = std::result::Result<T, Error>;

/// Entry in the parsing chart. Dotted rule indicate next symbol to be parsed
/// (terminal/non-terminal). Second field is start position in the input buffer.
///
/// Index is usize as to not limit the length of the input buffer.
///
/// TODO: Limit the size of the input buffer.
type ChartEntry = (DottedRule, usize);
type StateList = Vec<ChartEntry>;

/// Incrementally parse the input buffer.
pub struct Parser<T> {
    grammar: CompiledGrammar<T>,

    /// Parsing chart.
    ///
    /// Outer dimension index corresponds to buffer index. Inner dimensions are the possible rules that
    /// apply at this buffer index.
    ///
    /// chart[0] contains the rules that derive directly or indirectly from the start symbol. In
    /// general, chart[i+1] contain the rules that apply after buffer[i] has been processed.
    ///
    /// TODO: Flatten this array
    chart: Vec<StateList>,

    /// Number of buffer entries (from the beginning) where the parse is valid.
    ///
    /// This value might be decreased when the buffer is changed and increased when the parser is
    /// updated.
    ///
    /// The value is to interpreted as the index into the chart from which the scanner reads to
    /// check if the current token matches.
    valid_entries: usize,
}

/// Result of parser update.
#[derive(PartialEq, Debug)]
pub enum Verdict {
    /// Need more input to decide
    More,

    /// At least one rule of the start symbol has been completed
    Accept,

    /// There are no terminals for the next update to match. Input has been rejected.
    Reject,
}

fn add_to_state_list(state_list: &mut StateList, entry: ChartEntry) {
    for e in state_list.iter() {
        if *e == entry {
            return;
        }
    }
    state_list.push(entry);
}

fn predict<T>(
    state_list: &mut StateList,
    symbol: SymbolId,
    dot_buffer: usize,
    grammar: &CompiledGrammar<T>,
) where
    T: Clone,
{
    for i in 0..grammar.rule_count() {
        if grammar.lhs_is(i, symbol) {
            let new_entry = (DottedRule::new(i), dot_buffer);
            add_to_state_list(state_list, new_entry);
        }
    }
}

impl<T> Parser<T>
where
    T: Clone + PartialEq,
{
    pub fn new(grammar: CompiledGrammar<T>) -> Self {
        // Index 0 is special: It contains all the predictions of the start symbol. As the chart is
        // only extended while parsing, chart entries before the current one aren't changed. Thus,
        // the fully predicted chart[0] only needs to be generated once.
        let mut start_set = Vec::new();
        // Fill in the rules that have the start symbol as lhs.
        for i in 0..grammar.rule_count() {
            if grammar.is_start_rule(i) {
                let new_entry = (DottedRule::new(i), 0);
                add_to_state_list(&mut start_set, new_entry);
            }
        }

        // The predictor for the start state is also special. As no empty rules are allowed, there
        // is no need for *complete*.
        // Since the state list will grow during this operation, the index needs to be checked every
        // time.
        let mut i = 0;
        while i < start_set.len() {
            if let CompiledSymbol::NonTerminal(nt) = grammar.dotted_symbol(&start_set[i].0) {
                predict(&mut start_set, nt, 0, &grammar);
            }
            i += 1;
        }

        let mut chart = Vec::new();
        chart.push(start_set);
        Self {
            grammar,
            chart,
            valid_entries: 0,
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
    /// does not take the buffer directly, but just one token. The caller is respondible to ensure
    /// the token extraction is deterministc.
    ///
    /// If the index is inside the already-parsed section, the valid part will be reset.
    ///
    /// If the index is inside the unparsed section, an error will be returned.
    ///
    /// If the index is at the first unparsed position, the token will be processed.
    ///
    /// When the terminal has been processed, the next entry is fully predicted. This allows *ruby
    /// slippers* parsing when the user requests the acceptable tokens and inserts it into the
    /// buffer before updating the parser.
    ///
    /// The function returns if the input is accepted, rejected or still undecided.
    pub fn update(&mut self, index: usize, token: T) -> Result<Verdict> {
        self.buffer_changed(index);
        if index > self.valid_entries {
            return Err(Error::InvalidIndex);
        }

        // Index is valid.
        //
        // The chart must have at least one entry more than the buffer. That means chart[index+1]
        // needs to exist. If everything is correct so far and we're parsing the first time,
        // `index + 1 == chart.len()`. If we're not parsing the first time, the chart may be
        // longer.
        assert!(index + 1 <= self.chart.len());
        // Check if room for index+1 needs to be made.
        if (index + 1) == self.chart.len() {
            // Should only need to add one state list
            self.chart.push(Vec::new());
            assert!(index + 1 < self.chart.len());
        }
        // Get the state list to write to in the scanner. We work on a new vector to simplify the
        // access. This will change anyway when the chart is flattened.
        let mut new_state_list = Vec::new();
        self.chart[index + 1].clear();

        // Get the state list to read from
        let state_list = &self.chart[index];

        // Perform *scan*.
        //
        // The invariant of chart is that chart[i] has been fully predicted and completed before
        // update(i) is called. Thus, only *scan* remains to be done. The order of operations
        // doesn't matter as *scan* will not change the chart[i].
        let mut scanned = false;
        for state in state_list {
            let dr = &state.0;
            if let CompiledSymbol::Terminal(t) = self.grammar.dotted_symbol(&dr) {
                if t == token {
                    // Successful, advance the dot and store in new_state
                    let new_state = (dr.advance_dot(), state.1);
                    add_to_state_list(&mut new_state_list, new_state);
                    scanned = true;
                }
            }
        }

        if !scanned {
            return Ok(Verdict::Reject);
        }

        // Predict and complete the new state. This will usually grow the state list. Thus, indexed
        // access is required.
        let mut start_rule_completed = false;
        let mut i = 0;
        while i < new_state_list.len() {
            match self.grammar.dotted_symbol(&new_state_list[i].0) {
                CompiledSymbol::NonTerminal(nt) => {
                    predict(&mut new_state_list, nt, index + 1, &self.grammar)
                }
                CompiledSymbol::Terminal(_) => {
                    // Can't do anything as we don't know the new token.
                }
                CompiledSymbol::Completed(completed) => {
                    // Complete
                    start_rule_completed =
                        start_rule_completed | self.grammar.is_start_symbol(completed);
                    let start = new_state_list[i].1;
                    // Check all the rules at *start* if the dot is at the completed symbol
                    for rule in self.chart[start].iter() {
                        let start_dr = &rule.0;
                        if let CompiledSymbol::NonTerminal(maybe_completed) =
                            self.grammar.dotted_symbol(&start_dr)
                        {
                            if maybe_completed == completed {
                                let new_entry = (
                                    start_dr.advance_dot(),
                                        rule.1,
                                );
                                add_to_state_list(&mut new_state_list, new_entry);
                            }
                        }
                    }
                }
            }
            i += 1;
        }

        self.chart[index + 1] = new_state_list;
        self.valid_entries = index + 1;

        Ok(if start_rule_completed {
            Verdict::Accept
        } else {
            Verdict::More
        })
    }
}

impl<T> Parser<T>
where
    T: Clone + PartialEq + std::fmt::Display,
{
    pub fn print_chart(&self) {
        for i in 0..=self.valid_entries {
            println!("chart[{}]:", i);
            for e in self.chart[i].iter() {
                print!("  ");
                self.grammar.print_dotted_rule(&e.0);
                println!(", [{}]", e.1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::grammar::tests::define_grammar;

    #[test]
    fn seq_success() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char>::new(compiled_grammar);
        let mut index = 0;
        for (i, c) in "john called mary from denver".chars().enumerate() {
            println!("\n");
            parser.print_chart();
            println!("{}, '{}'", i, c);
            let res = parser.update(i, c);
            parser.print_chart();
            assert!(res.is_ok());
            assert!(res.unwrap() != Verdict::Reject);
            index = i;
        }
        println!("\n");
        parser.print_chart();
        println!("{}, ' '", index + 1);
        let res = parser.update(index + 1, ' ');
        parser.print_chart();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Verdict::Accept);
    }

    #[test]
    fn seq_fail() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char>::new(compiled_grammar);
        let mut index = 0;
        for (i, c) in "john ".chars().enumerate() {
            let res = parser.update(i, c);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::More);
            index = i;
        }
        let res = parser.update(index + 1, 'w');
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Verdict::Reject);
    }

    #[test]
    fn reset() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char>::new(compiled_grammar);

        // Start as "john called denver"
        for (i, c) in "john called denver".chars().enumerate() {
            let res = parser.update(i, c);
            assert!(res.is_ok());
            assert!(res.unwrap() != Verdict::Reject);
        }

        // Reset to the beginning of "denver"
        parser.buffer_changed(12);

        // Complete the sentence
        let mut index = 0;
        for (i, c) in "mary from denver".chars().enumerate() {
            index = i + 12;
            let res = parser.update(index, c);
            assert!(res.is_ok());
            assert!(res.unwrap() != Verdict::Reject);
        }

        let res = parser.update(index + 1, ' ');
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Verdict::Accept);
    }
}
