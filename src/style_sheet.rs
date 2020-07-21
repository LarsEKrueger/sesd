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

//! Style sheet with arbitrary styles

use super::SymbolId;

pub struct StyleSheet<Style> {
    /// All style matchers
    styles: Vec<StyleMatcher<Style>>,
}

/// Simple matcher for parse tree paths
enum SymbolMatcher {
    /// Match exactly one symbol
    Exact(SymbolId),

    /// Zero or more matches of the same symbol
    Star(SymbolId),

    /// Skip over non-matching symbol, advance on match
    SkipTo(SymbolId),
}

/// A simple matcher of non-terminals, to return a style.
pub struct StyleMatcher<Style> {
    pattern: Vec<SymbolMatcher>,
    style: Style,
}

/// Result of lookup operation
#[derive(Debug)]
pub enum LookedUp<'a, Style> {
    /// Found a style for a parent path
    Parent,

    /// Found the style for this node
    Found(&'a Style),

    /// Found nothing
    Nothing,
}

impl<Style> StyleSheet<Style> {
    pub fn new() -> Self {
        Self { styles: Vec::new() }
    }

    pub fn add(&mut self, m: StyleMatcher<Style>) {
        self.styles.push(m);
    }

    /// Lookup a path in the style sheet.
    pub fn lookup(&self, path: &[SymbolId]) -> LookedUp<Style> {
        // Keep track of the still-possible matchers and respective position in the match list.
        let mut active: Vec<(usize, usize)> = (0..self.styles.len()).map(|i| (i, 0)).collect();

        // Process the symbols in the path one by one. Check for each matcher:
        // * If the current symbol does not match and the matcher is Exact, remove the matcher from
        //   the active list and continue.
        // * If the current symbol does not match and the matcher is Star, go to the next index in
        //   the matcher.
        // * If the matcher reaches the end before the path, return Parent
        // * If the matcher reaches the end together with path, return Found.
        //
        // If there are no matchers left, return Nothing.
        //
        // In case of multiple matches, use the longest one.
        let mut res = LookedUp::Nothing;
        for s in path.iter() {
            let mut i = 0;
            while i < active.len() {
                if active[i].1 >= self.styles[active[i].0].pattern.len() {
                    res = LookedUp::Parent;
                    active.remove(i);
                } else {
                    match self.styles[active[i].0].pattern[active[i].1] {
                        SymbolMatcher::Exact(sym) => {
                            if sym == *s {
                                active[i].1 += 1;
                                i += 1;
                            } else {
                                active.remove(i);
                            }
                        }
                        SymbolMatcher::Star(sym) => {
                            if sym == *s {
                                i += 1;
                            } else {
                                active[i].1 += 1;
                            }
                        }
                        SymbolMatcher::SkipTo(sym) => {
                            if sym == *s {
                                active[i].1 += 1;
                            }
                            i += 1;
                        }
                    }
                }
            }

            if active.len() == 0 {
                return res;
            }
        }
        // There is at least one active matcher left. If there is one at the end, return it as
        // found.
        debug_assert!(!active.is_empty());
        for a in active {
            if a.1 == self.styles[a.0].pattern.len() {
                return LookedUp::Found(&self.styles[a.0].style);
            }
        }

        res
    }
}

impl<Style> StyleMatcher<Style> {
    pub fn new(style: Style) -> Self {
        Self {
            pattern: Vec::new(),
            style,
        }
    }

    pub fn exact(mut self, sym: SymbolId) -> Self {
        self.pattern.push(SymbolMatcher::Exact(sym));
        self
    }

    pub fn star(mut self, sym: SymbolId) -> Self {
        self.pattern.push(SymbolMatcher::Star(sym));
        self
    }

    pub fn skip_to(mut self, sym: SymbolId) -> Self {
        self.pattern.push(SymbolMatcher::SkipTo(sym));
        self
    }
}
