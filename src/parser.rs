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

//! Earley Parser

use itertools::Itertools;

use super::grammar::{CompiledGrammar, CompiledSymbol, DottedRule, Matcher, SymbolId, ERROR_ID};

/// Entry in the parsing chart. Dotted rule indicate next symbol to be parsed
/// (terminal/non-terminal). Second field is start position in the token buffer.
///
/// Position is usize as to not limit the length of the input buffer.
///
/// TODO: Limit the size of the input buffer.
type ChartEntry = (DottedRule, usize);

/// Ordered list of states for one position of the token buffer.
type StateList = Vec<ChartEntry>;

/// Entry in the parse tree.
///
/// The node of the tree are the parse state entries in the chart. The edges are stored separately.
#[derive(PartialEq)]
struct CstEdge {
    /// Index into StateList at the buffer position where the edge originates.
    ///
    /// This allows access to the start index and the symbol
    from_state: SymbolId,

    /// Index into StateList at the buffer position where the edge ends
    to_state: SymbolId,

    /// Buffer position where the target of the link is to be found
    to_position: usize,
}

/// List of edges at a given buffer position
type CstList = Vec<CstEdge>;

/// Earley Parser on streams.
///
/// Incrementally parse the input steam using the Earley Algorithm. Does not store any parsed
/// tokens itself. If the parsed tokens cannot be reconstructed from a successful parse, they need
/// to be stored separately.
///
/// It is technically possible to change the grammar on the fly, but not implemented. File a
/// feature request if you need that.
pub struct Parser<T, M>
where
    M: Matcher<T>,
{
    /// Compiled grammar to parse.
    grammar: CompiledGrammar<T, M>,

    /// Parsing chart.
    ///
    /// Outer dimension index corresponds to buffer position. Inner dimensions are the possible
    /// rules that apply at this buffer index.
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
    /// This value might be decreased when the buffer is changed and will increase when the parser is
    /// updated.
    ///
    /// The value is to interpreted as the index into the chart from which the scanner reads to
    /// check if the current token matches.
    valid_entries: usize,
}

/// Result of parser update.
#[derive(PartialEq, Debug)]
pub enum Verdict {
    /// Buffer position to continue parsing was incorrect.
    InvalidPosition,

    /// Need more input to decide
    More,

    /// At least one rule of the start symbol has been completed
    Accept,

    /// There are no terminals for the next update to match. Input has been rejected.
    Reject,
}

/// Identify a node in a CST path
#[derive(Clone, Debug)]
pub struct CstPathNode {
    /// Index into buffer/chart
    position: usize,
    /// Index into chart list at buffer position
    state: SymbolId,
}

/// Path from root of parse tree to current node
#[derive(Debug)]
pub struct CstPath(pub Vec<CstPathNode>);

/// One node in the parse tree as returned by the iterator
#[derive(Debug)]
pub struct CstIterItemNode {
    /// Start position of the parsed item. All tokens at positions *p* with `start` <= *p* < `end`
    pub start: usize,
    /// End position of the parsed item. All tokens at positions *p* with `start` <= *p* < `end`
    pub end: usize,
    /// The dotted rule that applied. The dot is right before the parser item.
    pub dotted_rule: DottedRule,
    /// Path from the root of the parse tree to this node.
    ///
    /// Only contains completed rules. Does not contain the path node to locate the current node.
    pub path: CstPath,
    /// Current node as a path node.
    pub current: CstPathNode,
}

/// Returned by the `CstIter` for each parsed element.
#[derive(Debug)]
pub enum CstIterItem {
    /// A node of the parse tree
    Parsed(CstIterItemNode),

    /// Beginning at this index, the buffer has not been parsed
    Unparsed(usize),
}

/// Iterator to access the parse tree in pre-order.
///
/// Returns all parsed nodes, then the index of the first unparsed position of the buffer.
pub struct CstIter<'a, T, M>
where
    M: Matcher<T>,
{
    /// The parser
    parser: &'a Parser<T, M>,

    /// Graph nodes to be visited.
    /// Contains (item, completed)
    stack: Vec<(CstPathNode, bool)>,

    /// Index of the first unparsed token
    unparsed: usize,

    /// State: Has unparsed been returned
    done: bool,
}

