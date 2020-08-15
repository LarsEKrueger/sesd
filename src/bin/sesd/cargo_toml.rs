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

use super::look_and_feel::StyleBuilder as SB;
use super::look_and_feel::{LookAndFeel, Style, StyleMatcher};

grammar! {
    pub cargo_toml,
    {
        use sesd::char::CharMatcher::*;
    },
    char,
    sesd::char::CharMatcher,
    TOML,
    [
        NON_EOLS,
        MAYBE_COMMENT,
        MAYBE_INLINE_TABLE_KEYVALS,
        MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS,
        MAYBE_ARRAY_VALUES,
        MAYBE_ARRAY_SEP,
        WS_COMMENT_NEWLINE,
        MAYBE_TIME_SECFRAC,
        SIGN,
        HEX_INT_REST,
        OCT_INT_REST,
        BIN_INT_REST,
        MAYBE_EXP,
        ZERO_PREFIXABLE_INT_REST,
        BASIC_CHARS,
        MAYBE_MLB_QUOTES,
        MLB_QUOTES_CONTENT,
        MAYBE_MLB_CONTENT,
        WSCHAR_NLS,
        MAYBE_LITERAL_CHAR,
        MAYBE_MLL_CONTENT,
        MAYBE_MLL_QUOTES,
        SOME_MLL_QUOTES_CONTENT,
        WS
    ],
    [
        DOTTED_KEY_REST,
        DOTTED_KEY,
        DOT_SEP,
        KEY,
        ONE_STAR_TWO_QUOTATION_MARK,
        ONE_STAR_DIGIT,
        ONE_MAYBE_MLB_CONTENT,
        ONE_MAYBE_MLL_CONTENT,
        TWO_DIGIT,
        FOUR_DIGIT,
        THREE_APOSTROPHE,
        FOUR_HEXDIG,
        EIGHT_HEXDIG,
        ALPHA,
        APOSTROPHE,
        ARRAY_CLOSE,
        ARRAY_OPEN,
        ARRAY_SEP,
        ARRAY_TABLE_CLOSE,
        ARRAY_TABLE_OPEN,
        ARRAY_TABLE,
        ARRAY_VALUES,
        ARRAY,
        BASIC_CHAR,
        BASIC_STRING,
        BASIC_UNESCAPED,
        BIN_INT,
        BIN_PREFIX,
        BOOLEAN,
        COMMENT_START_SYMBOL,
        COMMENT,
        DATE_FULLYEAR,
        DATE_MDAY,
        DATE_MONTH,
        DATE_TIME,
        DEC_INT,
        DECIMAL_POINT,
        DIGIT,
        DIGIT_,
        DIGITZERO_ONE_,
        DIGITZERO_ONE_UNDERSCORE,
        DIGITZERO_SEVEN_,
        DIGITZERO_SEVEN_UNDERSCORE,
        DIGITONE_NINE_,
        ESCAPE_SEQ_CHAR,
        ESCAPE,
        ESCAPED,
        EXP,
        EXPRESSION,
        EXPRESSIONS,
        SYM_FALSE,
        FLOAT_EXP_PART,
        FLOAT_INT_PART,
        FLOAT,
        FLOAT_REST,
        FRAC,
        FULL_DATE,
        FULL_TIME,
        HEX_INT,
        HEX_PREFIX,
        HEXDIG,
        HEXDIG_,
        INF,
        INLINE_TABLE_CLOSE,
        INLINE_TABLE_KEYVALS,
        INLINE_TABLE_OPEN,
        INLINE_TABLE_SEP,
        INLINE_TABLE,
        INTEGER,
        KEYVAL_SEP,
        KEYVAL,
        LITERAL_CHAR,
        LITERAL_STRING,
        LOCAL_DATE_TIME,
        LOCAL_DATE,
        LOCAL_TIME,
        MINUS,
        ML_BASIC_BODY,
        ML_BASIC_STRING_DELIM,
        ML_BASIC_STRING,
        ML_LITERAL_BODY,
        ML_LITERAL_STRING_DELIM,
        ML_LITERAL_STRING,
        MLB_CHAR,
        MLB_CONTENT,
        MLB_ESCAPED_NL,
        MLB_QUOTES,
        MLB_UNESCAPED,
        MLL_CHAR,
        MLL_CONTENT,
        MLL_QUOTES,
        NAN,
        NEWLINE,
        NON_ASCII,
        NON_EOL,
        OCT_INT,
        OCT_PREFIX,
        OFFSET_DATE_TIME,
        PARTIAL_TIME,
        PLUS,
        QUOTATION_MARK,
        QUOTED_KEY,
        SIMPLE_KEY,
        SPECIAL_FLOAT,
        STD_TABLE_CLOSE,
        STD_TABLE_OPEN,
        STD_TABLE,
        STRING,
        TABLE,
        TIME_DELIM,
        TIME_HOUR,
        TIME_MINUTE,
        TIME_NUMOFFSET,
        TIME_OFFSET,
        TIME_SECFRAC,
        TIME_SECOND,
        TOML,
        SYM_TRUE,
        UNDERSCORE,
        UNQUOTED_KEY_CHAR,
        UNQUOTED_KEY,
        UNS_DEC_INT_REST,
        UNSIGNED_DEC_INT,
        VAL,
        WSCHAR_NL,
        WSCHAR,
        WSCN,
        ZERO_PREFIXABLE_INT
    ],
    [
        T_MINUS = Exact('-'),
        T_SPACE = Exact(' '),
        T_BANG = Exact('!'),
        T_DQUOT = Exact('"'),
        T_HASH = Exact('#'),
        T_COMMA = Exact(','),
        T_DOT = Exact('.'),
        T_COLON = Exact(':'),
        T_SQ_OPEN = Exact('['),
        T_TICK = Exact('\''),
        T_BACKSLASH = Exact('\\'),
        T_TAB = Exact('\t'),
        T_NL = Exact('\x0A'),
        T_CR = Exact('\x0D'),
        T_22 = Exact('\x22'),
        T_2D = Exact('\x2D'),
        T_55 = Exact('\x55'),
        T_5C = Exact('\x5C'),
        T_5F = Exact('\x5F'),
        T_62 = Exact('\x62'),
        T_66 = Exact('\x66'),
        T_6E = Exact('\x6E'),
        T_72 = Exact('\x72'),
        T_74 = Exact('\x74'),
        T_75 = Exact('\x75'),
        T_SQ_CLOSE = Exact(']'),
        T_UNDERSCORE = Exact('_'),
        T_CURLY_OPEN = Exact('{'),
        T_CURLY_CLOSE = Exact('}'),
        T_PLUS = Exact('+'),
        T_EQUAL = Exact('='),
        T_ZERO = Exact('0'),
        T_A = Exact('a'),
        T_B = Exact('b'),
        T_E = Exact('e'),
        T_F = Exact('f'),
        T_I = Exact('i'),
        T_L = Exact('l'),
        T_N = Exact('n'),
        T_O = Exact('o'),
        T_R = Exact('r'),
        T_S = Exact('s'),
        T_LC_T = Exact('t'),
        T_UC_T = Exact('T'),
        T_U = Exact('u'),
        T_X = Exact('x'),
        T_LC_Z = Exact('z'),
        T_UC_Z = Exact('Z'),
        T_80_D7FF = Range('\u{80}', '\u{D7FF}'),
        T_E000_10FFFF = Range('\u{E000}', '\u{10FFFF}'),
        T_20_26 = Range('\x20', '\x26'),
        T_20_7F = Range('\x20', '\x7F'),
        T_23_5B = Range('\x23', '\x5B'),
        T_28_7E = Range('\x28', '\x7E'),
        T_5D_7E = Range('\x5D', '\x7E'),
        T_0_1 = Range('0', '1'),
        T_0_7 = Range('0', '7'),
        T_0_9 = Range('0', '9'),
        T_1_9 = Range('1', '9'),
        T_LC_A_F = Range('a', 'f'),
        T_UC_A_F = Range('A', 'F'),
        T_UC_A_Z = Range('A', 'Z'),
        T_LC_A_Z = Range('a', 'z')
    ],
    [
        ALPHA = T_UC_A_Z,
        ALPHA = T_LC_A_Z,
        DIGIT = T_0_9,
        HEXDIG = DIGIT,
        HEXDIG = T_UC_A_F,
        HEXDIG = T_LC_A_F,
        FOUR_HEXDIG = HEXDIG HEXDIG HEXDIG HEXDIG,
        EIGHT_HEXDIG = FOUR_HEXDIG FOUR_HEXDIG,
        WS = WSCHAR WS,
        WSCHAR = T_SPACE,
        WSCHAR = T_TAB,
        NEWLINE = T_NL,
        NEWLINE = T_CR T_NL,
        COMMENT_START_SYMBOL = T_HASH,
        NON_ASCII = T_80_D7FF,
        NON_ASCII = T_E000_10FFFF,
        NON_EOL = T_TAB,
        NON_EOL = T_20_7F,
        NON_EOL = NON_ASCII,
        COMMENT = COMMENT_START_SYMBOL NON_EOLS,
        NON_EOLS = NON_EOL NON_EOLS,
        MAYBE_COMMENT = COMMENT,
        TABLE = STD_TABLE,
        TABLE = ARRAY_TABLE,
        STD_TABLE = STD_TABLE_OPEN KEY STD_TABLE_CLOSE,
        STD_TABLE_OPEN = T_SQ_OPEN WS,
        STD_TABLE_CLOSE = WS T_SQ_CLOSE,
        INLINE_TABLE =  INLINE_TABLE_OPEN MAYBE_INLINE_TABLE_KEYVALS INLINE_TABLE_CLOSE ,
        INLINE_TABLE_OPEN = T_CURLY_OPEN WS,
        INLINE_TABLE_CLOSE = WS T_CURLY_CLOSE,
        INLINE_TABLE_SEP = WS T_COMMA WS,
        MAYBE_INLINE_TABLE_KEYVALS = INLINE_TABLE_KEYVALS,
        INLINE_TABLE_KEYVALS = KEYVAL MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS,
        MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS = INLINE_TABLE_SEP INLINE_TABLE_KEYVALS,
        ARRAY_TABLE = ARRAY_TABLE_OPEN KEY ARRAY_TABLE_CLOSE,
        ARRAY_TABLE_OPEN = T_SQ_OPEN T_SQ_OPEN WS,
        ARRAY_TABLE_CLOSE = WS T_SQ_CLOSE T_SQ_CLOSE,
        ARRAY =  ARRAY_OPEN MAYBE_ARRAY_VALUES WS_COMMENT_NEWLINE ARRAY_CLOSE ,
        MAYBE_ARRAY_VALUES = ARRAY_VALUES,
        ARRAY_OPEN = T_SQ_OPEN,
        ARRAY_CLOSE = T_SQ_CLOSE,
        ARRAY_VALUES = WS_COMMENT_NEWLINE VAL WS ARRAY_SEP ARRAY_VALUES,
        ARRAY_VALUES = WS_COMMENT_NEWLINE VAL WS MAYBE_ARRAY_SEP,
        MAYBE_ARRAY_SEP = ARRAY_SEP,
        ARRAY_SEP = T_COMMA,
        WS_COMMENT_NEWLINE = WSCN WS_COMMENT_NEWLINE,
        WSCN = WSCHAR,
        WSCN = MAYBE_COMMENT NEWLINE,
        MAYBE_COMMENT = COMMENT,
        DATE_TIME = OFFSET_DATE_TIME,
        DATE_TIME = LOCAL_DATE_TIME,
        DATE_TIME = LOCAL_DATE,
        DATE_TIME = LOCAL_TIME,
        DATE_FULLYEAR = FOUR_DIGIT,
        FOUR_DIGIT = TWO_DIGIT TWO_DIGIT,
        TWO_DIGIT = DIGIT DIGIT,
        DATE_MONTH = TWO_DIGIT,
        DATE_MDAY = TWO_DIGIT,
        TIME_DELIM = T_UC_T,
        TIME_DELIM = T_LC_T,
        TIME_DELIM = T_SPACE,
        TIME_HOUR = TWO_DIGIT,
        TIME_MINUTE = TWO_DIGIT,
        TIME_SECOND = TWO_DIGIT,
        TIME_SECFRAC = T_COMMA ONE_STAR_DIGIT,
        ONE_STAR_DIGIT = DIGIT ONE_STAR_DIGIT,
        ONE_STAR_DIGIT = DIGIT,
        TIME_NUMOFFSET = SIGN TIME_HOUR T_COLON TIME_MINUTE,
        TIME_OFFSET = T_UC_Z,
        TIME_OFFSET = T_LC_Z,
        TIME_OFFSET = TIME_NUMOFFSET,
        PARTIAL_TIME =  TIME_HOUR T_COLON TIME_MINUTE T_COLON TIME_SECOND MAYBE_TIME_SECFRAC ,
        MAYBE_TIME_SECFRAC = TIME_SECFRAC,
        FULL_DATE =  DATE_FULLYEAR T_UNDERSCORE DATE_MONTH T_UNDERSCORE DATE_MDAY ,
        FULL_TIME = PARTIAL_TIME TIME_OFFSET,
        OFFSET_DATE_TIME = FULL_DATE TIME_DELIM FULL_TIME,
        LOCAL_DATE_TIME = FULL_DATE TIME_DELIM PARTIAL_TIME,
        LOCAL_DATE = FULL_DATE,
        LOCAL_TIME = PARTIAL_TIME,
        INTEGER = DEC_INT,
        INTEGER = HEX_INT,
        INTEGER = OCT_INT,
        INTEGER = BIN_INT,
        MINUS = T_MINUS,
        PLUS = T_PLUS,
        UNDERSCORE = T_UNDERSCORE,
        DIGITONE_NINE_ = T_1_9,
        DIGITZERO_SEVEN_ = T_0_7,
        DIGITZERO_ONE_ = T_0_1,
        HEX_PREFIX = T_ZERO T_X,
        OCT_PREFIX = T_ZERO T_O,
        BIN_PREFIX = T_ZERO T_B,
        DEC_INT = SIGN UNSIGNED_DEC_INT,
        SIGN = MINUS,
        SIGN = PLUS,
        UNSIGNED_DEC_INT = DIGIT,
        UNSIGNED_DEC_INT = DIGITONE_NINE_ UNS_DEC_INT_REST,
        UNS_DEC_INT_REST = DIGIT_ UNS_DEC_INT_REST,
        UNS_DEC_INT_REST = DIGIT_,
        DIGIT_ = DIGIT,
        DIGIT_ = UNDERSCORE DIGIT,
        HEX_INT = HEX_PREFIX HEXDIG HEX_INT_REST,
        HEX_INT_REST = HEXDIG_ HEX_INT_REST,
        HEXDIG_ = HEXDIG,
        HEXDIG_ = UNDERSCORE HEXDIG,
        OCT_INT = OCT_PREFIX DIGITZERO_SEVEN_ OCT_INT_REST,
        OCT_INT_REST = DIGITZERO_SEVEN_ OCT_INT_REST,
        DIGITZERO_SEVEN_UNDERSCORE = DIGITZERO_SEVEN_,
        DIGITZERO_SEVEN_UNDERSCORE = UNDERSCORE DIGITZERO_SEVEN_,
        BIN_INT = BIN_PREFIX DIGITZERO_ONE_ BIN_INT_REST,
        BIN_INT_REST = DIGITZERO_ONE_UNDERSCORE BIN_INT_REST,
        DIGITZERO_ONE_UNDERSCORE = DIGITZERO_ONE_,
        DIGITZERO_ONE_UNDERSCORE = UNDERSCORE DIGITZERO_ONE_,
        FLOAT = FLOAT_INT_PART FLOAT_REST,
        FLOAT = SPECIAL_FLOAT,
        FLOAT_REST = EXP,
        FLOAT_REST = FRAC MAYBE_EXP,
        MAYBE_EXP = EXP,
        FLOAT_INT_PART = DEC_INT,
        FRAC = DECIMAL_POINT ZERO_PREFIXABLE_INT,
        DECIMAL_POINT = T_DOT,
        ZERO_PREFIXABLE_INT = DIGIT ZERO_PREFIXABLE_INT_REST,
        ZERO_PREFIXABLE_INT_REST = DIGIT_ ZERO_PREFIXABLE_INT_REST,
        EXP = T_E FLOAT_EXP_PART,
        FLOAT_EXP_PART = SIGN ZERO_PREFIXABLE_INT,
        SPECIAL_FLOAT = SIGN INF,
        SPECIAL_FLOAT = SIGN NAN,
        INF = T_I T_N T_F,
        NAN = T_N T_A T_N,
        BOOLEAN = SYM_TRUE,
        BOOLEAN = SYM_FALSE,
        SYM_TRUE = T_LC_T T_R T_U T_E,
        SYM_FALSE = T_F T_A T_L T_S T_E,
        STRING = ML_BASIC_STRING,
        STRING = BASIC_STRING,
        STRING = ML_LITERAL_STRING,
        STRING = LITERAL_STRING,
        BASIC_STRING = QUOTATION_MARK BASIC_CHARS QUOTATION_MARK,
        BASIC_CHARS = BASIC_CHAR BASIC_CHARS,
        QUOTATION_MARK = T_DQUOT,
        BASIC_CHAR = BASIC_UNESCAPED,
        BASIC_CHAR = ESCAPED,
        BASIC_UNESCAPED = WSCHAR,
        BASIC_UNESCAPED = T_BANG,
        BASIC_UNESCAPED = T_23_5B,
        BASIC_UNESCAPED = T_5D_7E,
        BASIC_UNESCAPED = NON_ASCII,
        ESCAPED = ESCAPE ESCAPE_SEQ_CHAR,
        ESCAPE = T_BACKSLASH,
        ESCAPE_SEQ_CHAR = T_22,
        ESCAPE_SEQ_CHAR = T_5C,
        ESCAPE_SEQ_CHAR = T_62,
        ESCAPE_SEQ_CHAR = T_66,
        ESCAPE_SEQ_CHAR = T_6E,
        ESCAPE_SEQ_CHAR = T_72,
        ESCAPE_SEQ_CHAR = T_74,
        ESCAPE_SEQ_CHAR = T_75 FOUR_HEXDIG,
        ESCAPE_SEQ_CHAR = T_55 EIGHT_HEXDIG,
        ML_BASIC_STRING = ML_BASIC_STRING_DELIM ML_BASIC_BODY ML_BASIC_STRING_DELIM,
        ML_BASIC_STRING_DELIM = QUOTATION_MARK QUOTATION_MARK QUOTATION_MARK,
        ML_BASIC_BODY = MAYBE_MLB_CONTENT MLB_QUOTES_CONTENT MAYBE_MLB_QUOTES,
        MAYBE_MLB_QUOTES = MLB_QUOTES,
        ONE_MAYBE_MLB_CONTENT = MLB_CONTENT ONE_MAYBE_MLB_CONTENT,
        ONE_MAYBE_MLB_CONTENT = MLB_CONTENT,
        MLB_QUOTES_CONTENT = MLB_QUOTES ONE_MAYBE_MLB_CONTENT MLB_QUOTES_CONTENT,
        MAYBE_MLB_CONTENT = MLB_CONTENT MAYBE_MLB_CONTENT,
        MLB_CONTENT = MLB_CHAR,
        MLB_CONTENT = NEWLINE,
        MLB_CONTENT = MLB_ESCAPED_NL,
        MLB_CHAR = MLB_UNESCAPED,
        MLB_CHAR = ESCAPED,
        MLB_QUOTES = ONE_STAR_TWO_QUOTATION_MARK,
        MLB_UNESCAPED = WSCHAR,
        MLB_UNESCAPED = T_BANG,
        MLB_UNESCAPED = T_23_5B,
        MLB_UNESCAPED = T_5D_7E,
        MLB_UNESCAPED = NON_ASCII,
        MLB_ESCAPED_NL = ESCAPE WS NEWLINE WSCHAR_NLS,
        WSCHAR_NL = WSCHAR,
        WSCHAR_NL = NEWLINE,
        WSCHAR_NLS = WSCHAR_NL WSCHAR_NLS,
        ONE_STAR_TWO_QUOTATION_MARK = T_DQUOT T_DQUOT,
        ONE_STAR_TWO_QUOTATION_MARK = T_DQUOT,
        LITERAL_STRING = APOSTROPHE MAYBE_LITERAL_CHAR APOSTROPHE,
        MAYBE_LITERAL_CHAR = LITERAL_CHAR MAYBE_LITERAL_CHAR,
        APOSTROPHE = T_TICK,
        LITERAL_CHAR = T_TAB,
        LITERAL_CHAR = T_20_26,
        LITERAL_CHAR = T_28_7E,
        LITERAL_CHAR = NON_ASCII,
        ML_LITERAL_STRING =  ML_LITERAL_STRING_DELIM ML_LITERAL_BODY ML_LITERAL_STRING_DELIM ,
        ML_LITERAL_STRING_DELIM = THREE_APOSTROPHE,
        ML_LITERAL_BODY = MAYBE_MLL_CONTENT SOME_MLL_QUOTES_CONTENT MAYBE_MLL_QUOTES,
        THREE_APOSTROPHE = APOSTROPHE APOSTROPHE APOSTROPHE,
        MAYBE_MLL_CONTENT = MLL_CONTENT MAYBE_MLL_CONTENT,
        ONE_MAYBE_MLL_CONTENT = MLL_CONTENT ONE_MAYBE_MLL_CONTENT,
        ONE_MAYBE_MLL_CONTENT = MLL_CONTENT,
        MAYBE_MLL_QUOTES = MLL_QUOTES,
        SOME_MLL_QUOTES_CONTENT = MLL_QUOTES ONE_MAYBE_MLL_CONTENT SOME_MLL_QUOTES_CONTENT,
        MLL_CONTENT = MLL_CHAR,
        MLL_CONTENT = NEWLINE,
        MLL_CHAR = T_TAB,
        MLL_CHAR = T_20_26,
        MLL_CHAR = T_28_7E,
        MLL_CHAR = NON_ASCII,
        MLL_QUOTES = APOSTROPHE,
        MLL_QUOTES = APOSTROPHE APOSTROPHE,
        TOML = EXPRESSION,
        TOML = EXPRESSION EXPRESSIONS,
        EXPRESSIONS = NEWLINE EXPRESSION EXPRESSIONS,
        EXPRESSIONS = NEWLINE,
        EXPRESSION = WS MAYBE_COMMENT,
        EXPRESSION = WS KEYVAL WS MAYBE_COMMENT,
        EXPRESSION = WS TABLE WS MAYBE_COMMENT,
        KEYVAL = KEY KEYVAL_SEP VAL,
        KEY = SIMPLE_KEY,
        KEY = DOTTED_KEY,
        SIMPLE_KEY = QUOTED_KEY,
        SIMPLE_KEY = UNQUOTED_KEY,
        UNQUOTED_KEY = UNQUOTED_KEY_CHAR UNQUOTED_KEY,
        UNQUOTED_KEY = UNQUOTED_KEY_CHAR,
        UNQUOTED_KEY_CHAR = ALPHA,
        UNQUOTED_KEY_CHAR = DIGIT,
        UNQUOTED_KEY_CHAR = T_2D,
        UNQUOTED_KEY_CHAR = T_5F,
        QUOTED_KEY = BASIC_STRING,
        QUOTED_KEY = LITERAL_STRING,
        DOTTED_KEY = SIMPLE_KEY DOTTED_KEY_REST,
        DOTTED_KEY_REST = DOT_SEP SIMPLE_KEY DOTTED_KEY_REST,
        DOTTED_KEY_REST = DOT_SEP SIMPLE_KEY,
        DOT_SEP = WS T_DOT WS,
        KEYVAL_SEP = WS T_EQUAL WS,
        VAL = STRING,
        VAL = BOOLEAN,
        VAL = ARRAY,
        VAL = INLINE_TABLE,
        VAL = DATE_TIME,
        VAL = FLOAT,
        VAL = INTEGER
    ]
}

