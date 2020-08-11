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

//! Compiled-in data for Cargo.toml files
//!
//! This is based on https://github.com/toml-lang/toml/blob/master/toml.abnf, which is
//! MIT licensed.

use sesd::SymbolId;

use super::look_and_feel::{LookAndFeel, Style, StyleMatcher};

mod cargo_toml {
    use super::SymbolId;

    // Non-terminals with empty rules
    const NON_EOLS: SymbolId = 1;
    const MAYBE_COMMENT: SymbolId = 2;
    const MAYBE_INLINE_TABLE_KEYVALS: SymbolId = 3;
    const MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS: SymbolId = 4;
    const MAYBE_ARRAY_VALUES: SymbolId = 5;
    const MAYBE_ARRAY_SEP: SymbolId = 6;
    const WS_COMMENT_NEWLINE: SymbolId = 7;
    const MAYBE_TIME_SECFRAC: SymbolId = 8;
    const SIGN: SymbolId = 9;
    const HEX_INT_REST: SymbolId = 10;
    const OCT_INT_REST: SymbolId = 11;
    const BIN_INT_REST: SymbolId = 12;
    const MAYBE_EXP: SymbolId = 13;
    const ZERO_PREFIXABLE_INT_REST: SymbolId = 14;
    const BASIC_CHARS: SymbolId = 15;
    const MAYBE_MLB_QUOTES: SymbolId = 16;
    const MLB_QUOTES_CONTENT: SymbolId = 17;
    const MAYBE_MLB_CONTENT: SymbolId = 18;
    const WSCHAR_NLS: SymbolId = 19;
    const MAYBE_LITERAL_CHAR: SymbolId = 20;
    const MAYBE_MLL_CONTENT: SymbolId = 21;
    const MAYBE_MLL_QUOTES: SymbolId = 22;
    const SOME_MLL_QUOTES_CONTENT: SymbolId = 23;
    const WS: SymbolId = 24;

    const NUMBER_OF_EMPTY_NTS: SymbolId = 25;

