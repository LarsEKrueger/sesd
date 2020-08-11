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

//! Style sheet and predictions for a language.

use std::collections::HashMap;

use pancurses::Attributes;

use sesd::style_sheet::StyleSheet;
use sesd::SymbolId;

/// Style of a syntactic element.
#[derive(Debug)]
pub struct Style {
    /// pancurses Attributes to render the element
    pub attr: Attributes,
    /// Shall the renderer break the line before the element
    pub line_break_before: bool,
    /// Shall the renderer break the line after the element
    pub line_break_after: bool,
}

/// Look and Feel of a language
pub struct LookAndFeel {
    /// Everything not matched by the style sheet will be rendered in this style
    pub default: Style,

    /// All style matchers and the correspondig styles
    style_sheet: StyleSheet<Style>,

    /// List of predictions for a given symbol
    predictions: HashMap<SymbolId, Vec<String>>,
}

/// Re-export the style matcher for brevity
pub type StyleMatcher = sesd::style_sheet::StyleMatcher<Style>;
/// Re-export the style look up result for brevity
pub type LookedUp<'a> = sesd::style_sheet::LookedUp<'a, Style>;

/// Style Builder
pub struct StyleBuilder {
    pub s: Style,
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

impl LookAndFeel {
    /// Create a new look and feel
    pub fn new(default: Style) -> Self {
        Self {
            default,
            style_sheet: StyleSheet::new(),
            predictions: HashMap::new(),
        }
    }

    /// Add a style matcher
    pub fn add_style(&mut self, m: StyleMatcher) {
        self.style_sheet.add(m);
    }

    /// Lookup a path in the style sheet.
    pub fn lookup(&self, path: &[SymbolId]) -> LookedUp {
        self.style_sheet.lookup(path)
    }

    /// Add a prediction to the look and feel
    pub fn add_prediction(&mut self, sym: SymbolId, pred: &[&str]) {
        let preds = pred.iter().map(|s| s.to_string()).collect();
        self.predictions.insert(sym, preds);
    }

    /// Find the predictions for this symbol
    pub fn predictions(&self, sym: SymbolId) -> Vec<String> {
        self.predictions
            .get(&sym)
            .iter()
            .flat_map(|p| p.iter())
            .map(|s| s.clone())
            .collect()
    }
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self { s: Style::none() }
    }

    pub fn b(mut self) -> Self {
        self.s.attr.set_bold(true);
        self
    }

    pub fn i(mut self) -> Self {
        self.s.attr.set_italic(true);
        self
    }

    pub fn u(mut self) -> Self {
        self.s.attr.set_underline(true);
        self
    }

    pub fn cp(mut self, c: pancurses::ColorPair) -> Self {
        self.s.attr.set_color_pair(c);
        self
    }
}