/*
pub mod cargo_toml {
    use sesd::SymbolId;

    // Non-terminals with empty rules
    pub const NON_EOLS: SymbolId = 1;
    pub const MAYBE_COMMENT: SymbolId = 2;
    pub const MAYBE_INLINE_TABLE_KEYVALS: SymbolId = 3;
    pub const MAYBE_INLINE_TABLE_SEPINLINE_TABLE_KEYVALS: SymbolId = 4;
    pub const MAYBE_ARRAY_VALUES: SymbolId = 5;
    pub const MAYBE_ARRAY_SEP: SymbolId = 6;
    pub const WS_COMMENT_NEWLINE: SymbolId = 7;
    pub const MAYBE_TIME_SECFRAC: SymbolId = 8;
    pub const SIGN: SymbolId = 9;
    pub const HEX_INT_REST: SymbolId = 10;
    pub const OCT_INT_REST: SymbolId = 11;
    pub const BIN_INT_REST: SymbolId = 12;
    pub const MAYBE_EXP: SymbolId = 13;
    pub const ZERO_PREFIXABLE_INT_REST: SymbolId = 14;
    pub const BASIC_CHARS: SymbolId = 15;
    pub const MAYBE_MLB_QUOTES: SymbolId = 16;
    pub const MLB_QUOTES_CONTENT: SymbolId = 17;
    pub const MAYBE_MLB_CONTENT: SymbolId = 18;
    pub const WSCHAR_NLS: SymbolId = 19;
    pub const MAYBE_LITERAL_CHAR: SymbolId = 20;
    pub const MAYBE_MLL_CONTENT: SymbolId = 21;
    pub const MAYBE_MLL_QUOTES: SymbolId = 22;
    pub const SOME_MLL_QUOTES_CONTENT: SymbolId = 23;
    pub const WS: SymbolId = 24;

    const NUMBER_OF_EMPTY_NTS: SymbolId = 25;

    // Other non-terminals
    pub const DOTTED_KEY_REST: SymbolId = 25;
    pub const DOTTED_KEY: SymbolId = 26;
    pub const DOT_SEP: SymbolId = 27;
    pub const KEY: SymbolId = 28;
    pub const ONE_STAR_TWO_QUOTATION_MARK: SymbolId = 29;
    pub const ONE_STAR_DIGIT: SymbolId = 30;
    pub const ONE_MAYBE_MLB_CONTENT: SymbolId = 31;
    pub const ONE_MAYBE_MLL_CONTENT: SymbolId = 32;
    pub const TWO_DIGIT: SymbolId = 33;
    pub const FOUR_DIGIT: SymbolId = 34;
    pub const THREE_APOSTROPHE: SymbolId = 35;
    pub const FOUR_HEXDIG: SymbolId = 36;
    pub const EIGHT_HEXDIG: SymbolId = 37;
    pub const ALPHA: SymbolId = 38;
    pub const APOSTROPHE: SymbolId = 39;
    pub const ARRAY_CLOSE: SymbolId = 40;
    pub const ARRAY_OPEN: SymbolId = 41;
    pub const ARRAY_SEP: SymbolId = 42;
    pub const ARRAY_TABLE_CLOSE: SymbolId = 43;
    pub const ARRAY_TABLE_OPEN: SymbolId = 44;
    pub const ARRAY_TABLE: SymbolId = 45;
    pub const ARRAY_VALUES: SymbolId = 46;
    pub const ARRAY: SymbolId = 47;
    pub const BASIC_CHAR: SymbolId = 48;
    pub const BASIC_STRING: SymbolId = 49;
    pub const BASIC_UNESCAPED: SymbolId = 50;
    pub const BIN_INT: SymbolId = 51;
    pub const BIN_PREFIX: SymbolId = 52;
    pub const BOOLEAN: SymbolId = 53;
    pub const COMMENT_START_SYMBOL: SymbolId = 54;
    pub const COMMENT: SymbolId = 55;
    pub const DATE_FULLYEAR: SymbolId = 56;
    pub const DATE_MDAY: SymbolId = 57;
    pub const DATE_MONTH: SymbolId = 58;
    pub const DATE_TIME: SymbolId = 59;
    pub const DEC_INT: SymbolId = 60;
    pub const DECIMAL_POINT: SymbolId = 61;
    pub const DIGIT: SymbolId = 62;
    pub const DIGIT_: SymbolId = 63;
    pub const DIGITZERO_ONE_: SymbolId = 64;
    pub const DIGITZERO_ONE_UNDERSCORE: SymbolId = 65;
    pub const DIGITZERO_SEVEN_: SymbolId = 66;
    pub const DIGITZERO_SEVEN_UNDERSCORE: SymbolId = 67;
    pub const DIGITONE_NINE_: SymbolId = 68;
    pub const ESCAPE_SEQ_CHAR: SymbolId = 69;
    pub const ESCAPE: SymbolId = 70;
    pub const ESCAPED: SymbolId = 71;
    pub const EXP: SymbolId = 72;
    pub const EXPRESSION: SymbolId = 73;
    pub const EXPRESSIONS: SymbolId = 74;
    pub const SYM_FALSE: SymbolId = 75;
    pub const FLOAT_EXP_PART: SymbolId = 76;
    pub const FLOAT_INT_PART: SymbolId = 77;
    pub const FLOAT: SymbolId = 78;
    pub const FLOAT_REST: SymbolId = 79;
    pub const FRAC: SymbolId = 80;
    pub const FULL_DATE: SymbolId = 81;
    pub const FULL_TIME: SymbolId = 82;
    pub const HEX_INT: SymbolId = 83;
    pub const HEX_PREFIX: SymbolId = 84;
    pub const HEXDIG: SymbolId = 85;
    pub const HEXDIG_: SymbolId = 86;
    pub const INF: SymbolId = 87;
    pub const INLINE_TABLE_CLOSE: SymbolId = 88;
    pub const INLINE_TABLE_KEYVALS: SymbolId = 89;
    pub const INLINE_TABLE_OPEN: SymbolId = 90;
    pub const INLINE_TABLE_SEP: SymbolId = 91;
    pub const INLINE_TABLE: SymbolId = 92;
    pub const INTEGER: SymbolId = 93;
    pub const KEYVAL_SEP: SymbolId = 94;
    pub const KEYVAL: SymbolId = 95;
    pub const LITERAL_CHAR: SymbolId = 96;
    pub const LITERAL_STRING: SymbolId = 97;
    pub const LOCAL_DATE_TIME: SymbolId = 98;
    pub const LOCAL_DATE: SymbolId = 99;
    pub const LOCAL_TIME: SymbolId = 100;
    pub const MINUS: SymbolId = 101;
    pub const ML_BASIC_BODY: SymbolId = 102;
    pub const ML_BASIC_STRING_DELIM: SymbolId = 103;
    pub const ML_BASIC_STRING: SymbolId = 104;
    pub const ML_LITERAL_BODY: SymbolId = 105;
    pub const ML_LITERAL_STRING_DELIM: SymbolId = 106;
    pub const ML_LITERAL_STRING: SymbolId = 107;
    pub const MLB_CHAR: SymbolId = 108;
    pub const MLB_CONTENT: SymbolId = 109;
    pub const MLB_ESCAPED_NL: SymbolId = 110;
    pub const MLB_QUOTES: SymbolId = 111;
    pub const MLB_UNESCAPED: SymbolId = 112;
    pub const MLL_CHAR: SymbolId = 113;
    pub const MLL_CONTENT: SymbolId = 114;
    pub const MLL_QUOTES: SymbolId = 115;
    pub const NAN: SymbolId = 116;
    pub const NEWLINE: SymbolId = 117;
    pub const NON_ASCII: SymbolId = 118;
    pub const NON_EOL: SymbolId = 119;
    pub const OCT_INT: SymbolId = 120;
    pub const OCT_PREFIX: SymbolId = 121;
    pub const OFFSET_DATE_TIME: SymbolId = 122;
    pub const PARTIAL_TIME: SymbolId = 123;
    pub const PLUS: SymbolId = 124;
    pub const QUOTATION_MARK: SymbolId = 125;
    pub const QUOTED_KEY: SymbolId = 126;
    pub const SIMPLE_KEY: SymbolId = 127;
    pub const SPECIAL_FLOAT: SymbolId = 128;
    pub const STD_TABLE_CLOSE: SymbolId = 129;
    pub const STD_TABLE_OPEN: SymbolId = 130;
    pub const STD_TABLE: SymbolId = 131;
    pub const STRING: SymbolId = 132;
    pub const TABLE: SymbolId = 133;
    pub const TIME_DELIM: SymbolId = 134;
    pub const TIME_HOUR: SymbolId = 135;
    pub const TIME_MINUTE: SymbolId = 136;
    pub const TIME_NUMOFFSET: SymbolId = 137;
    pub const TIME_OFFSET: SymbolId = 138;
    pub const TIME_SECFRAC: SymbolId = 139;
    pub const TIME_SECOND: SymbolId = 140;
    pub const TOML: SymbolId = 141;
    pub const SYM_TRUE: SymbolId = 142;
    pub const UNDERSCORE: SymbolId = 143;
    pub const UNQUOTED_KEY_CHAR: SymbolId = 144;
    pub const UNQUOTED_KEY: SymbolId = 145;
    pub const UNS_DEC_INT_REST: SymbolId = 146;
    pub const UNSIGNED_DEC_INT: SymbolId = 147;
    pub const VAL: SymbolId = 148;
    pub const WSCHAR_NL: SymbolId = 149;
    pub const WSCHAR: SymbolId = 150;
    pub const WSCN: SymbolId = 151;
    pub const ZERO_PREFIXABLE_INT: SymbolId = 152;

    // Terminal symbols
    pub const T_MINUS: SymbolId = 153; //    Exact('-')
    pub const T_SPACE: SymbolId = 154; //    Exact(' ')
    pub const T_BANG: SymbolId = 155; //    Exact('!')
    pub const T_DQUOT: SymbolId = 156; //    Exact('"')
    pub const T_HASH: SymbolId = 157; //    Exact('#')
    pub const T_COMMA: SymbolId = 158; //    Exact(',')
    pub const T_DOT: SymbolId = 159; //    Exact('.')
    pub const T_COLON: SymbolId = 160; //    Exact(':')
    pub const T_SQ_OPEN: SymbolId = 161; //    Exact('[')
    pub const T_TICK: SymbolId = 162; //    Exact('\'')
    pub const T_BACKSLASH: SymbolId = 163; //    Exact('\\')
    pub const T_TAB: SymbolId = 164; //    Exact('\t')
    pub const T_NL: SymbolId = 165; //    Exact('\x0A')
    pub const T_CR: SymbolId = 166; //    Exact('\x0D')
    pub const T_22: SymbolId = 167; //    Exact('\x22')
    pub const T_2D: SymbolId = 168; //    Exact('\x2D')
    pub const T_55: SymbolId = 169; //    Exact('\x55')
    pub const T_5C: SymbolId = 170; //    Exact('\x5C')
    pub const T_5F: SymbolId = 171; //    Exact('\x5F')
    pub const T_62: SymbolId = 172; //    Exact('\x62')
    pub const T_66: SymbolId = 173; //    Exact('\x66')
    pub const T_6E: SymbolId = 174; //    Exact('\x6E')
    pub const T_72: SymbolId = 175; //    Exact('\x72')
    pub const T_74: SymbolId = 176; //    Exact('\x74')
    pub const T_75: SymbolId = 177; //    Exact('\x75')
    pub const T_SQ_CLOSE: SymbolId = 178; //    Exact(']')
    pub const T_UNDERSCORE: SymbolId = 179; //    Exact('_')
    pub const T_CURLY_OPEN: SymbolId = 180; //    Exact('{')
    pub const T_CURLY_CLOSE: SymbolId = 181; //    Exact('}')
    pub const T_PLUS: SymbolId = 182; //    Exact('+')
    pub const T_EQUAL: SymbolId = 183; //    Exact('=')
    pub const T_ZERO: SymbolId = 184; //    Exact('0')
    pub const T_A: SymbolId = 185; //    Exact('a')
    pub const T_B: SymbolId = 186; //    Exact('b')
    pub const T_E: SymbolId = 187; //    Exact('e')
    pub const T_F: SymbolId = 188; //    Exact('f')
    pub const T_I: SymbolId = 189; //    Exact('i')
    pub const T_L: SymbolId = 190; //    Exact('l')
    pub const T_N: SymbolId = 191; //    Exact('n')
    pub const T_O: SymbolId = 192; //    Exact('o')
    pub const T_R: SymbolId = 193; //    Exact('r')
    pub const T_S: SymbolId = 194; //    Exact('s')
    pub const T_LC_T: SymbolId = 195; //    Exact('t')
    pub const T_UC_T: SymbolId = 196; //    Exact('T')
    pub const T_U: SymbolId = 197; //    Exact('u')
    pub const T_X: SymbolId = 198; //    Exact('x')
    pub const T_LC_Z: SymbolId = 199; //    Exact('z')
    pub const T_UC_Z: SymbolId = 200; //    Exact('Z')
    pub const T_80_D7FF: SymbolId = 201; //    Range('\u{80}', '\u{D7FF}')
    pub const T_E000_10FFFF: SymbolId = 202; //    Range('\u{E000}', '\u{10FFFF}')
    pub const T_20_26: SymbolId = 203; //    Range('\x20', '\x26')
    pub const T_20_7F: SymbolId = 204; //    Range('\x20', '\x7F')
    pub const T_23_5B: SymbolId = 205; //    Range('\x23', '\x5B')
    pub const T_28_7E: SymbolId = 206; //    Range('\x28', '\x7E')
    pub const T_5D_7E: SymbolId = 207; //    Range('\x5D', '\x7E')
    pub const T_0_1: SymbolId = 208; //    Range('0', '1')
    pub const T_0_7: SymbolId = 209; //    Range('0', '7')
    pub const T_0_9: SymbolId = 210; //    Range('0', '9')
    pub const T_1_9: SymbolId = 211; //    Range('1', '9')
    pub const T_LC_A_F: SymbolId = 212; //    Range('a', 'f')
    pub const T_UC_A_F: SymbolId = 213; //    Range('A', 'F')
    pub const T_UC_A_Z: SymbolId = 214; //    Range('A', 'Z')
    pub const T_LC_A_Z: SymbolId = 215; //    Range('a', 'z')

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

    const RULES: [(SymbolId, &[SymbolId]); 240] = [
        (sesd::ERROR_ID, &[]),
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
    pub fn grammar() -> cargo_toml::Grammar {
        cargo_toml::Grammar {}
    }

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
*/

