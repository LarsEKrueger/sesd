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

/// Symbol IDs are indices into the symbol table. As such, the can be fairly small integers to
/// save space. 16 bit should be sufficient for all purposes. If not, file a feature request.
pub type SymbolId = u16;

/// ID of the pseudo-non-terminal to represent parsing errors
pub const ERROR_ID: SymbolId = 0;

/// Trait to access a checked and compacted representation of a grammar.
///
/// Symbols (terminals and non-terminals) are identified by small integers. For debugging and
/// queries, the names of the non-terminals are kept in a table. The matchers of terminals are kept in a
/// separate table.
///
/// A compiled grammar identifies non-terminals by their index into the symbol table. This table is
/// used for debugging and error messages. The terminals cannot be queried from the public API,
/// thus all parameters of type `SymbolId` refer to non-terminal symbols.
///
/// The following invariant for rhs of rules holds: If a symbol id < nt_count(), it is a non-terminal.
/// All other ids are terminal symbols. If a symbol id < nt_empty_count(), the symbol has an empty
/// rule. If the internal represenation of the trait implementation encodes the rules differently,
/// `rhs()` needs to convert the data accordingly.
///
/// TODO: Make finding rules of NonTerminal more efficient. Sort rules by lhs. Either keep separate table of
/// first fule index or store first rule index in rhs instead of symbol index.
pub trait CompiledGrammar<T, M>
where
    M: Matcher<T>,
{
    /// Id of the start symbol
    fn start_symbol(&self) -> SymbolId;

    /// Number of rules.
    ///
    /// Calls to `lhs` and `rhs` will always be below the returned number.
    fn rules_count(&self) -> usize;

    /// Left-hand-side symbol of a rule
    fn lhs(&self, rule: usize) -> SymbolId;

    /// Right-hand-side symbols of a rule.
    ///
    /// The following invariant for the return value holds: If a symbol id < nt_count(), it is a
    /// non-terminal. All other ids are terminal symbols. If a symbol id < nt_empty_count(), the
    /// symbol has an empty rule. If the internal represenation of the trait implementation encodes
    /// the rules differently, `rhs()` needs to convert the data accordingly.
    fn rhs(&self, rule: usize) -> &[SymbolId];

    /// Printable name of a non-terminal
    fn nt_name(&self, nt: SymbolId) -> &str;

    /// Number of non-terminal symbols
    fn nt_count(&self) -> SymbolId;

    /// Number of terminal symbols
    fn t_count(&self) -> SymbolId;

    /// Number of non-terminal symbols that have empty rules.
    fn nt_empty_count(&self) -> SymbolId;

    /// Return a matcher for a given terminal symbol. The symbol has been corrected by the number
    /// of non-terminal symbols already.
    fn matcher(&self, term: SymbolId) -> M;
}