    // Other non-terminals
    const DOTTED_KEY_REST: SymbolId = 25;
    const DOTTED_KEY: SymbolId = 26;
    const DOT_SEP: SymbolId = 27;
    const KEY: SymbolId = 28;
    const ONE_STAR_TWO_QUOTATION_MARK: SymbolId = 29;
    const ONE_STAR_DIGIT: SymbolId = 30;
    const ONE_MAYBE_MLB_CONTENT: SymbolId = 31;
    const ONE_MAYBE_MLL_CONTENT: SymbolId = 32;
    const TWO_DIGIT: SymbolId = 33;
    const FOUR_DIGIT: SymbolId = 34;
    const THREE_APOSTROPHE: SymbolId = 35;
    const FOUR_HEXDIG: SymbolId = 36;
    const EIGHT_HEXDIG: SymbolId = 37;
    const ALPHA: SymbolId = 38;
    const APOSTROPHE: SymbolId = 39;
    const ARRAY_CLOSE: SymbolId = 40;
    const ARRAY_OPEN: SymbolId = 41;
    const ARRAY_SEP: SymbolId = 42;
    const ARRAY_TABLE_CLOSE: SymbolId = 43;
    const ARRAY_TABLE_OPEN: SymbolId = 44;
    const ARRAY_TABLE: SymbolId = 45;
    const ARRAY_VALUES: SymbolId = 46;
    const ARRAY: SymbolId = 47;
    const BASIC_CHAR: SymbolId = 48;
    const BASIC_STRING: SymbolId = 49;
    const BASIC_UNESCAPED: SymbolId = 50;
    const BIN_INT: SymbolId = 51;
    const BIN_PREFIX: SymbolId = 52;
    const BOOLEAN: SymbolId = 53;
    const COMMENT_START_SYMBOL: SymbolId = 54;
    const COMMENT: SymbolId = 55;
    const DATE_FULLYEAR: SymbolId = 56;
    const DATE_MDAY: SymbolId = 57;
    const DATE_MONTH: SymbolId = 58;
    const DATE_TIME: SymbolId = 59;
    const DEC_INT: SymbolId = 60;
    const DECIMAL_POINT: SymbolId = 61;
    const DIGIT: SymbolId = 62;
    const DIGIT_: SymbolId = 63;
    const DIGITZERO_ONE_: SymbolId = 64;
    const DIGITZERO_ONE_UNDERSCORE: SymbolId = 65;
    const DIGITZERO_SEVEN_: SymbolId = 66;
    const DIGITZERO_SEVEN_UNDERSCORE: SymbolId = 67;
    const DIGITONE_NINE_: SymbolId = 68;
    const ESCAPE_SEQ_CHAR: SymbolId = 69;
    const ESCAPE: SymbolId = 70;
    const ESCAPED: SymbolId = 71;
    const EXP: SymbolId = 72;
    const EXPRESSION: SymbolId = 73;
    const EXPRESSIONS: SymbolId = 74;
    const SYM_FALSE: SymbolId = 75;
    const FLOAT_EXP_PART: SymbolId = 76;
    const FLOAT_INT_PART: SymbolId = 77;
    const FLOAT: SymbolId = 78;
    const FLOAT_REST: SymbolId = 79;
    const FRAC: SymbolId = 80;
    const FULL_DATE: SymbolId = 81;
    const FULL_TIME: SymbolId = 82;
    const HEX_INT: SymbolId = 83;
    const HEX_PREFIX: SymbolId = 84;
    const HEXDIG: SymbolId = 85;
    const HEXDIG_: SymbolId = 86;
    const INF: SymbolId = 87;
    const INLINE_TABLE_CLOSE: SymbolId = 88;
    const INLINE_TABLE_KEYVALS: SymbolId = 89;
    const INLINE_TABLE_OPEN: SymbolId = 90;
    const INLINE_TABLE_SEP: SymbolId = 91;
    const INLINE_TABLE: SymbolId = 92;
    const INTEGER: SymbolId = 93;
    const KEYVAL_SEP: SymbolId = 94;
    const KEYVAL: SymbolId = 95;
    const LITERAL_CHAR: SymbolId = 96;
    const LITERAL_STRING: SymbolId = 97;
    const LOCAL_DATE_TIME: SymbolId = 98;
    const LOCAL_DATE: SymbolId = 99;
    const LOCAL_TIME: SymbolId = 100;
    const MINUS: SymbolId = 101;
    const ML_BASIC_BODY: SymbolId = 102;
    const ML_BASIC_STRING_DELIM: SymbolId = 103;
    const ML_BASIC_STRING: SymbolId = 104;
    const ML_LITERAL_BODY: SymbolId = 105;
    const ML_LITERAL_STRING_DELIM: SymbolId = 106;
    const ML_LITERAL_STRING: SymbolId = 107;
    const MLB_CHAR: SymbolId = 108;
    const MLB_CONTENT: SymbolId = 109;
    const MLB_ESCAPED_NL: SymbolId = 110;
    const MLB_QUOTES: SymbolId = 111;
    const MLB_UNESCAPED: SymbolId = 112;
    const MLL_CHAR: SymbolId = 113;
    const MLL_CONTENT: SymbolId = 114;
    const MLL_QUOTES: SymbolId = 115;
    const NAN: SymbolId = 116;
    const NEWLINE: SymbolId = 117;
    const NON_ASCII: SymbolId = 118;
    const NON_EOL: SymbolId = 119;
    const OCT_INT: SymbolId = 120;
    const OCT_PREFIX: SymbolId = 121;
    const OFFSET_DATE_TIME: SymbolId = 122;
    const PARTIAL_TIME: SymbolId = 123;
    const PLUS: SymbolId = 124;
    const QUOTATION_MARK: SymbolId = 125;
    const QUOTED_KEY: SymbolId = 126;
    const SIMPLE_KEY: SymbolId = 127;
    const SPECIAL_FLOAT: SymbolId = 128;
    const STD_TABLE_CLOSE: SymbolId = 129;
    const STD_TABLE_OPEN: SymbolId = 130;
    const STD_TABLE: SymbolId = 131;
    const STRING: SymbolId = 132;
    const TABLE: SymbolId = 133;
    const TIME_DELIM: SymbolId = 134;
    const TIME_HOUR: SymbolId = 135;
    const TIME_MINUTE: SymbolId = 136;
    const TIME_NUMOFFSET: SymbolId = 137;
    const TIME_OFFSET: SymbolId = 138;
    const TIME_SECFRAC: SymbolId = 139;
    const TIME_SECOND: SymbolId = 140;
    const TOML: SymbolId = 141;
    const SYM_TRUE: SymbolId = 142;
    const UNDERSCORE: SymbolId = 143;
    const UNQUOTED_KEY_CHAR: SymbolId = 144;
    const UNQUOTED_KEY: SymbolId = 145;
    const UNS_DEC_INT_REST: SymbolId = 146;
    const UNSIGNED_DEC_INT: SymbolId = 147;
    const VAL: SymbolId = 148;
    const WSCHAR_NL: SymbolId = 149;
    const WSCHAR: SymbolId = 150;
    const WSCN: SymbolId = 151;
    const ZERO_PREFIXABLE_INT: SymbolId = 152;

