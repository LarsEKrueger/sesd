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

use super::grammar::{CompiledGrammar, SymbolId};

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
    /// Outer dimension corresponds to buffer index. Inner dimensions are the possible rules that
    /// apply at this buffer index.
    chart: Vec<Vec<(DottedRule, StartDot)>>,
}

impl<T> Parser<T> {
    pub fn new(grammar: CompiledGrammar<T>) -> Self {
        Self {
            grammar,
            chart: Vec::new(),
        }
    }
}