/// Add an entry to a state list if the entry does not already exist.
///
/// Return the index into the state list.
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

/// Add an entry to the CST edge list if the entry does not already exist.
fn add_to_cst_list(cst_list: &mut CstList, entry: CstEdge) {
    for e in cst_list.iter() {
        if *e == entry {
            return;
        }
    }
    cst_list.push(entry);
}

/// Predict function of the Earley Algorithm.
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
    /// Create a new parser, given a grammar.
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
                CompiledSymbol::NonTerminal(nt) => {
                    predict(&mut start_set, nt, 0, &grammar);
                    if grammar.nt_with_empty_rule(nt) {
                        let new_entry = (start_set[i].0.advance_dot(), start_set[i].1);
                        add_to_state_list(&mut start_set, new_entry);
                    }
                }
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
                                        to_position: 0,
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
                                            to_position: start,
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

    /// Borrow the grammar
    pub fn grammar<'a>(&'a self) -> &'a CompiledGrammar<T, M> {
        &self.grammar
    }

    /// Get the dotted rule from a CST path node.
    pub fn dotted_rule(&self, node: &CstPathNode) -> DottedRule {
        self.chart[node.position][node.state as usize].0.clone()
    }

    /// The buffer has changed at `position`. All parse entries are invalid beginning with the given
    /// position.
    ///
    /// The chart will not be changed to keep the function small and fast.
    pub fn buffer_changed(&mut self, position: usize) {
        if position < self.valid_entries {
            self.valid_entries = position;
        }
    }

    /// Process one entry in the buffer. To support lexers/character class mappers, this function
    /// does not take the buffer directly, but just one token. The caller is respondible to ensure
    /// the token extraction is deterministc.
    ///
    /// If the position is inside the already-parsed section, the valid part will be reset.
    ///
    /// If the position is inside the unparsed section, an error will be returned.
    ///
    /// If the position is at the first unparsed position, the token will be processed.
    ///
    /// When the terminal has been processed, the next entry is fully predicted. This allows *ruby
    /// slippers* parsing when the user requests the acceptable tokens and inserts it into the
    /// buffer before updating the parser.
    ///
    /// The function returns whether the input is accepted, rejected or still undecided.
    pub fn update(&mut self, position: usize, token: T) -> Verdict {
        self.buffer_changed(position);
        if position > self.valid_entries {
            return Verdict::InvalidPosition;
        }

        // position is valid.
        //
        // The chart must have at least one entry more than the buffer. That means chart[position+1]
        // needs to exist. If everything is correct so far and we're parsing the first time,
        // `position + 1 == chart.len()`. If we're not parsing the first time, the chart may be
        // longer.
        debug_assert!(position + 1 <= self.chart.len());
        // Check if room for position+1 needs to be made.
        if (position + 1) == self.chart.len() {
            // Should only need to add one state list
            self.chart.push(Vec::new());
            debug_assert!(position + 1 < self.chart.len());
            self.cst.push(Vec::new());
            debug_assert_eq!(self.cst.len(), self.chart.len());
        }
        // Get the state list to write to in the scanner. We work on a new vector to simplify the
        // access. This will change anyway when the chart is flattened.
        let mut new_state_list = Vec::new();
        self.chart[position + 1].clear();

        // Get the state list to read from
        let state_list = &self.chart[position];

        // New entries for cst edge. Child edges need to come first for iterator to work. In case
        // of errors, the error links need to come first.
        let mut cst_child_list = Vec::new();
        let mut cst_sibling_list = Vec::new();

        // Perform *scan*.
        //
        // The invariant of chart is that chart[i] has been fully predicted and completed before
        // update(i) is called. Thus, only *scan* remains to be done. The order of operations
        // doesn't matter as *scan* will not change the chart[i].
        let mut scanned = false;
        for (state_index, state) in state_list.iter().enumerate() {
            let dr = &state.0;
            if let CompiledSymbol::Terminal(t) = self.grammar.dotted_symbol(&dr) {
                if t.matches(token.clone()) {
                    // Successful, advance the dot and store in new_state
                    let new_entry = (dr.advance_dot(), state.1);
                    let new_state = add_to_state_list(&mut new_state_list, new_entry);

                    // Add a sibling link if this isn't the first symbol in the rule.
                    if !dr.is_first() {
                        add_to_cst_list(
                            &mut cst_sibling_list,
                            CstEdge {
                                from_state: new_state,
                                to_state: state_index as SymbolId,
                                to_position: position,
                            },
                        );
                    }

                    scanned = true;
                }
            }
        }

        let mut verdict = None;

        // In order to handle empty rules, the chart must be used, not a separate copy.
        let new_position = position + 1;
        self.chart[new_position] = new_state_list;

        if !scanned {
            // None of the predicted symbols matched.
            // Remedy: Accept all terminals and insert one error pseudo-rule per terminal into the
            //         parse tree. Then, predict as usual, but link the
            //         predictions to the error rules.

            // Only process the existing entries.
            for i in 0..self.chart[position].len() {
                let dr = &self.chart[position][i].0;
                if let CompiledSymbol::Terminal(_t) = self.grammar.dotted_symbol(&dr) {
                    // Pretend to be successful, advance the dot and store in new_state
                    let new_entry = (dr.advance_dot(), self.chart[position][i].1);
                    let new_state = add_to_state_list(&mut self.chart[new_position], new_entry);
                    // Mark as error by adding the error pseudo-rule
                    let error_state = self.chart[new_position].len() as SymbolId;
                    self.chart[new_position].push((DottedRule::new(ERROR_ID as usize), position));

                    // Link pretended match to error entry. Must not be de-duplicated if multiple
                    // errors occur.
                    cst_child_list.push(CstEdge {
                        from_state: new_state,
                        to_state: error_state,
                        to_position: new_position,
                    });
                }
            }

            verdict = Some(Verdict::Reject);
        }

        // Predict and complete the new state. This will usually grow the state list. Thus, indexed
        // access is required.
        let mut start_rule_completed = false;
        let mut i = 0;
        while i < self.chart[new_position].len() {
            match self.grammar.dotted_symbol(&self.chart[new_position][i].0) {
                CompiledSymbol::NonTerminal(nt) => {
                    predict(
                        &mut self.chart[new_position],
                        nt,
                        new_position,
                        &self.grammar,
                    );
                    if self.grammar.nt_with_empty_rule(nt) {
                        let new_entry = (
                            self.chart[new_position][i].0.advance_dot(),
                            self.chart[new_position][i].1,
                        );
                        let new_state = add_to_state_list(&mut self.chart[new_position], new_entry);
                        // Add a CST sibling link to the previous position as not to break the
                        // tree.
                        add_to_cst_list(
                            &mut cst_sibling_list,
                            CstEdge {
                                from_state: new_state,
                                to_state: i as SymbolId,
                                to_position: new_position,
                            },
                        );
                    }
                }
                CompiledSymbol::Terminal(_) => {
                    // Can't do anything as we don't know the new token.
                }
                CompiledSymbol::Completed(completed) => {
                    // Complete
                    start_rule_completed =
                        start_rule_completed | self.grammar.is_start_symbol(completed);
                    let start = self.chart[new_position][i].1;
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
                                    add_to_state_list(&mut self.chart[new_position], new_entry);
                                // Create the CST edge from the completed rule to the rule that
                                // started it, i.e. the parent/child link. Keep in mind that the
                                // links have to go towards the older entries to keep them
                                // consistent with the siblings edges.
                                add_to_cst_list(
                                    &mut cst_child_list,
                                    CstEdge {
                                        from_state: new_state,
                                        to_state: i as SymbolId,
                                        to_position: new_position,
                                    },
                                );
                                // Create the CST edge how the dot moved, i.e. the sibling link. Omit
                                // links to the beginning of rules as they can't link to further
                                // completions.
                                if !self.chart[start][rule_index].0.is_first() {
                                    add_to_cst_list(
                                        &mut cst_sibling_list,
                                        CstEdge {
                                            from_state: new_state,
                                            to_state: rule_index as SymbolId,
                                            to_position: start,
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

        self.cst[new_position] = cst_child_list;
        self.cst[new_position].append(&mut cst_sibling_list);

        self.valid_entries = new_position;

        verdict = verdict.or_else(|| {
            Some(if start_rule_completed {
                Verdict::Accept
            } else {
                Verdict::More
            })
        });

        verdict.unwrap()
    }

    /// Return a pre-order CST iterator, starting at the last position that accepted the input.
    pub fn cst_iter(&self) -> CstIter<T, M> {
        // Collect all the entries that complete a start symbol. Search backwards from the last
        // entry.
        let mut stack = Vec::new();

        debug_assert!(self.valid_entries < self.chart.len());
        debug_assert!(self.valid_entries < self.cst.len());
        debug_assert!(self.chart.len() == self.cst.len());
        let mut position = self.valid_entries;
        let mut unparsed = position;
        loop {
            for (rule_index, rule) in self.chart[position].iter().enumerate() {
                // If the rule indicates a completed start symbol, push it to the stack.
                if self.grammar.dotted_is_completed_start(&rule.0) {
                    stack.push((
                        CstPathNode {
                            position,
                            state: rule_index as SymbolId,
                        },
                        false,
                    ));
                }
            }
            if !stack.is_empty() {
                break;
            }
            if position == 0 {
                break;
            }
            position -= 1;
            unparsed = position;
        }

        CstIter {
            parser: &self,
            stack,
            unparsed,
            done: false,
        }
    }

    /// Return a pre-order CST iterator, starting at the current position regardless of acceptance.
    ///
    /// This will return all partial parses that are possible at the given position.
    ///
    /// Return None if the position was invalid.
    pub fn partial_cst_iter(&self, position: usize) -> Option<CstIter<T, M>> {
        if position >= self.valid_entries {
            return None;
        }

        // Collect all the entries at the position
        let mut stack = Vec::new();

        for rule_index in 0..self.chart[position].len() {
            stack.push((
                CstPathNode {
                    position,
                    state: rule_index as SymbolId,
                },
                false,
            ));
        }

        Some(CstIter {
            parser: &self,
            stack,
            unparsed: position,
            done: false,
        })
    }

    /// Iterate through the predictions in the same order that the cst would generate them.
    ///
    /// Return None if position is invalid
    pub fn predictions(&self, position: usize) -> Vec<SymbolId> {
        debug_assert!(self.valid_entries < self.chart.len());
        if position >= self.chart.len() {
            return Vec::new();
        }
        // In ambiguous grammars, the symbols might appear multiple times
        self.chart[position]
            .iter()
            .rev()
            .filter_map(|state| {
                if state.0.is_first() {
                    Some(self.grammar.lhs(state.0.rule as usize))
                } else {
                    None
                }
            })
            .unique()
            .collect()
    }
}

impl<'a, T, M> Iterator for CstIter<'a, T, M>
where
    M: Matcher<T> + Clone,
{
    type Item = CstIterItem;

    fn next(&mut self) -> Option<CstIterItem> {
        // Traverse the tree
        // Algo
        // - If the stack is empty, switch to end sequence (return unparsed, then none)
        // - Get the top-of-stack (TOS) item, but leave it on the stack. There is at least one entry.
        // - If the TOS is marked as completed, return it. In that case, all outgoing nodes
        //   have been processed in previous calls.
        // - Mark the TOS as completed. If there are outgoing edges, the will be processed before
        //   the TOS. If we return to this entry later, we know, it has been processed and can be
        //   returned.
        // - Process the ooutgoing edges in order. This will process the parent/child links (i.e. downward
        //   links) first. That way, thwy will be put on the stack first, i.e. processed later.
        // - Put the node the edge points to on the stack, mark as incomplete.
        // - Continue with the new TOS item.
        loop {
            if let Some(tos) = self.stack.last_mut() {
                if tos.1 {
                    // TOS is complete
                    let tos = self.stack.pop().unwrap();
                    let state = &self.parser.chart[tos.0.position][tos.0.state as usize];
                    let start = state.1;
                    let end = tos.0.position;
                    // The path is the list of completed, processed entries on the stack.
                    let path = CstPath(
                        self.stack
                            .iter()
                            .filter_map(|(node, processed)| {
                                let is_result = if *processed {
                                    let dr =
                                        &self.parser.chart[node.position][node.state as usize].0;
                                    self.parser.grammar.dotted_symbol(dr).is_complete()
                                } else {
                                    false
                                };
                                if is_result {
                                    Some(node.clone())
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    );

                    let node = CstIterItemNode {
                        start,
                        end,
                        dotted_rule: state.0.clone(),
                        path,
                        current: tos.0.clone(),
                    };
                    return Some(CstIterItem::Parsed(node));
                } else {
                    // TOS is no processed yet, mark it and process.
                    tos.1 = true;
                    // Find the edges and put the node they point to on the stack.
                    let from_state = tos.0.state;
                    let from_position = tos.0.position;
                    for edge in self.parser.cst[from_position].iter() {
                        if edge.from_state == from_state {
                            let node = CstPathNode {
                                position: edge.to_position,
                                state: edge.to_state,
                            };
                            self.stack.push((node, false));
                        }
                    }
                }
            } else {
                if self.done {
                    return None;
                } else {
                    self.done = true;
                    return Some(CstIterItem::Unparsed(self.unparsed));
                }
            }
        }
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

    pub fn trace_chart(&self) {
        for i in 0..=self.valid_entries {
            trace!("chart[{}]:", i);
            for (j, e) in self.chart[i].iter().enumerate() {
                trace!(
                    "  {:6}: {}, [{}]",
                    j,
                    self.grammar.dotted_rule_to_string(&e.0).unwrap(),
                    e.1
                );
            }
        }
    }
}

impl CstIterItemNode {
    pub fn path_iter(&self) -> impl Iterator<Item = &CstPathNode> {
        self.path.0.iter()
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

    fn print_cst_as_dot<T, M>(parser: &Parser<T, M>, prefix: &str, preorder: bool)
    where
        M: Matcher<T> + Clone + std::fmt::Debug,
        T: Clone,
    {
        // Print the parse tree for dot
        println!("\n{}:\tdigraph {{", prefix);
        // Print the nodes, using their position as an id
        for (chart_index, state_list) in parser.chart.iter().enumerate() {
            for (state_index, state) in state_list.iter().enumerate() {
                println!(
                    "{}:\tc_{}_{} [label=\"{} [{},{}]\"]",
                    prefix,
                    chart_index,
                    state_index,
                    parser.grammar.dotted_rule_to_string(&state.0).unwrap(),
                    state.1,
                    chart_index
                );
            }
        }
        // Print the edges
        for (from_position, es) in parser.cst.iter().enumerate() {
            for e in es.iter() {
                println!(
                    "{}:\tc_{}_{}  -> c_{}_{}",
                    prefix, from_position, e.from_state, e.to_position, e.to_state
                );
            }
        }

        if preorder {
            // Print the CST in pre-order
            let mut last_cst_node: Option<CstPathNode> = None;
            for (i, cst_item) in parser.cst_iter().enumerate() {
                if let CstIterItem::Parsed(cst_node) = cst_item {
                    if let Some(last_cst_node) = last_cst_node {
                        println!(
                            "{}:\tc_{}_{}  -> c_{}_{} [label=\"{}\",color=red]",
                            prefix,
                            last_cst_node.position,
                            last_cst_node.state,
                            cst_node.current.position,
                            cst_node.current.state,
                            i,
                        );
                    }

                    last_cst_node = Some(cst_node.current.clone());
                }
            }
        }
        println!("{}:\t}}", prefix);
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
    /// `cargo test -- --test-threads 1 --nocapture | grep '^john:' | cut -f2 > john.dot && dot -O -Tpng john.dot`
    ///
    /// The graph is in `john.dot.png`.
    #[test]
    fn seq_success() {
        let grammar = token_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<Token, Token>::new(compiled_grammar);
        let mut position = 0;
        for (i, c) in [Token::John, Token::Called, Token::Mary, Token::From]
            .iter()
            .enumerate()
        {
            let res = parser.update(i, c.clone());
            assert!(res != Verdict::Reject);
            position = i;
        }
        let res = parser.update(position + 1, Token::Denver);
        parser.print_chart();
        assert_eq!(res, Verdict::Accept);

        print_cst_as_dot(&parser, "john", false);

        let cst_iter = parser.cst_iter();
        for i in cst_iter {
            match i {
                CstIterItem::Parsed(i) => {
                    println!(
                        "iter: {}, {}-{}",
                        parser
                            .grammar
                            .dotted_rule_to_string(&i.dotted_rule)
                            .unwrap(),
                        i.start,
                        i.end
                    );
                    for n in i.path.0.iter() {
                        let dr = &parser.chart[n.position][n.state as usize].0;
                        println!(
                            "iter:   {}",
                            parser.grammar.dotted_rule_to_string(&dr).unwrap()
                        );
                    }
                }
                _ => {
                    println!("iter: {:?}", i);
                }
            }
        }

        // Construct the node parse tree iterator
        let mut cst_iter = parser.cst_iter();

        // It should contain single entry on the stack and nothing unparsed.
        assert_eq!(cst_iter.stack.len(), 1);
        assert_eq!(cst_iter.unparsed, 5);

        // Get the items in sequence. Check only the depth of path.
        if let CstIterItem::Parsed(node) = cst_iter.next().expect("item 0") {
            assert_eq!(node.start, 0);
            assert_eq!(node.end, 1);
            assert_eq!(node.path.0.len(), 2);
        } else {
            panic!("Item 0 should be CstIterItem::Parsed.");
        }
        if let CstIterItem::Parsed(node) = cst_iter.next().expect("item 1") {
            assert_eq!(node.start, 0);
            assert_eq!(node.end, 1);
            assert_eq!(node.path.0.len(), 1);
        } else {
            panic!("Item 1 should be CstIterItem::Parsed.");
        }
    }

    #[test]
    fn seq_fail() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);
        let mut position = 0;
        for (i, c) in "john ".chars().enumerate() {
            let res = parser.update(i, c);
            assert_eq!(res, Verdict::More);
            position = i;
        }
        let res = parser.update(position + 1, 'w');
        assert_eq!(res, Verdict::Reject);

        // Construct the node parse tree iterator
        let mut cst_iter = parser.cst_iter();

        // It should contain an empty stack and everything is unparsed.
        assert_eq!(cst_iter.stack.len(), 0);
        assert_eq!(cst_iter.unparsed, 0);

        // Test the end sequence of the iterator
        let item = cst_iter.next();
        assert!(item.is_some());
        match item {
            Some(CstIterItem::Unparsed(_)) => {
                // All fine.
            }
            _ => {
                panic!("Expected Unparsed.");
            }
        }
        let item = cst_iter.next();
        assert!(item.is_none());
    }

    #[test]
    fn reset() {
        let grammar = define_grammar();
        let compiled_grammar = grammar.compile().expect("compilation should have worked");

        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);

        // Start as "john called denver"
        for (i, c) in "john called denver".chars().enumerate() {
            let res = parser.update(i, c);
            assert!(res != Verdict::Reject);
        }

        // Reset to the beginning of "denver"
        parser.buffer_changed(12);

        // Complete the sentence
        let mut position = 0;
        for (i, c) in "mary from denver".chars().enumerate() {
            position = i + 12;
            let res = parser.update(position, c);
            assert!(res != Verdict::Reject);
        }

        let res = parser.update(position + 1, ' ');
        assert_eq!(res, Verdict::Accept);
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
            assert_eq!(res, Verdict::More);
        }
        {
            let res = parser.update(1, 'b');
            assert_eq!(res, Verdict::More);
        }
        {
            let res = parser.update(2, 'c');
            assert_eq!(res, Verdict::Accept);
        }

        // "ac" should be acceptable
        parser.buffer_changed(0);
        {
            let res = parser.update(0, 'a');
            assert_eq!(res, Verdict::More);
        }
        {
            let res = parser.update(1, 'c');
            parser.print_chart();
            assert_eq!(res, Verdict::Accept);
        }
        // "abb" should fail
        parser.buffer_changed(0);
        {
            let res = parser.update(0, 'a');
            assert_eq!(res, Verdict::More);
        }
        {
            let res = parser.update(1, 'b');
            assert_eq!(res, Verdict::More);
        }
        {
            let res = parser.update(2, 'b');
            assert_eq!(res, Verdict::Reject);
        }
    }

    /// Test error handling
    ///
    /// S = A B
    /// A = a A
    /// A = a
    /// B = b
    /// B = c
    ///
    /// Input:
    /// `aadefaab`
    ///
    /// Output (E = error)
    /// AAEEEAAB
    ///
    /// Print the parse chart at the end.
    ///
    /// Generate input for a visual representation using `dot`. Show with:
    /// `cargo test -- --test-threads 1 --nocapture | grep '^error:' | cut -f2 > error.dot && dot -O -Tpng error.dot`
    ///
    /// The graph is in `error.dot.png`.
    #[test]
    fn error() {
        let mut grammar = Grammar::<char, CharMatcher>::new();
        use super::super::grammar::Rule;
        use CharMatcher::*;
        use Verdict::*;
        grammar.set_start("S".to_string());
        grammar.add(Rule::new("S").nt("A").nt("B"));
        grammar.add(Rule::new("A").t(Exact('a')).nt("A"));
        grammar.add(Rule::new("A").t(Exact('a')));
        grammar.add(Rule::new("B").t(Exact('b')));
        grammar.add(Rule::new("B").t(Exact('c')));

        let compiled_grammar = grammar.compile().expect("compilation should have worked");
        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);

        // "aab" should be accepted
        for (i, (c, v)) in [('a', More), ('a', More), ('b', Accept)].iter().enumerate() {
            let res = parser.update(i, *c);
            assert_eq!(res, *v);
        }

        // "adab" should fail and recover
        for (i, (c, v)) in [
            ('a', More),
            ('d', Reject),
            ('e', Reject),
            ('a', More),
            ('b', Accept),
        ]
        .iter()
        .enumerate()
        {
            let res = parser.update(i, *c);
            eprintln!("c={:?}, res={:?}", *c, res);
            assert_eq!(res, *v);
        }

        parser.print_chart();
        print_cst_as_dot(&parser, "error", true);

        // Go through the parse tree
        for (cst_node, gt) in parser.cst_iter().zip(
            [
                ("A", 0, 1),
                ("~~~ERROR~~~", 1, 2),
                ("A", 1, 2),
                ("~~~ERROR~~~", 2, 3),
                ("A", 2, 3),
                ("A", 3, 4),
                ("A", 2, 4),
                ("A", 1, 4),
                ("A", 0, 4),
                ("S", 0, 4),
                ("B", 4, 5),
                ("S", 0, 5),
            ]
            .iter(),
        ) {
            match cst_node {
                CstIterItem::Unparsed(p) => {
                    // There should be no actual unparsed data
                    assert_eq!(p, 8);
                }
                CstIterItem::Parsed(cst_node) => {
                    let r = cst_node.dotted_rule.rule;
                    let s = parser.grammar.lhs(r as usize);
                    let name = parser.grammar.nt_name(s);
                    eprintln!("{:?} / {} <=> {:?}", cst_node, name, gt);
                    assert_eq!(name, gt.0);
                    assert_eq!(cst_node.start, gt.1);
                    assert_eq!(cst_node.end, gt.2);
                }
            }
        }
    }

    /// Test terminals in the middle of a rule.
    ///
    /// S = id ws '=' ws id
    /// id = a id
    /// id = a
    /// ws = ' ' ws
    /// ws = ' '
    ///
    /// Input
    /// aa /= aa
    ///
    /// Print the parse chart at the end.
    ///
    /// Generate input for a visual representation using `dot`. Show with:
    /// `cargo test -- --test-threads 1 --nocapture | grep '^mid_term:' | cut -f2 > mid_term.dot && dot -O -Tpng mid_term.dot`
    ///
    /// The graph is in `mid_term.dot.png`.
    #[test]
    fn mid_term() {
        let mut grammar = Grammar::<char, CharMatcher>::new();
        use super::super::grammar::Rule;
        use CharMatcher::*;
        use Verdict::*;
        grammar.set_start("S".to_string());
        grammar.add(
            Rule::new("S")
                .nt("id")
                .nt("ws")
                .t(Exact('='))
                .nt("ws")
                .nt("id"),
        );
        grammar.add(Rule::new("id").t(Exact('a')).nt("id"));
        grammar.add(Rule::new("id").t(Exact('a')));
        grammar.add(Rule::new("ws").t(Exact(' ')).nt("ws"));
        grammar.add(Rule::new("ws").t(Exact(' ')));

        let compiled_grammar = grammar.compile().expect("compilation should have worked");
        let mut parser = Parser::<char, CharMatcher>::new(compiled_grammar);

        // "aa = aa" should be accepted
        for (i, (c, v)) in [
            ('a', More),
            ('a', More),
            (' ', More),
            ('=', More),
            (' ', More),
            ('a', Accept),
            ('a', Accept),
        ]
        .iter()
        .enumerate()
        {
            let res = parser.update(i, *c);
            assert_eq!(res, *v);
        }

        parser.print_chart();
        print_cst_as_dot(&parser, "mid_term_ok", true);

        // Go through the parse tree
        for (cst_node, gt) in parser.cst_iter().zip(
            [
                ("id", 0, 1),
                ("id", 1, 2),
                ("id", 0, 2),
                ("S", 0, 2),
                ("ws", 2, 3),
                ("S", 0, 3),
                ("S", 0, 4),
                ("ws", 4, 5),
                ("S", 0, 5),
                ("id", 5, 6),
                ("id", 6, 7),
                ("id", 5, 7),
                ("S", 0, 7),
            ]
            .iter(),
        ) {
            match cst_node {
                CstIterItem::Unparsed(p) => {
                    // There should be no actual unparsed data
                    assert_eq!(p, 8);
                }
                CstIterItem::Parsed(cst_node) => {
                    let r = cst_node.dotted_rule.rule;
                    let s = parser.grammar.lhs(r as usize);
                    let name = parser.grammar.nt_name(s);
                    eprintln!("{:?} / {} <=> {:?}", cst_node, name, gt);
                    assert_eq!(name, gt.0);
                    assert_eq!(cst_node.start, gt.1);
                    assert_eq!(cst_node.end, gt.2);
                }
            }
        }

        // "aa /= aa" should fail
        for (i, (c, v)) in [
            ('a', More),
            ('a', More),
            (' ', More),
            ('/', Reject),
            ('=', More),
            (' ', More),
            ('a', Accept),
            ('a', Accept),
        ]
        .iter()
        .enumerate()
        {
            let res = parser.update(i, *c);
            eprintln!("c={:?}, res={:?}", *c, res);
            assert_eq!(res, *v);
        }

        // Print chart and graph
        parser.print_chart();
        print_cst_as_dot(&parser, "mid_term", true);

        // Go through the parse tree
        for (cst_node, gt) in parser.cst_iter().zip(
            [
                ("id", 0, 1),
                ("id", 1, 2),
                ("id", 0, 2),
                ("S", 0, 2),
                ("ws", 2, 3),
                ("~~~ERROR~~~", 3, 4),
                ("ws", 3, 4),
                ("ws", 2, 4),
                ("S", 0, 4),
                ("S", 0, 5),
                ("ws", 5, 6),
                ("S", 0, 6),
                ("id", 6, 7),
                ("id", 7, 8),
                ("id", 6, 8),
                ("S", 0, 8),
            ]
            .iter(),
        ) {
            match cst_node {
                CstIterItem::Unparsed(p) => {
                    // There should be no actual unparsed data
                    assert_eq!(p, 8);
                }
                CstIterItem::Parsed(cst_node) => {
                    let r = cst_node.dotted_rule.rule;
                    let s = parser.grammar.lhs(r as usize);
                    let name = parser.grammar.nt_name(s);
                    eprintln!("{:?} / {} <=> {:?}", cst_node, name, gt);
                    assert_eq!(name, gt.0);
                    assert_eq!(cst_node.start, gt.1);
                    assert_eq!(cst_node.end, gt.2);
                }
            }
        }
    }

}