    // Terminal symbols
    const T_MINUS: SymbolId = 153; //    Exact('-')
    const T_SPACE: SymbolId = 154; //    Exact(' ')
    const T_BANG: SymbolId = 155; //    Exact('!')
    const T_DQUOT: SymbolId = 156; //    Exact('"')
    const T_HASH: SymbolId = 157; //    Exact('#')
    const T_COMMA: SymbolId = 158; //    Exact(',')
    const T_DOT: SymbolId = 159; //    Exact('.')
    const T_COLON: SymbolId = 160; //    Exact(':')
    const T_SQ_OPEN: SymbolId = 161; //    Exact('[')
    const T_TICK: SymbolId = 162; //    Exact('\'')
    const T_BACKSLASH: SymbolId = 163; //    Exact('\\')
    const T_TAB: SymbolId = 164; //    Exact('\t')
    const T_NL: SymbolId = 165; //    Exact('\x0A')
    const T_CR: SymbolId = 166; //    Exact('\x0D')
    const T_22: SymbolId = 167; //    Exact('\x22')
    const T_2D: SymbolId = 168; //    Exact('\x2D')
    const T_55: SymbolId = 169; //    Exact('\x55')
    const T_5C: SymbolId = 170; //    Exact('\x5C')
    const T_5F: SymbolId = 171; //    Exact('\x5F')
    const T_62: SymbolId = 172; //    Exact('\x62')
    const T_66: SymbolId = 173; //    Exact('\x66')
    const T_6E: SymbolId = 174; //    Exact('\x6E')
    const T_72: SymbolId = 175; //    Exact('\x72')
    const T_74: SymbolId = 176; //    Exact('\x74')
    const T_75: SymbolId = 177; //    Exact('\x75')
    const T_SQ_CLOSE: SymbolId = 178; //    Exact(']')
    const T_UNDERSCORE: SymbolId = 179; //    Exact('_')
    const T_CURLY_OPEN: SymbolId = 180; //    Exact('{')
    const T_CURLY_CLOSE: SymbolId = 181; //    Exact('}')
    const T_PLUS: SymbolId = 182; //    Exact('+')
    const T_EQUAL: SymbolId = 183; //    Exact('=')
    const T_ZERO: SymbolId = 184; //    Exact('0')
    const T_A: SymbolId = 185; //    Exact('a')
    const T_B: SymbolId = 186; //    Exact('b')
    const T_E: SymbolId = 187; //    Exact('e')
    const T_F: SymbolId = 188; //    Exact('f')
    const T_I: SymbolId = 189; //    Exact('i')
    const T_L: SymbolId = 190; //    Exact('l')
    const T_N: SymbolId = 191; //    Exact('n')
    const T_O: SymbolId = 192; //    Exact('o')
    const T_R: SymbolId = 193; //    Exact('r')
    const T_S: SymbolId = 194; //    Exact('s')
    const T_LC_T: SymbolId = 195; //    Exact('t')
    const T_UC_T: SymbolId = 196; //    Exact('T')
    const T_U: SymbolId = 197; //    Exact('u')
    const T_X: SymbolId = 198; //    Exact('x')
    const T_LC_Z: SymbolId = 199; //    Exact('z')
    const T_UC_Z: SymbolId = 200; //    Exact('Z')
    const T_80_D7FF: SymbolId = 201; //    Range('\u{80}', '\u{D7FF}')
    const T_E000_10FFFF: SymbolId = 202; //    Range('\u{E000}', '\u{10FFFF}')
    const T_20_26: SymbolId = 203; //    Range('\x20', '\x26')
    const T_20_7F: SymbolId = 204; //    Range('\x20', '\x7F')
    const T_23_5B: SymbolId = 205; //    Range('\x23', '\x5B')
    const T_28_7E: SymbolId = 206; //    Range('\x28', '\x7E')
    const T_5D_7E: SymbolId = 207; //    Range('\x5D', '\x7E')
    const T_0_1: SymbolId = 208; //    Range('0', '1')
    const T_0_7: SymbolId = 209; //    Range('0', '7')
    const T_0_9: SymbolId = 210; //    Range('0', '9')
    const T_1_9: SymbolId = 211; //    Range('1', '9')
    const T_LC_A_F: SymbolId = 212; //    Range('a', 'f')
    const T_UC_A_F: SymbolId = 213; //    Range('A', 'F')
    const T_UC_A_Z: SymbolId = 214; //    Range('A', 'Z')
    const T_LC_A_Z: SymbolId = 215; //    Range('a', 'z')