/// Build the style sheet for Cargo.toml files
pub fn look_and_feel() -> LookAndFeel {
    let mut sheet = LookAndFeel::new(Style::none());

    use cargo_toml::*;

    // Table headers, underlined
    sheet.add_style(
        StyleMatcher::new(SB::new().u().s)
            .exact(TOML)
            .star(EXPRESSIONS)
            .exact(EXPRESSION)
            .exact(TABLE),
    );

    // Comments, italic
    sheet.add_style(
        StyleMatcher::new(SB::new().i().s)
            .exact(TOML)
            .star(EXPRESSIONS)
            .exact(EXPRESSION)
            .exact(MAYBE_COMMENT)
            .exact(COMMENT),
    );

    // Keys, cyan on black
    sheet.add_style(
        StyleMatcher::new(SB::new().cp(pancurses::ColorPair(0o60)).s)
            .exact(TOML)
            .star(EXPRESSIONS)
            .exact(EXPRESSION)
            .exact(KEYVAL)
            .exact(KEY),
    );

    // String values, magenta on black
    sheet.add_style(
        StyleMatcher::new(SB::new().cp(pancurses::ColorPair(0o50)).s)
            .exact(TOML)
            .star(EXPRESSIONS)
            .exact(EXPRESSION)
            .exact(KEYVAL)
            .exact(VAL)
            .exact(STRING),
    );

    // Array values, magenta on black, underline
    sheet.add_style(
        StyleMatcher::new(SB::new().cp(pancurses::ColorPair(0o50)).u().s)
            .exact(TOML)
            .star(EXPRESSIONS)
            .exact(EXPRESSION)
            .exact(KEYVAL)
            .exact(VAL)
            .exact(ARRAY),
    );

    // Struct values, magenta on black, italic
    sheet.add_style(
        StyleMatcher::new(SB::new().cp(pancurses::ColorPair(0o50)).i().s)
            .exact(TOML)
            .star(EXPRESSIONS)
            .exact(EXPRESSION)
            .exact(KEYVAL)
            .exact(VAL)
            .exact(INLINE_TABLE),
    );

    // Any error, white on red
    sheet.add_style(
        StyleMatcher::new(SB::new().cp(pancurses::ColorPair(0o71)).i().s).skip_to(sesd::ERROR_ID),
    );

    // Predictions
    sheet.add_prediction(
        TABLE,
        &[
            "[package]",
            "[lib]",
            "[[bin]]",
            "[[example]]",
            "[[test]]",
            "[[bench]]",
            "[dependencies]",
            "[dev-dependencies]",
            "[build-dependencies]",
            "[target]",
            "[badges]",
            "[features]",
            "[patch]",
            "[replace]",
            "[profile]",
            "[workspace]",
        ],
    );

    sheet
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
