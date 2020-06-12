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

use super::grammar::{CompiledGrammar, CompiledSymbol, DottedRule, Matcher, SymbolId};

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

/// Entry in the parse tree.
///
/// The node of the tree are the chart entries. The edges are stored separately.
#[derive(PartialEq)]
struct CstEdge {
    /// Index into StateList at the buffer index where the edge originates.
    ///
    /// This allows access to the start index and the symbol
    from_state: SymbolId,

    /// Index into StateList at the buffer index where the edge ends
    to_state: SymbolId,

    /// Buffer index where the target of the link is to be found
    to_index: usize,
}

type CstList = Vec<CstEdge>;

/// Incrementally parse the input buffer.
pub struct Parser<T, M>
where
    M: Matcher<T>,
{
    grammar: CompiledGrammar<T, M>,

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

    /// Nodes of the parse tree.
    ///
    /// Uses the same indexing as chart.
    ///
    /// TODO: Flatten this array
    cst: Vec<CstList>,

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

fn add_to_state_list(state_list: &mut StateList, entry: ChartEntry) -> SymbolId {
    for (i, e) in state_list.iter().enumerate() {
        if *e == entry {
            return i as SymbolId;
        }
    }
    let res = state_list.len();
    state_list.push(entry);
    (res as SymbolId)
}

fn add_to_cst_list(cst_list: &mut CstList, entry: CstEdge) {
    for e in cst_list.iter() {
        if *e == entry {
            return;
        }
    }
    cst_list.push(entry);
}

fn predict<T, M>(
    state_list: &mut StateList,
    symbol: SymbolId,
    dot_buffer: usize,
    grammar: &CompiledGrammar<T, M>,
) where
    M: Matcher<T> + Clone,
{
    for i in 0..grammar.rule_count() {
        if grammar.lhs_is(i, symbol) {
            let new_entry = (DottedRule::new(i), dot_buffer);
            add_to_state_list(state_list, new_entry);
        }
    }
}

impl<T, M> Parser<T, M>
where
    T: Clone,
    M: Matcher<T> + Clone,
{
    pub fn new(grammar: CompiledGrammar<T, M>) -> Self {
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

        // The predictor for the start state is also special. As empty rules are allowed,
        // *complete* needs to run. However, it is restricted to start == 0.  Since the state list
        // will grow during this operation, the index needs to be checked every time.
        let mut new_cst_list = Vec::new();
        let mut i = 0;
        while i < start_set.len() {
            match grammar.dotted_symbol(&start_set[i].0) {
                CompiledSymbol::NonTerminal(nt) => predict(&mut start_set, nt, 0, &grammar),
                CompiledSymbol::Terminal(_) => {
                    // Can't do anything as we don't know the first token.
                }
                CompiledSymbol::Completed(completed) => {
                    // Complete
                    let start = start_set[i].1;
                    // Check all the rules at *start* if the dot is at the completed symbol. Start
                    // must be 0. Thus a double-borrow would occur of this done with an iterator.
                    let mut rule_index = 0;
                    while rule_index < start_set.len() {
                        if let CompiledSymbol::NonTerminal(maybe_completed) =
                            grammar.dotted_symbol(&start_set[rule_index].0)
                        {
                            if maybe_completed == completed {
                                // Update the Earley chart
                                let new_entry = (
                                    start_set[rule_index].0.advance_dot(),
                                    start_set[rule_index].1,
                                );
                                let new_state = add_to_state_list(&mut start_set, new_entry);
                                // Create the CST edge from the completed rule to the rule that
                                // started it, i.e. the parent/child link. Keep in mind that the
                                // links have to go towards the older entries to keep them
                                // consistent with the siblings edges.
                                add_to_cst_list(
                                    &mut new_cst_list,
                                    CstEdge {
                                        from_state: new_state,
                                        to_state: i as SymbolId,
                                        to_index: 0,
                                    },
                                );
                                // Create the CST edge how the dot moved, i.e. the sibling link. Omit
                                // links to the beginning of rules as they can't link to further
                                // completions.
                                if !start_set[rule_index].0.is_first() {
                                    add_to_cst_list(
                                        &mut new_cst_list,
                                        CstEdge {
                                            from_state: new_state,
                                            to_state: rule_index as SymbolId,
                                            to_index: start,
                                        },
                                    );
                                }
                            }
                        }
                        rule_index += 1;
                    }
                }
            }
            i += 1;
        }

        let mut chart = Vec::new();
        chart.push(start_set);
        let mut cst = Vec::new();
        cst.push(new_cst_list);
        Self {
            grammar,
            chart,
            cst,
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
            self.cst.push(Vec::new());
            assert_eq!(self.cst.len(), self.chart.len());
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
                if t.matches(token.clone()) {
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

        let mut new_cst_list = Vec::new();

        // In order to handle empty rules, the chart must be used, not a separate copy.
        let new_index = index + 1;
        self.chart[new_index] = new_state_list;
        self.cst[new_index] = new_cst_list;

        // Predict and complete the new state. This will usually grow the state list. Thus, indexed
        // access is required.
        let mut start_rule_completed = false;
        let mut i = 0;
        while i < self.chart[new_index].len() {
            match self.grammar.dotted_symbol(&self.chart[new_index][i].0) {
                CompiledSymbol::NonTerminal(nt) => {
                    predict(&mut self.chart[new_index], nt, new_index, &self.grammar)
                }
                CompiledSymbol::Terminal(_) => {
                    // Can't do anything as we don't know the new token.
                }
                CompiledSymbol::Completed(completed) => {
                    // Complete
                    start_rule_completed =
                        start_rule_completed | self.grammar.is_start_symbol(completed);
                    let start = self.chart[new_index][i].1;
                    // Check all the rules at *start* if the dot is at the completed symbol
                    let mut rule_index = 0;
                    while rule_index < self.chart[start].len() {
                        if let CompiledSymbol::NonTerminal(maybe_completed) =
                            self.grammar.dotted_symbol(&self.chart[start][rule_index].0)
                        {
                            if maybe_completed == completed {
                                // Update the Earley chart
                                let new_entry = (
                                    self.chart[start][rule_index].0.advance_dot(),
                                    self.chart[start][rule_index].1,
                                );
                                let new_state =
                                    add_to_state_list(&mut self.chart[new_index], new_entry);
                                // Create the CST edge from the completed rule to the rule that
                                // started it, i.e. the parent/child link. Keep in mind that the
                                // links have to go towards the older entries to keep them
                                // consistent with the siblings edges.
                                add_to_cst_list(
                                    &mut self.cst[new_index],
                                    CstEdge {
                                        from_state: new_state,
                                        to_state: i as SymbolId,
                                        to_index: new_index,
                                    },
                                );
                                // Create the CST edge how the dot moved, i.e. the sibling link. Omit
                                // links to the beginning of rules as they can't link to further
                                // completions.
                                if !self.chart[start][rule_index].0.is_first() {
                                    add_to_cst_list(
                                        &mut self.cst[new_index],
                                        CstEdge {
                                            from_state: new_state,
                                            to_state: rule_index as SymbolId,
                                            to_index: start,
                                        },
                                    );
                                }
                            }
                        }
                        rule_index += 1;
                    }
                }
            }
            i += 1;
        }

        self.valid_entries = index + 1;

        Ok(if start_rule_completed {
            Verdict::Accept
        } else {
            Verdict::More
        })
    }
}

impl<T, M> Parser<T, M>
where
    M: Matcher<T> + Clone + PartialEq + std::fmt::Debug,
{
    pub fn print_chart(&self) {
        for i in 0..=self.valid_entries {
            println!("chart[{}]:", i);
            for e in self.chart[i].iter() {
                println!(
                    "  {}, [{}]",
                    self.grammar.dotted_rule_to_string(&e.0).unwrap(),
                    e.1
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::char::CharMatcher;
    use super::super::grammar::tests::define_grammar;
    use super::super::grammar::{Grammar, Symbol};

    /// Define the grammar from: https://www.cs.unm.edu/~luger/ai-final2/CH9_Dynamic%20Programming%20and%20the%20Earley%20Parser.pdf
    ///
    /// These are the alrady tokenized words
    #[derive(Hash, PartialOrd, PartialEq, Clone, Debug, Eq, Ord)]
    pub enum Token {
        John,
        Called,
        Mary,
        From,
        Denver,
    }

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
    pub fn token_grammar() -> Grammar<Token, Token> {
        let mut grammar: Grammar<Token, Token> = Grammar::new();

        use super::super::grammar::Symbol::*;
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
        grammar.add_rule("Noun".to_string(), vec![Terminal(Token::John)]);
        grammar.add_rule("Noun".to_string(), vec![Terminal(Token::Mary)]);
        grammar.add_rule("Noun".to_string(), vec![Terminal(Token::Denver)]);
        grammar.add_rule("Verb".to_string(), vec![Terminal(Token::Called)]);
        grammar.add_rule("Prep".to_string(), vec![Terminal(Token::From)]);

        grammar
    }

    /// Successfully parse the example from
    /// https://www.cs.unm.edu/~luger/ai-final2/CH9_Dynamic%20Programming%20and%20the%20Earley%20Parser.pdf.
    ///
    /// Print the parse chart at the end.
    ///
    /// Generate input for a visual representation using `dot`. Show with:
    /// `cargo test -- --test-threads 1 --nocapture | grep '^dot:' | cut -f2 > john.dot && dot -O -Tpng john.dot`
    ///
    /// The graph is in `john.dot.png`.
    #[test]
    fn seq_success() {
        let grammar = token_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<Token, Token>::new(compiled_grammar);
        let mut index = 0;
        for (i, c) in [Token::John, Token::Called, Token::Mary, Token::From]
            .iter()
            .enumerate()
        {
            let res = parser.update(i, c.clone());
            assert!(res.is_ok());
            assert!(res.unwrap() != Verdict::Reject);
            index = i;
        }
        let res = parser.update(index + 1, Token::Denver);
        parser.print_chart();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Verdict::Accept);

        // Print the parse tree for dot
        println!("\ndot:\tdigraph {{");
        // Print the nodes, using their index as an id
        for (chart_index, state_list) in parser.chart.iter().enumerate() {
            for (state_index, state) in state_list.iter().enumerate() {
                println!(
                    "dot:\tc_{}_{} [label=\"{} [{},{}]\"]",
                    chart_index,
                    state_index,
                    parser.grammar.dotted_rule_to_string(&state.0).unwrap(),
                    state.1,
                    chart_index
                );
            }
        }
        // Print the edges
        for (from_index, es) in parser.cst.iter().enumerate() {
            for e in es.iter() {
                println!(
                    "dot:\tc_{}_{}  -> c_{}_{}",
                    from_index, e.from_state, e.to_index, e.to_state
                );
            }
        }
        println!("dot:\t}}");
    }

    #[test]
    fn seq_fail() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);
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

        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);

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

    /// Test a grammar with empty rules
    ///
    /// S = a maybe_b c
    /// maybe_b = b
    /// maybe_b =
    #[test]
    fn empty() {
        let mut grammar = Grammar::<char, CharMatcher>::new();
        use CharMatcher::*;
        use Symbol::*;
        grammar.set_start("S".to_string());
        grammar.add_rule(
            "S".to_string(),
            vec![
                Terminal(Exact('a')),
                NonTerminal("maybe_b".to_string()),
                Terminal(Exact('c')),
            ],
        );
        grammar.add_rule("maybe_b".to_string(), vec![Terminal(Exact('b'))]);
        grammar.add_rule("maybe_b".to_string(), vec![]);

        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);

        // "abc" should be acceptable
        {
            let res = parser.update(0, 'a');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::More);
        }
        {
            let res = parser.update(1, 'b');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::More);
        }
        {
            let res = parser.update(2, 'c');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::Accept);
        }

        // "ac" should be acceptable
        parser.buffer_changed(0);
        {
            let res = parser.update(0, 'a');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::More);
        }
        {
            let res = parser.update(1, 'c');
            parser.print_chart();
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::Accept);
        }
        // "abb" should fail
        parser.buffer_changed(0);
        {
            let res = parser.update(0, 'a');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::More);
        }
        {
            let res = parser.update(1, 'b');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::More);
        }
        {
            let res = parser.update(2, 'b');
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Verdict::Reject);
        }
    }
}