    const NT_NAMES: [&str; 153] = [
        "~~~ERROR~~~",
        "NON_EOLS",
        "MAYBE_COMMENT",
        "MAYBE_INLINE_TABLE_KEYVALS",
        "MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS",
        "MAYBE_ARRAY_VALUES",
        "MAYBE_ARRAY_SEP",
        "WS_COMMENT_NEWLINE",
        "MAYBE_TIME_SECFRAC",
        "SIGN",
        "HEX_INT_REST",
        "OCT_INT_REST",
        "BIN_INT_REST",
        "MAYBE_EXP",
        "ZERO_PREFIXABLE_INT_REST",
        "BASIC_CHARS",
        "MAYBE_MLB_QUOTES",
        "MLB_QUOTES_CONTENT",
        "MAYBE_MLB_CONTENT",
        "WSCHAR_NLS",
        "MAYBE_LITERAL_CHAR",
        "MAYBE_MLL_CONTENT",
        "MAYBE_MLL_QUOTES",
        "SOME_MLL_QUOTES_CONTENT",
        "WS",
        "DOTTED_KEY_REST",
        "DOTTED_KEY",
        "DOT_SEP",
        "KEY",
        "ONE_STAR_TWO_QUOTATION_MARK",
        "ONE_STAR_DIGIT",
        "ONE_MAYBE_MLB_CONTENT",
        "ONE_MAYBE_MLL_CONTENT",
        "TWO_DIGIT",
        "FOUR_DIGIT",
        "THREE_APOSTROPHE",
        "FOUR_HEXDIG",
        "EIGHT_HEXDIG",
        "ALPHA",
        "APOSTROPHE",
        "ARRAY_CLOSE",
        "ARRAY_OPEN",
        "ARRAY_SEP",
        "ARRAY_TABLE_CLOSE",
        "ARRAY_TABLE_OPEN",
        "ARRAY_TABLE",
        "ARRAY_VALUES",
        "ARRAY",
        "BASIC_CHAR",
        "BASIC_STRING",
        "BASIC_UNESCAPED",
        "BIN_INT",
        "BIN_PREFIX",
        "BOOLEAN",
        "COMMENT_START_SYMBOL",
        "COMMENT",
        "DATE_FULLYEAR",
        "DATE_MDAY",
        "DATE_MONTH",
        "DATE_TIME",
        "DEC_INT",
        "DECIMAL_POINT",
        "DIGIT",
        "DIGIT_",
        "DIGITZERO_ONE_",
        "DIGITZERO_ONE_UNDERSCORE",
        "DIGITZERO_SEVEN_",
        "DIGITZERO_SEVEN_UNDERSCORE",
        "DIGITONE_NINE_",
        "ESCAPE_SEQ_CHAR",
        "ESCAPE",
        "ESCAPED",
        "EXP",
        "EXPRESSION",
        "EXPRESSIONS",
        "SYM_FALSE",
        "FLOAT_EXP_PART",
        "FLOAT_INT_PART",
        "FLOAT",
        "FLOAT_REST",
        "FRAC",
        "FULL_DATE",
        "FULL_TIME",
        "HEX_INT",
        "HEX_PREFIX",
        "HEXDIG",
        "HEXDIG_",
        "INF",
        "INLINE_TABLE_CLOSE",
        "INLINE_TABLE_KEYVALS",
        "INLINE_TABLE_OPEN",
        "INLINE_TABLE_SEP",
        "INLINE_TABLE",
        "INTEGER",
        "KEYVAL_SEP",
        "KEYVAL",
        "LITERAL_CHAR",
        "LITERAL_STRING",
        "LOCAL_DATE_TIME",
        "LOCAL_DATE",
        "LOCAL_TIME",
        "MINUS",
        "ML_BASIC_BODY",
        "ML_BASIC_STRING_DELIM",
        "ML_BASIC_STRING",
        "ML_LITERAL_BODY",
        "ML_LITERAL_STRING_DELIM",
        "ML_LITERAL_STRING",
        "MLB_CHAR",
        "MLB_CONTENT",
        "MLB_ESCAPED_NL",
        "MLB_QUOTES",
        "MLB_UNESCAPED",
        "MLL_CHAR",
        "MLL_CONTENT",
        "MLL_QUOTES",
        "NAN",
        "NEWLINE",
        "NON_ASCII",
        "NON_EOL",
        "OCT_INT",
        "OCT_PREFIX",
        "OFFSET_DATE_TIME",
        "PARTIAL_TIME",
        "PLUS",
        "QUOTATION_MARK",
        "QUOTED_KEY",
        "SIMPLE_KEY",
        "SPECIAL_FLOAT",
        "STD_TABLE_CLOSE",
        "STD_TABLE_OPEN",
        "STD_TABLE",
        "STRING",
        "TABLE",
        "TIME_DELIM",
        "TIME_HOUR",
        "TIME_MINUTE",
        "TIME_NUMOFFSET",
        "TIME_OFFSET",
        "TIME_SECFRAC",
        "TIME_SECOND",
        "TOML",
        "SYM_TRUE",
        "UNDERSCORE",
        "UNQUOTED_KEY_CHAR",
        "UNQUOTED_KEY",
        "UNS_DEC_INT_REST",
        "UNSIGNED_DEC_INT",
        "VAL",
        "WSCHAR_NL",
        "WSCHAR",
        "WSCN",
        "ZERO_PREFIXABLE_INT",
    ];

    use sesd::char::CharMatcher::*;
    const TERMINALS: [sesd::char::CharMatcher; 63] = [
        Exact('-'),
        Exact(' '),
        Exact('!'),
        Exact('"'),
        Exact('#'),
        Exact(','),
        Exact('.'),
        Exact(':'),
        Exact('['),
        Exact('\''),
        Exact('\\'),
        Exact('\t'),
        Exact('\x0A'),
        Exact('\x0D'),
        Exact('\x22'),
        Exact('\x2D'),
        Exact('\x55'),
        Exact('\x5C'),
        Exact('\x5F'),
        Exact('\x62'),
        Exact('\x66'),
        Exact('\x6E'),
        Exact('\x72'),
        Exact('\x74'),
        Exact('\x75'),
        Exact(']'),
        Exact('_'),
        Exact('{'),
        Exact('}'),
        Exact('+'),
        Exact('='),
        Exact('0'),
        Exact('a'),
        Exact('b'),
        Exact('e'),
        Exact('f'),
        Exact('i'),
        Exact('l'),
        Exact('n'),
        Exact('o'),
        Exact('r'),
        Exact('s'),
        Exact('t'),
        Exact('T'),
        Exact('u'),
        Exact('x'),
        Exact('z'),
        Exact('Z'),
        Range('\u{80}', '\u{D7FF}'),
        Range('\u{E000}', '\u{10FFFF}'),
        Range('\x20', '\x26'),
        Range('\x20', '\x7F'),
        Range('\x23', '\x5B'),
        Range('\x28', '\x7E'),
        Range('\x5D', '\x7E'),
        Range('0', '1'),
        Range('0', '7'),
        Range('0', '9'),
        Range('1', '9'),
        Range('a', 'f'),
        Range('A', 'F'),
        Range('A', 'Z'),
        Range('a', 'z'),
    ];

