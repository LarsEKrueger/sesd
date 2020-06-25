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

//! Style sheet

use pancurses::Attributes;

use sesd::SymbolId;

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub attr: Attributes,
    pub line_break_before: bool,
    pub line_break_after: bool,
}

pub struct StyleSheet {
    pub default: Style,

    /// All style matchers
    styles: Vec<StyleMatcher>,
}

/// Simple matcher for parse tree paths
#[derive(Debug)]
pub enum SymbolMatcher {
    /// Match exactly one symbol
    Exact(SymbolId),

    /// Zero or more matches of the same symbol
    Star(SymbolId),
}

/// A simple matcher of non-terminals, to return a style.
#[derive(Debug)]
struct StyleMatcher {
    pattern: Vec<SymbolMatcher>,
    style: Style,
}

/// Result of lookup operation
#[derive(Debug)]
pub enum LookedUp {
    /// Found a style for a parent path
    Parent,

    /// Found the style for this node
    Found(Style),

    /// Found nothing
    Nothing,
}

impl Style {
    pub fn none() -> Self {
        Self {
            attr: Attributes::new(),
            line_break_before: false,
            line_break_after: false,
        }
    }
}

impl StyleSheet {
    pub fn new(default: Style) -> Self {
        Self {
            default,
            styles: Vec::new(),
        }
    }

    pub fn add(&mut self, pattern: Vec<SymbolMatcher>, style: Style) {
        // Convert the path to a byte array
        self.styles.push(StyleMatcher { pattern, style });
    }

    /// Lookup a path in the style sheet.
    pub fn lookup(&self, path: &[SymbolId]) -> LookedUp {
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
        trace!("lookup: {:?}", path);
        for s in path.iter() {
            trace!("  {:?}", s);
            let mut i = 0;
            let mut found = false;
            while i < active.len() {
                trace!("  {}, {:?}, {:?}", i, active[i], self.styles[active[i].0]);
                if active[i].1 >= self.styles[active[i].0].pattern.len() {
                    return LookedUp::Parent;
                }
                match self.styles[active[i].0].pattern[active[i].1] {
                    SymbolMatcher::Exact(sym) => {
                        if sym == *s {
                            active[i].1 += 1;
                            found = true;
                            i += 1;
                        } else {
                            active.remove(i);
                        }
                    }
                    SymbolMatcher::Star(sym) => {
                        if sym == *s {
                            found = true;
                            i += 1;
                        } else {
                            active[i].1 += 1;
                        }
                    }
                }
            }

            if active.len() == 0 || !found {
                return LookedUp::Nothing;
            }
        }
        // There is at least one active matcher left. If there is one at the end, return it as
        // found.
        debug_assert!(!active.is_empty());
        for a in active {
            if a.1 == self.styles[a.0].pattern.len() {
                return LookedUp::Found(self.styles[a.0].style);
            }
        }

        LookedUp::Nothing
    }
}