/// Define a grammar at compile time.
///
/// The parameters are:
/// ```ignore
/// grammar!{
///   // Define the name of the mod in which the grammar is enclosed.
///   module_name,
///   // Make the Matcher variants available inside the grammar. Any definitions made in this
///   // section will not be visible outside the grammar.
///   //
///   // The braces are mandatory.
///   {
///     use any::symbols::you::need::*;
///   },
///   TokenType,
///   MatcherType,
///   START_SYMBOL,
///   [
///     NONTERMINAL_EMPTY_1,
///     NONTERMINAL_EMPTY_2
///   ],
///   [
///     NONTERMINAL_NONEMPTY_1,
///     NONTERMINAL_NONEMPTY_2,
///     START_SYMBOL
///   ],
///   [
///       TERMINAL_1 = MatcherType::Enum1( CONSTANT),
///       TERMINAL_2 = MatcherType::Enum2
///   ],
///   // List of rules. Non-terminals and matchers can be mixed freely.
///   [
///       START_SYMBOL = NONTERMINAL_EMPTY_1 TERMINAL_1 NONTERMINAL_NONEMPTY_1,
///       START_SYMBOL = NONTERMINAL_EMPTY_2 TERMINAL_2 NONTERMINAL_NONEMPTY_2
///   ]
/// }
/// ```
///
/// This will compile to the following code. Private definitions have been left out for brevity.
///
/// ```ignore
/// mod module_name {
///   use sesd::SymbolId;
///   use any::symbols::you::need::*;
///
///   pub const NONTERMINAL_EMPTY_1 : SymbolId = 1;
///   pub const NONTERMINAL_EMPTY_2 : SymbolId = 2;
///   pub const NONTERMINAL_NONEMPTY_1 : SymbolId = 3;
///   pub const NONTERMINAL_NONEMPTY_2 : SymbolId = 4;
///   pub const START_SYMBOL : SymbolId = 5;
///   pub const TERMINAL_1 : SymbolId = 6;
///   pub const TERMINAL_2 : SymbolId = 7;
///
///   pub struct Grammar {}
///
///   pub fn grammar() -> Grammar {
///       Grammar {}
///   }
///
///   impl CompiledGrammar<TokenType, MatcherType> for Grammar {
///     ...
///   }
/// }
/// ```
///
/// # Debugging hints
///
/// If you get any errors during the compilation of a grammar, this section will help you
/// pinpointing the cause.
///
/// ## `Cannot find value 'XXX' in this scope` in the rules
///
/// You forgot to declare `XXX` as a terminal or non-terminal. The compiler will indicate the
/// correct position where the undeclared symbol has been used.
///
/// Add it to the appropriate declaration list.
///
/// ## `no rules expected this token in macro call`
///
/// You made a syntax error in one of the lists. The compiler is sometimes able to indicate the
/// position of the error. Fix those errors from top to bottom. This might also resolve later
/// errors.
///
/// If the compiler indicates the error to be inside the macro, i.e. the offending line starts with
/// `grammar!{@`, go through the following checklist:
///
/// * There must be no comma after the last non-empty non-terminal. Offening line starts with `grammar!{@nt_names`.
/// * There must be no comma after the last terminal. Offening line starts with `grammar!{@term`.
///
/// ## Using the nightly toolchain
///
/// If you have the [*nightly* toolchain](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/rustup-for-managing-rust-versions.html) installed, you can run
///
/// ```bash
///    rustup run nightly cargo  rustc --lib -- -Z debug-macros=yes
/// ```
///
/// This will usually provide you with the error position directly.
#[macro_export]
macro_rules! grammar {

    // NTs with empty rules
    (@empty_nt [], $idx:expr, $nts:tt, $terms:tt, $const_num:ident) => {
        pub const $const_num : SymbolId = $idx;
        grammar!{@nt $nts, $idx, $terms}
    };

    (@empty_nt [$nt:ident], $idx:expr, $nts:tt, $terms:tt, $const_num:ident ) => {
        pub const $nt : SymbolId = $idx;
        pub const $const_num : SymbolId = $idx+1u16;
        grammar!{@nt $nts, $idx+1u16, $terms}
    };

    (@empty_nt [$nt:ident, $($rest:tt)*], $idx:expr, $nts:tt, $terms:tt, $const_num:ident ) => {
        pub const $nt : SymbolId = $idx;
        grammar!{@empty_nt [$($rest)*], $idx+1u16, $nts, $terms, $const_num}
    };

    // NTs without empty rules
    (@nt [], $idx:expr, $terms:tt ) => {
        grammar!{@term $terms, $idx}
    };

    (@nt [$nt:ident], $idx:expr, $terms:tt ) => {
        pub const $nt : SymbolId = $idx;
        grammar!{@term $terms, $idx+1u16}
    };

    (@nt [$nt:ident,$($nts:tt)*], $idx:expr, $terms:tt ) => {
        pub const $nt : SymbolId = $idx;
        grammar!{ @nt [$($nts)*], $idx+1u16, $terms}
    };

    // Terminal Ids
    (@term [$term:ident = $match:expr], $idx:expr) => {
        pub const $term : SymbolId = $idx;
    };

    (@term [$term:ident = $match:expr, $($terms:tt)*], $idx:expr) => {
        pub const $term : SymbolId = $idx;
        grammar!{@term [$($terms)*], $idx+1u16}
    };

    // NT names
    (@nte_names [], $nts:tt, $const_names:ident, $idx:expr, $names:tt) => {
        grammar!{@nt_names $nts, $const_names, $idx, $names}
    };
    (@nte_names [$nte:ident], $nts:tt, $const_names:ident, $idx:expr, [$($names:tt)*]) => {
        grammar!{@nt_names $nts, $const_names, $idx+1u16 , [$($names)*,stringify!($nte)]}
    };
    (@nte_names [$nte:ident,$($rest:tt)*], $nts:tt, $const_names:ident, $idx:expr, [$($names:tt)*]) => {
        grammar!{@nte_names [$($rest)*], $nts, $const_names, $idx+1u16, [$($names)*,stringify!($nte)]}
    };

    (@nt_names [$nt:ident], $const_names:ident, $idx:expr, [$($names:tt)*]) => {
        const $const_names : [&str;($idx+1u16) as usize] = [$($names)*,stringify!($nt)];
    };
    (@nt_names [$nt:ident,$($nts:tt)*], $const_names:ident, $idx:expr, [$($names:tt)*]) => {
        grammar!{@nt_names [$($nts)*], $const_names, $idx+1, [$($names)*,stringify!($nt)]}
    };

    (@nt_array $nte:tt, $nts:tt, $const_names:ident) => {
        grammar!{@nte_names $nte, $nts, $const_names, 1u16, ["~~~ERROR~~~"]}
    };

    // Terminal table
    (@termtab [$term:ident = $match:expr], $idx:expr, $matcher:ty, $const_terms:ident, [$($terms:tt)*]) => {
        const $const_terms: [$matcher;($idx+1u16) as usize] = [ $($terms)* $match ];
    };
    (@termtab [$term:ident = $match:expr, $($rest:tt)*], $idx:expr, $matcher:ty, $const_terms:ident, [$($terms:tt)*]) => {
        grammar!{ @termtab [$($rest)*], $idx+1u16, $matcher, $const_terms, [$($terms)* $match, ]}
    };

    // Rules
    (@rules [$lhs:ident = $($rhs:ident)*], $idx:expr, $const_rules:ident, [$($rules:tt)*]) => {
        const $const_rules: [(SymbolId, &[SymbolId]);($idx+1u16) as usize] = [$($rules)* ($lhs, &[ $($rhs),* ])];
    };
    (@rules [$lhs:ident = $($rhs:ident)*, $($rest:tt)*], $idx:expr, $const_rules:ident, [$($rules:tt)*]) => {
        grammar!{@rules [$($rest)*], $idx+1u16, $const_rules, [$($rules)* ($lhs, &[ $($rhs),* ]),]}
    };

    // Trait implementation
    (@impl $token:ty, $matcher:ty, $start:ident, $const_rules:ident, $const_names:ident, $const_terms:ident, $const_num:ident) => {
        impl $crate::CompiledGrammar<$token, $matcher> for Grammar {
            fn start_symbol(&self) -> SymbolId {
                $start
            }

            fn rules_count(&self) -> usize {
                $const_rules.len()
            }

            fn lhs(&self, rule: usize) -> SymbolId {
                $const_rules[rule].0
            }

            fn rhs(&self, rule: usize) -> &[SymbolId] {
                $const_rules[rule].1
            }

            fn nt_name(&self, nt: SymbolId) -> &str {
                $const_names[nt as usize]
            }

            fn nt_count(&self) -> SymbolId {
                $const_names.len() as SymbolId
            }

            fn t_count(&self) -> SymbolId {
                $const_terms.len() as SymbolId
            }

            fn nt_empty_count(&self) -> SymbolId {
                $const_num
            }

            fn matcher(&self, term: SymbolId) -> $matcher {
                $const_terms[term as usize].clone()
            }
        }
    };

    (@generate {$($prefix:tt)*}, $token:ty, $matcher:ty, $start:ident, $nte:tt, $nts:tt, $terms:tt, $rules:tt) => {
        use $crate::{SymbolId,ERROR_ID};
        $($prefix)*

        grammar!{@empty_nt $nte, 1u16, $nts, $terms, NUMBER_OF_EMPTY_NTS}

        grammar!{@nt_array $nte, $nts, NT_NAMES}

        grammar!{@termtab $terms, 0u16, $matcher, TERMINALS, []}

        grammar!{@rules $rules, 1u16, RULES, [(ERROR_ID, &[]),]}

        pub struct Grammar { }

        pub fn grammar() -> Grammar {
            Grammar {}
        }

        grammar!{@impl $token, $matcher, $start, RULES, NT_NAMES, TERMINALS, NUMBER_OF_EMPTY_NTS}
    };

    (pub $mod:ident, $prefix:tt, $token:ty, $matcher:ty, $start:ident, $nte:tt, $nts:tt, $terms:tt, $rules:tt) => { pub mod $mod {
        grammar!{@generate $prefix,$token,$matcher,$start,$nte,$nts,$terms,$rules}
    }};

    ( $mod:ident, $prefix:tt, $token:ty, $matcher:ty, $start:ident, $nte:tt, $nts:tt, $terms:tt, $rules:tt) => { mod $mod {
        grammar!{@generate $prefix,$token,$matcher,$start,$nte,$nts,$terms,$rules}
    }};

}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn sentence_grammar() {
        // Test grammar
        //
        // S = A B C
        // A = Range('a','z')
        // A =
        // B = Exact('b')
        // B =
        // C = Exact('c')
        grammar! {g1,
        {
            use crate::char::CharMatcher::*;
        },
        char,crate::char::CharMatcher,
        S,
        [A,B],
        [S,C],
        [
            T_A = Range('a','z'),
            T_B = Exact('b'),
            T_C = Exact('c')
        ],
        [
            S = A B C,
            A = T_A,
            B = T_B,
            C = T_C
        ]}

        let grammar = g1::grammar();
        assert_eq!(g1::A, 1);
        assert_eq!(g1::B, 2);
        assert_eq!(g1::NUMBER_OF_EMPTY_NTS, 3);
        assert_eq!(g1::S, 3);
        assert_eq!(g1::C, 4);
        assert_eq!(g1::T_A, 5);
        assert_eq!(g1::T_B, 6);
        assert_eq!(g1::T_C, 7);

        assert_eq!(grammar.nt_name(0), "~~~ERROR~~~");
        assert_eq!(grammar.nt_name(g1::A), "A");
        assert_eq!(grammar.nt_name(g1::B), "B");
        assert_eq!(grammar.nt_name(g1::C), "C");
        assert_eq!(grammar.nt_name(g1::S), "S");

        use crate::Parser;
        use crate::Verdict::*;
        let mut parser = Parser::<char, crate::char::CharMatcher, g1::Grammar>::new(grammar);
        for (i, (c, v)) in [('a', More), ('b', More), ('c', Accept)].iter().enumerate() {
            let res = parser.update(i, *c);
            parser.print_chart();
            assert_eq!(res, *v);
        }
        for (i, (c, v)) in [('c', Accept)].iter().enumerate() {
            let res = parser.update(i, *c);
            assert_eq!(res, *v);
        }
    }
}