    const RULES: [(SymbolId, &[SymbolId]); 239] = [
        (ALPHA, &[T_UC_A_Z]),
        (ALPHA, &[T_LC_A_Z]),
        (DIGIT, &[T_0_9]),
        (HEXDIG, &[DIGIT]),
        (HEXDIG, &[T_UC_A_F]),
        (HEXDIG, &[T_LC_A_F]),
        (FOUR_HEXDIG, &[HEXDIG, HEXDIG, HEXDIG, HEXDIG]),
        (EIGHT_HEXDIG, &[FOUR_HEXDIG, FOUR_HEXDIG]),
        (WS, &[WSCHAR, WS]),
        (WSCHAR, &[T_SPACE]),
        (WSCHAR, &[T_TAB]),
        (NEWLINE, &[T_NL]),
        (NEWLINE, &[T_CR, T_NL]),
        (COMMENT_START_SYMBOL, &[T_HASH]),
        (NON_ASCII, &[T_80_D7FF]),
        (NON_ASCII, &[T_E000_10FFFF]),
        (NON_EOL, &[T_TAB]),
        (NON_EOL, &[T_20_7F]),
        (NON_EOL, &[NON_ASCII]),
        (COMMENT, &[COMMENT_START_SYMBOL, NON_EOLS]),
        (NON_EOLS, &[NON_EOL, NON_EOLS]),
        (MAYBE_COMMENT, &[COMMENT]),
        (TABLE, &[STD_TABLE]),
        (TABLE, &[ARRAY_TABLE]),
        (STD_TABLE, &[STD_TABLE_OPEN, KEY, STD_TABLE_CLOSE]),
        (STD_TABLE_OPEN, &[T_SQ_OPEN, WS]),
        (STD_TABLE_CLOSE, &[WS, T_SQ_CLOSE]),
        (
            INLINE_TABLE,
            &[
                INLINE_TABLE_OPEN,
                MAYBE_INLINE_TABLE_KEYVALS,
                INLINE_TABLE_CLOSE,
            ],
        ),
        (INLINE_TABLE_OPEN, &[T_CURLY_OPEN, WS]),
        (INLINE_TABLE_CLOSE, &[WS, T_CURLY_CLOSE]),
        (INLINE_TABLE_SEP, &[WS, T_COMMA, WS]),
        (MAYBE_INLINE_TABLE_KEYVALS, &[INLINE_TABLE_KEYVALS]),
        (
            INLINE_TABLE_KEYVALS,
            &[KEYVAL, MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS],
        ),
        (
            MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS,
            &[INLINE_TABLE_SEP, INLINE_TABLE_KEYVALS],
        ),
        (ARRAY_TABLE, &[ARRAY_TABLE_OPEN, KEY, ARRAY_TABLE_CLOSE]),
        (ARRAY_TABLE_OPEN, &[T_SQ_OPEN, T_SQ_OPEN, WS]),
        (ARRAY_TABLE_CLOSE, &[WS, T_SQ_CLOSE, T_SQ_CLOSE]),
        (
            ARRAY,
            &[
                ARRAY_OPEN,
                MAYBE_ARRAY_VALUES,
                WS_COMMENT_NEWLINE,
                ARRAY_CLOSE,
            ],
        ),
        (MAYBE_ARRAY_VALUES, &[ARRAY_VALUES]),
        (ARRAY_OPEN, &[T_SQ_OPEN]),
        (ARRAY_CLOSE, &[T_SQ_CLOSE]),
        (
            ARRAY_VALUES,
            &[WS_COMMENT_NEWLINE, VAL, WS, ARRAY_SEP, ARRAY_VALUES],
        ),
        (
            ARRAY_VALUES,
            &[WS_COMMENT_NEWLINE, VAL, WS, MAYBE_ARRAY_SEP],
        ),
        (MAYBE_ARRAY_SEP, &[ARRAY_SEP]),
        (ARRAY_SEP, &[T_COMMA]),
        (WS_COMMENT_NEWLINE, &[WSCN, WS_COMMENT_NEWLINE]),
        (WSCN, &[WSCHAR]),
        (WSCN, &[MAYBE_COMMENT, NEWLINE]),
        (MAYBE_COMMENT, &[COMMENT]),
        (DATE_TIME, &[OFFSET_DATE_TIME]),
        (DATE_TIME, &[LOCAL_DATE_TIME]),
        (DATE_TIME, &[LOCAL_DATE]),
        (DATE_TIME, &[LOCAL_TIME]),
        (DATE_FULLYEAR, &[FOUR_DIGIT]),
        (FOUR_DIGIT, &[TWO_DIGIT, TWO_DIGIT]),
        (TWO_DIGIT, &[DIGIT, DIGIT]),
        (DATE_MONTH, &[TWO_DIGIT]),
        (DATE_MDAY, &[TWO_DIGIT]),
        (TIME_DELIM, &[T_UC_T]),
        (TIME_DELIM, &[T_LC_T]),
        (TIME_DELIM, &[T_SPACE]),
        (TIME_HOUR, &[TWO_DIGIT]),
        (TIME_MINUTE, &[TWO_DIGIT]),
        (TIME_SECOND, &[TWO_DIGIT]),
        (TIME_SECFRAC, &[T_COMMA, ONE_STAR_DIGIT]),
        (ONE_STAR_DIGIT, &[DIGIT, ONE_STAR_DIGIT]),
        (ONE_STAR_DIGIT, &[DIGIT]),
        (TIME_NUMOFFSET, &[SIGN, TIME_HOUR, T_COLON, TIME_MINUTE]),
        (TIME_OFFSET, &[T_UC_Z]),
        (TIME_OFFSET, &[T_LC_Z]),
        (TIME_OFFSET, &[TIME_NUMOFFSET]),
        (
            PARTIAL_TIME,
            &[
                TIME_HOUR,
                T_COLON,
                TIME_MINUTE,
                T_COLON,
                TIME_SECOND,
                MAYBE_TIME_SECFRAC,
            ],
        ),
        (MAYBE_TIME_SECFRAC, &[TIME_SECFRAC]),
        (
            FULL_DATE,
            &[
                DATE_FULLYEAR,
                T_UNDERSCORE,
                DATE_MONTH,
                T_UNDERSCORE,
                DATE_MDAY,
            ],
        ),
        (FULL_TIME, &[PARTIAL_TIME, TIME_OFFSET]),
        (OFFSET_DATE_TIME, &[FULL_DATE, TIME_DELIM, FULL_TIME]),
        (LOCAL_DATE_TIME, &[FULL_DATE, TIME_DELIM, PARTIAL_TIME]),
        (LOCAL_DATE, &[FULL_DATE]),
        (LOCAL_TIME, &[PARTIAL_TIME]),
        (INTEGER, &[DEC_INT]),
        (INTEGER, &[HEX_INT]),
        (INTEGER, &[OCT_INT]),
        (INTEGER, &[BIN_INT]),
        (MINUS, &[T_MINUS]),
        (PLUS, &[T_PLUS]),
        (UNDERSCORE, &[T_UNDERSCORE]),
        (DIGITONE_NINE_, &[T_1_9]),
        (DIGITZERO_SEVEN_, &[T_0_7]),
        (DIGITZERO_ONE_, &[T_0_1]),
        (HEX_PREFIX, &[T_ZERO, T_X]),
        (OCT_PREFIX, &[T_ZERO, T_O]),
        (BIN_PREFIX, &[T_ZERO, T_B]),
        (DEC_INT, &[SIGN, UNSIGNED_DEC_INT]),
        (SIGN, &[MINUS]),
        (SIGN, &[PLUS]),
        (UNSIGNED_DEC_INT, &[DIGIT]),
        (UNSIGNED_DEC_INT, &[DIGITONE_NINE_, UNS_DEC_INT_REST]),
        (UNS_DEC_INT_REST, &[DIGIT_, UNS_DEC_INT_REST]),
        (UNS_DEC_INT_REST, &[DIGIT_]),
        (DIGIT_, &[DIGIT]),
        (DIGIT_, &[UNDERSCORE, DIGIT]),
        (HEX_INT, &[HEX_PREFIX, HEXDIG, HEX_INT_REST]),
        (HEX_INT_REST, &[HEXDIG_, HEX_INT_REST]),
        (HEXDIG_, &[HEXDIG]),
        (HEXDIG_, &[UNDERSCORE, HEXDIG]),
        (OCT_INT, &[OCT_PREFIX, DIGITZERO_SEVEN_, OCT_INT_REST]),
        (OCT_INT_REST, &[DIGITZERO_SEVEN_, OCT_INT_REST]),
        (DIGITZERO_SEVEN_UNDERSCORE, &[DIGITZERO_SEVEN_]),
        (DIGITZERO_SEVEN_UNDERSCORE, &[UNDERSCORE, DIGITZERO_SEVEN_]),
        (BIN_INT, &[BIN_PREFIX, DIGITZERO_ONE_, BIN_INT_REST]),
        (BIN_INT_REST, &[DIGITZERO_ONE_UNDERSCORE, BIN_INT_REST]),
        (DIGITZERO_ONE_UNDERSCORE, &[DIGITZERO_ONE_]),
        (DIGITZERO_ONE_UNDERSCORE, &[UNDERSCORE, DIGITZERO_ONE_]),
        (FLOAT, &[FLOAT_INT_PART, FLOAT_REST]),
        (FLOAT, &[SPECIAL_FLOAT]),
        (FLOAT_REST, &[EXP]),
        (FLOAT_REST, &[FRAC, MAYBE_EXP]),
        (MAYBE_EXP, &[EXP]),
        (FLOAT_INT_PART, &[DEC_INT]),
        (FRAC, &[DECIMAL_POINT, ZERO_PREFIXABLE_INT]),
        (DECIMAL_POINT, &[T_DOT]),
        (ZERO_PREFIXABLE_INT, &[DIGIT, ZERO_PREFIXABLE_INT_REST]),
        (
            ZERO_PREFIXABLE_INT_REST,
            &[DIGIT_, ZERO_PREFIXABLE_INT_REST],
        ),
        (EXP, &[T_E, FLOAT_EXP_PART]),
        (FLOAT_EXP_PART, &[SIGN, ZERO_PREFIXABLE_INT]),
        (SPECIAL_FLOAT, &[SIGN, INF]),
        (SPECIAL_FLOAT, &[SIGN, NAN]),
        (INF, &[T_I, T_N, T_F]),
        (NAN, &[T_N, T_A, T_N]),
        (BOOLEAN, &[SYM_TRUE]),
        (BOOLEAN, &[SYM_FALSE]),
        (SYM_TRUE, &[T_LC_T, T_R, T_U, T_E]),
        (SYM_FALSE, &[T_F, T_A, T_L, T_S, T_E]),
        (STRING, &[ML_BASIC_STRING]),
        (STRING, &[BASIC_STRING]),
        (STRING, &[ML_LITERAL_STRING]),
        (STRING, &[LITERAL_STRING]),
        (BASIC_STRING, &[QUOTATION_MARK, BASIC_CHARS, QUOTATION_MARK]),
        (BASIC_CHARS, &[BASIC_CHAR, BASIC_CHARS]),
        (QUOTATION_MARK, &[T_DQUOT]),
        (BASIC_CHAR, &[BASIC_UNESCAPED]),
        (BASIC_CHAR, &[ESCAPED]),
        (BASIC_UNESCAPED, &[WSCHAR]),
        (BASIC_UNESCAPED, &[T_BANG]),
        (BASIC_UNESCAPED, &[T_23_5B]),
        (BASIC_UNESCAPED, &[T_5D_7E]),
        (BASIC_UNESCAPED, &[NON_ASCII]),
        (ESCAPED, &[ESCAPE, ESCAPE_SEQ_CHAR]),
        (ESCAPE, &[T_BACKSLASH]),
        (ESCAPE_SEQ_CHAR, &[T_22]),
        (ESCAPE_SEQ_CHAR, &[T_5C]),
        (ESCAPE_SEQ_CHAR, &[T_62]),
        (ESCAPE_SEQ_CHAR, &[T_66]),
        (ESCAPE_SEQ_CHAR, &[T_6E]),
        (ESCAPE_SEQ_CHAR, &[T_72]),
        (ESCAPE_SEQ_CHAR, &[T_74]),
        (ESCAPE_SEQ_CHAR, &[T_75, FOUR_HEXDIG]),
        (ESCAPE_SEQ_CHAR, &[T_55, EIGHT_HEXDIG]),
        (
            ML_BASIC_STRING,
            &[ML_BASIC_STRING_DELIM, ML_BASIC_BODY, ML_BASIC_STRING_DELIM],
        ),
        (
            ML_BASIC_STRING_DELIM,
            &[QUOTATION_MARK, QUOTATION_MARK, QUOTATION_MARK],
        ),
        (
            ML_BASIC_BODY,
            &[MAYBE_MLB_CONTENT, MLB_QUOTES_CONTENT, MAYBE_MLB_QUOTES],
        ),
        (MAYBE_MLB_QUOTES, &[MLB_QUOTES]),
        (ONE_MAYBE_MLB_CONTENT, &[MLB_CONTENT, ONE_MAYBE_MLB_CONTENT]),
        (ONE_MAYBE_MLB_CONTENT, &[MLB_CONTENT]),
        (
            MLB_QUOTES_CONTENT,
            &[MLB_QUOTES, ONE_MAYBE_MLB_CONTENT, MLB_QUOTES_CONTENT],
        ),
        (MAYBE_MLB_CONTENT, &[MLB_CONTENT, MAYBE_MLB_CONTENT]),
        (MLB_CONTENT, &[MLB_CHAR]),
        (MLB_CONTENT, &[NEWLINE]),
        (MLB_CONTENT, &[MLB_ESCAPED_NL]),
        (MLB_CHAR, &[MLB_UNESCAPED]),
        (MLB_CHAR, &[ESCAPED]),
        (MLB_QUOTES, &[ONE_STAR_TWO_QUOTATION_MARK]),
        (MLB_UNESCAPED, &[WSCHAR]),
        (MLB_UNESCAPED, &[T_BANG]),
        (MLB_UNESCAPED, &[T_23_5B]),
        (MLB_UNESCAPED, &[T_5D_7E]),
        (MLB_UNESCAPED, &[NON_ASCII]),
        (MLB_ESCAPED_NL, &[ESCAPE, WS, NEWLINE, WSCHAR_NLS]),
        (WSCHAR_NL, &[WSCHAR]),
        (WSCHAR_NL, &[NEWLINE]),
        (WSCHAR_NLS, &[WSCHAR_NL, WSCHAR_NLS]),
        (ONE_STAR_TWO_QUOTATION_MARK, &[T_DQUOT, T_DQUOT]),
        (ONE_STAR_TWO_QUOTATION_MARK, &[T_DQUOT]),
        (
            LITERAL_STRING,
            &[APOSTROPHE, MAYBE_LITERAL_CHAR, APOSTROPHE],
        ),
        (MAYBE_LITERAL_CHAR, &[LITERAL_CHAR, MAYBE_LITERAL_CHAR]),
        (APOSTROPHE, &[T_TICK]),
        (LITERAL_CHAR, &[T_TAB]),
        (LITERAL_CHAR, &[T_20_26]),
        (LITERAL_CHAR, &[T_28_7E]),
        (LITERAL_CHAR, &[NON_ASCII]),
        (
            ML_LITERAL_STRING,
            &[
                ML_LITERAL_STRING_DELIM,
                ML_LITERAL_BODY,
                ML_LITERAL_STRING_DELIM,
            ],
        ),
        (ML_LITERAL_STRING_DELIM, &[THREE_APOSTROPHE]),
        (
            ML_LITERAL_BODY,
            &[MAYBE_MLL_CONTENT, SOME_MLL_QUOTES_CONTENT, MAYBE_MLL_QUOTES],
        ),
        (THREE_APOSTROPHE, &[APOSTROPHE, APOSTROPHE, APOSTROPHE]),
        (MAYBE_MLL_CONTENT, &[MLL_CONTENT, MAYBE_MLL_CONTENT]),
        (ONE_MAYBE_MLL_CONTENT, &[MLL_CONTENT, ONE_MAYBE_MLL_CONTENT]),
        (ONE_MAYBE_MLL_CONTENT, &[MLL_CONTENT]),
        (MAYBE_MLL_QUOTES, &[MLL_QUOTES]),
        (
            SOME_MLL_QUOTES_CONTENT,
            &[MLL_QUOTES, ONE_MAYBE_MLL_CONTENT, SOME_MLL_QUOTES_CONTENT],
        ),
        (MLL_CONTENT, &[MLL_CHAR]),
        (MLL_CONTENT, &[NEWLINE]),
        (MLL_CHAR, &[T_TAB]),
        (MLL_CHAR, &[T_20_26]),
        (MLL_CHAR, &[T_28_7E]),
        (MLL_CHAR, &[NON_ASCII]),
        (MLL_QUOTES, &[APOSTROPHE]),
        (MLL_QUOTES, &[APOSTROPHE, APOSTROPHE]),
        (TOML, &[EXPRESSION]),
        (TOML, &[EXPRESSION, EXPRESSIONS]),
        (EXPRESSIONS, &[NEWLINE, EXPRESSION, EXPRESSIONS]),
        (EXPRESSIONS, &[NEWLINE]),
        (EXPRESSION, &[WS, MAYBE_COMMENT]),
        (EXPRESSION, &[WS, KEYVAL, WS, MAYBE_COMMENT]),
        (EXPRESSION, &[WS, TABLE, WS, MAYBE_COMMENT]),
        (KEYVAL, &[KEY, KEYVAL_SEP, VAL]),
        (KEY, &[SIMPLE_KEY]),
        (KEY, &[DOTTED_KEY]),
        (SIMPLE_KEY, &[QUOTED_KEY]),
        (SIMPLE_KEY, &[UNQUOTED_KEY]),
        (UNQUOTED_KEY, &[UNQUOTED_KEY_CHAR, UNQUOTED_KEY]),
        (UNQUOTED_KEY, &[UNQUOTED_KEY_CHAR]),
        (UNQUOTED_KEY_CHAR, &[ALPHA]),
        (UNQUOTED_KEY_CHAR, &[DIGIT]),
        (UNQUOTED_KEY_CHAR, &[T_2D]),
        (UNQUOTED_KEY_CHAR, &[T_5F]),
        (QUOTED_KEY, &[BASIC_STRING]),
        (QUOTED_KEY, &[LITERAL_STRING]),
        (DOTTED_KEY, &[SIMPLE_KEY, DOTTED_KEY_REST]),
        (DOTTED_KEY_REST, &[DOT_SEP, SIMPLE_KEY, DOTTED_KEY_REST]),
        (DOTTED_KEY_REST, &[DOT_SEP, SIMPLE_KEY]),
        (DOT_SEP, &[WS, T_DOT, WS]),
        (KEYVAL_SEP, &[WS, T_EQUAL, WS]),
        (VAL, &[STRING]),
        (VAL, &[BOOLEAN]),
        (VAL, &[ARRAY]),
        (VAL, &[INLINE_TABLE]),
        (VAL, &[DATE_TIME]),
        (VAL, &[FLOAT]),
        (VAL, &[INTEGER]),
    ];

    pub struct Grammar {}

    use sesd::{char::CharMatcher, CompiledGrammar};
    impl CompiledGrammar<char, CharMatcher> for Grammar {
        fn start_symbol(&self) -> SymbolId {
            TOML
        }

        fn rules_count(&self) -> usize {
            RULES.len()
        }

        fn lhs(&self, rule: usize) -> SymbolId {
            RULES[rule].0
        }

        fn rhs(&self, rule: usize) -> &[SymbolId] {
            RULES[rule].1
        }

        fn nt_name(&self, nt: SymbolId) -> &str {
            NT_NAMES[nt as usize]
        }

        fn nt_count(&self) -> SymbolId {
            NT_NAMES.len() as SymbolId
        }

        fn t_count(&self) -> SymbolId {
            TERMINALS.len() as SymbolId
        }

        fn nt_empty_count(&self) -> SymbolId {
            NUMBER_OF_EMPTY_NTS
        }

        fn matcher(&self, term: SymbolId) -> CharMatcher {
            TERMINALS[term as usize].clone()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use sesd::{char::CharMatcher, Parser, Verdict};

    #[test]
    fn table() {
        let compiled_grammar = super::cargo_toml::Grammar {};

        let mut parser =
            Parser::<char, CharMatcher, super::cargo_toml::Grammar>::new(compiled_grammar);
        let mut position = 0;
        for (i, c) in "[key.rest".chars().enumerate() {
            let res = parser.update(i, c);
            assert_eq!(res, Verdict::More);
            position = i;
        }
        let res = parser.update(position + 1, ']');
        parser.print_chart();
        assert_eq!(res, Verdict::Accept);
    }
}
