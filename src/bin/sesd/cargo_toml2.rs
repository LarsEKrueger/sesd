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

mod CargoToml {
    use super::SymbolId;

    // Non-terminals with empty rules
    const non_eols: SymbolId = 1;
    const maybe_comment: SymbolId = 2;
    const maybe_inline_table_keyvals: SymbolId = 3;
    const maybe_inline_table_sepinline_table_keyvals: SymbolId = 4;
    const maybe_array_values: SymbolId = 5;
    const maybe_array_sep: SymbolId = 6;
    const ws_comment_newline: SymbolId = 7;
    const maybe_time_secfrac: SymbolId = 8;
    const sign: SymbolId = 9;
    const hex_int_rest: SymbolId = 10;
    const oct_int_rest: SymbolId = 11;
    const bin_int_rest: SymbolId = 12;
    const maybe_exp: SymbolId = 13;
    const zero_prefixable_int_rest: SymbolId = 14;
    const basic_chars: SymbolId = 15;
    const maybe_mlb_quotes: SymbolId = 16;
    const mlb_quotes_content: SymbolId = 17;
    const maybe_mlb_content: SymbolId = 18;
    const wschar_nls: SymbolId = 19;
    const maybe_literal_char: SymbolId = 20;
    const maybe_mll_content: SymbolId = 21;
    const maybe_mll_quotes: SymbolId = 22;
    const some_mll_quotes_content: SymbolId = 23;
    const ws: SymbolId = 24;

    const number_of_empty_nts: SymbolId = 25;

    // Other non-terminals
    const dotted_key_rest: SymbolId = 25;
    const dotted_key: SymbolId = 26;
    const dot_sep: SymbolId = 27;
    const key: SymbolId = 28;
    const one_star_two_quotation_mark: SymbolId = 29;
    const one_star_DIGIT: SymbolId = 30;
    const one_maybe_mlb_content: SymbolId = 31;
    const one_maybe_mll_content: SymbolId = 32;
    const two_DIGIT: SymbolId = 33;
    const four_DIGIT: SymbolId = 34;
    const three_apostrophe: SymbolId = 35;
    const four_HEXDIG: SymbolId = 36;
    const eight_HEXDIG: SymbolId = 37;
    const ALPHA: SymbolId = 38;
    const apostrophe: SymbolId = 39;
    const array_close: SymbolId = 40;
    const array_open: SymbolId = 41;
    const array_sep: SymbolId = 42;
    const array_table_close: SymbolId = 43;
    const array_table_open: SymbolId = 44;
    const array_table: SymbolId = 45;
    const array_values: SymbolId = 46;
    const array: SymbolId = 47;
    const basic_char: SymbolId = 48;
    const basic_string: SymbolId = 49;
    const basic_unescaped: SymbolId = 50;
    const bin_int: SymbolId = 51;
    const bin_prefix: SymbolId = 52;
    const boolean: SymbolId = 53;
    const comment_start_symbol: SymbolId = 54;
    const comment: SymbolId = 55;
    const date_fullyear: SymbolId = 56;
    const date_mday: SymbolId = 57;
    const date_month: SymbolId = 58;
    const date_time: SymbolId = 59;
    const dec_int: SymbolId = 60;
    const decimal_point: SymbolId = 61;
    const DIGIT: SymbolId = 62;
    const DIGIT_: SymbolId = 63;
    const digitzero_one_: SymbolId = 64;
    const digitzero_one_underscore: SymbolId = 65;
    const digitzero_seven_: SymbolId = 66;
    const digitzero_seven_underscore: SymbolId = 67;
    const digitone_nine_: SymbolId = 68;
    const escape_seq_char: SymbolId = 69;
    const escape: SymbolId = 70;
    const escaped: SymbolId = 71;
    const exp: SymbolId = 72;
    const expression: SymbolId = 73;
    const expressions: SymbolId = 74;
    const sym_false: SymbolId = 75;
    const float_exp_part: SymbolId = 76;
    const float_int_part: SymbolId = 77;
    const float: SymbolId = 78;
    const float_rest: SymbolId = 79;
    const frac: SymbolId = 80;
    const full_date: SymbolId = 81;
    const full_time: SymbolId = 82;
    const hex_int: SymbolId = 83;
    const hex_prefix: SymbolId = 84;
    const HEXDIG: SymbolId = 85;
    const HEXDIG_: SymbolId = 86;
    const inf: SymbolId = 87;
    const inline_table_close: SymbolId = 88;
    const inline_table_keyvals: SymbolId = 89;
    const inline_table_open: SymbolId = 90;
    const inline_table_sep: SymbolId = 91;
    const inline_table: SymbolId = 92;
    const integer: SymbolId = 93;
    const keyval_sep: SymbolId = 94;
    const keyval: SymbolId = 95;
    const literal_char: SymbolId = 96;
    const literal_string: SymbolId = 97;
    const local_date_time: SymbolId = 98;
    const local_date: SymbolId = 99;
    const local_time: SymbolId = 100;
    const minus: SymbolId = 101;
    const ml_basic_body: SymbolId = 102;
    const ml_basic_string_delim: SymbolId = 103;
    const ml_basic_string: SymbolId = 104;
    const ml_literal_body: SymbolId = 105;
    const ml_literal_string_delim: SymbolId = 106;
    const ml_literal_string: SymbolId = 107;
    const mlb_char: SymbolId = 108;
    const mlb_content: SymbolId = 109;
    const mlb_escaped_nl: SymbolId = 110;
    const mlb_quotes: SymbolId = 111;
    const mlb_unescaped: SymbolId = 112;
    const mll_char: SymbolId = 113;
    const mll_content: SymbolId = 114;
    const mll_quotes: SymbolId = 115;
    const nan: SymbolId = 116;
    const newline: SymbolId = 117;
    const non_ascii: SymbolId = 118;
    const non_eol: SymbolId = 119;
    const oct_int: SymbolId = 120;
    const oct_prefix: SymbolId = 121;
    const offset_date_time: SymbolId = 122;
    const partial_time: SymbolId = 123;
    const plus: SymbolId = 124;
    const quotation_mark: SymbolId = 125;
    const quoted_key: SymbolId = 126;
    const simple_key: SymbolId = 127;
    const special_float: SymbolId = 128;
    const std_table_close: SymbolId = 129;
    const std_table_open: SymbolId = 130;
    const std_table: SymbolId = 131;
    const string: SymbolId = 132;
    const table: SymbolId = 133;
    const time_delim: SymbolId = 134;
    const time_hour: SymbolId = 135;
    const time_minute: SymbolId = 136;
    const time_numoffset: SymbolId = 137;
    const time_offset: SymbolId = 138;
    const time_secfrac: SymbolId = 139;
    const time_second: SymbolId = 140;
    const toml: SymbolId = 141;
    const sym_true: SymbolId = 142;
    const underscore: SymbolId = 143;
    const unquoted_key_char: SymbolId = 144;
    const unquoted_key: SymbolId = 145;
    const uns_dec_int_rest: SymbolId = 146;
    const unsigned_dec_int: SymbolId = 147;
    const val: SymbolId = 148;
    const wschar_nl: SymbolId = 149;
    const wschar: SymbolId = 150;
    const wscn: SymbolId = 151;
    const zero_prefixable_int: SymbolId = 152;

    // Terminal symbols
    const t_minus: SymbolId = 153; //    Exact('-')
    const t_space: SymbolId = 154; //    Exact(' ')
    const t_bang: SymbolId = 155; //    Exact('!')
    const t_dquot: SymbolId = 156; //    Exact('"')
    const t_hash: SymbolId = 157; //    Exact('#')
    const t_comma: SymbolId = 158; //    Exact(',')
    const t_dot: SymbolId = 159; //    Exact('.')
    const t_colon: SymbolId = 160; //    Exact(':')
    const t_sq_open: SymbolId = 161; //    Exact('[')
    const t_tick: SymbolId = 162; //    Exact('\'')
    const t_backslash: SymbolId = 163; //    Exact('\\')
    const t_tab: SymbolId = 164; //    Exact('\t')
    const t_nl: SymbolId = 165; //    Exact('\x0A')
    const t_cr: SymbolId = 166; //    Exact('\x0D')
    const t_22: SymbolId = 167; //    Exact('\x22')
    const t_2d: SymbolId = 168; //    Exact('\x2D')
    const t_55: SymbolId = 169; //    Exact('\x55')
    const t_5c: SymbolId = 170; //    Exact('\x5C')
    const t_5f: SymbolId = 171; //    Exact('\x5F')
    const t_62: SymbolId = 172; //    Exact('\x62')
    const t_66: SymbolId = 173; //    Exact('\x66')
    const t_6e: SymbolId = 174; //    Exact('\x6E')
    const t_72: SymbolId = 175; //    Exact('\x72')
    const t_74: SymbolId = 176; //    Exact('\x74')
    const t_75: SymbolId = 177; //    Exact('\x75')
    const t_sq_close: SymbolId = 178; //    Exact(']')
    const t_underscore: SymbolId = 179; //    Exact('_')
    const t_curly_open: SymbolId = 180; //    Exact('{')
    const t_curly_close: SymbolId = 181; //    Exact('}')
    const t_plus: SymbolId = 182; //    Exact('+')
    const t_equal: SymbolId = 183; //    Exact('=')
    const t_zero: SymbolId = 184; //    Exact('0')
    const t_a: SymbolId = 185; //    Exact('a')
    const t_b: SymbolId = 186; //    Exact('b')
    const t_e: SymbolId = 187; //    Exact('e')
    const t_f: SymbolId = 188; //    Exact('f')
    const t_i: SymbolId = 189; //    Exact('i')
    const t_l: SymbolId = 190; //    Exact('l')
    const t_n: SymbolId = 191; //    Exact('n')
    const t_o: SymbolId = 192; //    Exact('o')
    const t_r: SymbolId = 193; //    Exact('r')
    const t_s: SymbolId = 194; //    Exact('s')
    const t_lc_t: SymbolId = 195; //    Exact('t')
    const t_uc_t: SymbolId = 196; //    Exact('T')
    const t_u: SymbolId = 197; //    Exact('u')
    const t_x: SymbolId = 198; //    Exact('x')
    const t_lc_z: SymbolId = 199; //    Exact('z')
    const t_uc_z: SymbolId = 200; //    Exact('Z')
    const t_80_d7ff: SymbolId = 201; //    Range('\u{80}', '\u{D7FF}')
    const t_e000_10ffff: SymbolId = 202; //    Range('\u{E000}', '\u{10FFFF}')
    const t_20_26: SymbolId = 203; //    Range('\x20', '\x26')
    const t_20_7f: SymbolId = 204; //    Range('\x20', '\x7F')
    const t_23_5b: SymbolId = 205; //    Range('\x23', '\x5B')
    const t_28_7e: SymbolId = 206; //    Range('\x28', '\x7E')
    const t_5d_7e: SymbolId = 207; //    Range('\x5D', '\x7E')
    const t_0_1: SymbolId = 208; //    Range('0', '1')
    const t_0_7: SymbolId = 209; //    Range('0', '7')
    const t_0_9: SymbolId = 210; //    Range('0', '9')
    const t_1_9: SymbolId = 211; //    Range('1', '9')
    const t_lc_a_f: SymbolId = 212; //    Range('a', 'f')
    const t_uc_a_f: SymbolId = 213; //    Range('A', 'F')
    const t_uc_a_z: SymbolId = 214; //    Range('A', 'Z')
    const t_lc_a_z: SymbolId = 215; //    Range('a', 'z')

    const nt_names: [&str; 153] = [
        "~~~ERROR~~~~",
        "non_eols",
        "maybe_comment",
        "maybe_inline_table_keyvals",
        "maybe_inline_table_sepinline_table_keyvals",
        "maybe_array_values",
        "maybe_array_sep",
        "ws_comment_newline",
        "maybe_time_secfrac",
        "sign",
        "hex_int_rest",
        "oct_int_rest",
        "bin_int_rest",
        "maybe_exp",
        "zero_prefixable_int_rest",
        "basic_chars",
        "maybe_mlb_quotes",
        "mlb_quotes_content",
        "maybe_mlb_content",
        "wschar_nls",
        "maybe_literal_char",
        "maybe_mll_content",
        "maybe_mll_quotes",
        "some_mll_quotes_content",
        "ws",
        "dotted_key_rest",
        "dotted_key",
        "dot_sep",
        "key",
        "one_star_two_quotation_mark",
        "one_star_DIGIT",
        "one_maybe_mlb_content",
        "one_maybe_mll_content",
        "two_DIGIT",
        "four_DIGIT",
        "three_apostrophe",
        "four_HEXDIG",
        "eight_HEXDIG",
        "ALPHA",
        "apostrophe",
        "array_close",
        "array_open",
        "array_sep",
        "array_table_close",
        "array_table_open",
        "array_table",
        "array_values",
        "array",
        "basic_char",
        "basic_string",
        "basic_unescaped",
        "bin_int",
        "bin_prefix",
        "boolean",
        "comment_start_symbol",
        "comment",
        "date_fullyear",
        "date_mday",
        "date_month",
        "date_time",
        "dec_int",
        "decimal_point",
        "DIGIT",
        "DIGIT_",
        "digitzero_one_",
        "digitzero_one_underscore",
        "digitzero_seven_",
        "digitzero_seven_underscore",
        "digitone_nine_",
        "escape_seq_char",
        "escape",
        "escaped",
        "exp",
        "expression",
        "expressions",
        "sym_false",
        "float_exp_part",
        "float_int_part",
        "float",
        "float_rest",
        "frac",
        "full_date",
        "full_time",
        "hex_int",
        "hex_prefix",
        "HEXDIG",
        "HEXDIG_",
        "inf",
        "inline_table_close",
        "inline_table_keyvals",
        "inline_table_open",
        "inline_table_sep",
        "inline_table",
        "integer",
        "keyval_sep",
        "keyval",
        "literal_char",
        "literal_string",
        "local_date_time",
        "local_date",
        "local_time",
        "minus",
        "ml_basic_body",
        "ml_basic_string_delim",
        "ml_basic_string",
        "ml_literal_body",
        "ml_literal_string_delim",
        "ml_literal_string",
        "mlb_char",
        "mlb_content",
        "mlb_escaped_nl",
        "mlb_quotes",
        "mlb_unescaped",
        "mll_char",
        "mll_content",
        "mll_quotes",
        "nan",
        "newline",
        "non_ascii",
        "non_eol",
        "oct_int",
        "oct_prefix",
        "offset_date_time",
        "partial_time",
        "plus",
        "quotation_mark",
        "quoted_key",
        "simple_key",
        "special_float",
        "std_table_close",
        "std_table_open",
        "std_table",
        "string",
        "table",
        "time_delim",
        "time_hour",
        "time_minute",
        "time_numoffset",
        "time_offset",
        "time_secfrac",
        "time_second",
        "toml",
        "sym_true",
        "underscore",
        "unquoted_key_char",
        "unquoted_key",
        "uns_dec_int_rest",
        "unsigned_dec_int",
        "val",
        "wschar_nl",
        "wschar",
        "wscn",
        "zero_prefixable_int",
    ];

    use sesd::char::CharMatcher::*;
    const terminals: [sesd::char::CharMatcher; 63] = [
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

    const rules: [(SymbolId, &[SymbolId]); 239] = [
        (ALPHA, &[t_uc_a_z]),
        (ALPHA, &[t_lc_a_z]),
        (DIGIT, &[t_0_9]),
        (HEXDIG, &[DIGIT]),
        (HEXDIG, &[t_uc_a_f]),
        (HEXDIG, &[t_lc_a_f]),
        (four_HEXDIG, &[HEXDIG, HEXDIG, HEXDIG, HEXDIG]),
        (eight_HEXDIG, &[four_HEXDIG, four_HEXDIG]),
        (ws, &[wschar, ws]),
        (wschar, &[t_space]),
        (wschar, &[t_tab]),
        (newline, &[t_nl]),
        (newline, &[t_cr, t_nl]),
        (comment_start_symbol, &[t_hash]),
        (non_ascii, &[t_80_d7ff]),
        (non_ascii, &[t_e000_10ffff]),
        (non_eol, &[t_tab]),
        (non_eol, &[t_20_7f]),
        (non_eol, &[non_ascii]),
        (comment, &[comment_start_symbol, non_eols]),
        (non_eols, &[non_eol, non_eols]),
        (maybe_comment, &[comment]),
        (table, &[std_table]),
        (table, &[array_table]),
        (std_table, &[std_table_open, key, std_table_close]),
        (std_table_open, &[t_sq_open, ws]),
        (std_table_close, &[ws, t_sq_close]),
        (
            inline_table,
            &[
                inline_table_open,
                maybe_inline_table_keyvals,
                inline_table_close,
            ],
        ),
        (inline_table_open, &[t_curly_open, ws]),
        (inline_table_close, &[ws, t_curly_close]),
        (inline_table_sep, &[ws, t_comma, ws]),
        (maybe_inline_table_keyvals, &[inline_table_keyvals]),
        (
            inline_table_keyvals,
            &[keyval, maybe_inline_table_sepinline_table_keyvals],
        ),
        (
            maybe_inline_table_sepinline_table_keyvals,
            &[inline_table_sep, inline_table_keyvals],
        ),
        (array_table, &[array_table_open, key, array_table_close]),
        (array_table_open, &[t_sq_open, t_sq_open, ws]),
        (array_table_close, &[ws, t_sq_close, t_sq_close]),
        (
            array,
            &[
                array_open,
                maybe_array_values,
                ws_comment_newline,
                array_close,
            ],
        ),
        (maybe_array_values, &[array_values]),
        (array_open, &[t_sq_open]),
        (array_close, &[t_sq_close]),
        (
            array_values,
            &[ws_comment_newline, val, ws, array_sep, array_values],
        ),
        (
            array_values,
            &[ws_comment_newline, val, ws, maybe_array_sep],
        ),
        (maybe_array_sep, &[array_sep]),
        (array_sep, &[t_comma]),
        (ws_comment_newline, &[wscn, ws_comment_newline]),
        (wscn, &[wschar]),
        (wscn, &[maybe_comment, newline]),
        (maybe_comment, &[comment]),
        (date_time, &[offset_date_time]),
        (date_time, &[local_date_time]),
        (date_time, &[local_date]),
        (date_time, &[local_time]),
        (date_fullyear, &[four_DIGIT]),
        (four_DIGIT, &[two_DIGIT, two_DIGIT]),
        (two_DIGIT, &[DIGIT, DIGIT]),
        (date_month, &[two_DIGIT]),
        (date_mday, &[two_DIGIT]),
        (time_delim, &[t_uc_t]),
        (time_delim, &[t_lc_t]),
        (time_delim, &[t_space]),
        (time_hour, &[two_DIGIT]),
        (time_minute, &[two_DIGIT]),
        (time_second, &[two_DIGIT]),
        (time_secfrac, &[t_comma, one_star_DIGIT]),
        (one_star_DIGIT, &[DIGIT, one_star_DIGIT]),
        (one_star_DIGIT, &[DIGIT]),
        (time_numoffset, &[sign, time_hour, t_colon, time_minute]),
        (time_offset, &[t_uc_z]),
        (time_offset, &[t_lc_z]),
        (time_offset, &[time_numoffset]),
        (
            partial_time,
            &[
                time_hour,
                t_colon,
                time_minute,
                t_colon,
                time_second,
                maybe_time_secfrac,
            ],
        ),
        (maybe_time_secfrac, &[time_secfrac]),
        (
            full_date,
            &[
                date_fullyear,
                t_underscore,
                date_month,
                t_underscore,
                date_mday,
            ],
        ),
        (full_time, &[partial_time, time_offset]),
        (offset_date_time, &[full_date, time_delim, full_time]),
        (local_date_time, &[full_date, time_delim, partial_time]),
        (local_date, &[full_date]),
        (local_time, &[partial_time]),
        (integer, &[dec_int]),
        (integer, &[hex_int]),
        (integer, &[oct_int]),
        (integer, &[bin_int]),
        (minus, &[t_minus]),
        (plus, &[t_plus]),
        (underscore, &[t_underscore]),
        (digitone_nine_, &[t_1_9]),
        (digitzero_seven_, &[t_0_7]),
        (digitzero_one_, &[t_0_1]),
        (hex_prefix, &[t_zero, t_x]),
        (oct_prefix, &[t_zero, t_o]),
        (bin_prefix, &[t_zero, t_b]),
        (dec_int, &[sign, unsigned_dec_int]),
        (sign, &[minus]),
        (sign, &[plus]),
        (unsigned_dec_int, &[DIGIT]),
        (unsigned_dec_int, &[digitone_nine_, uns_dec_int_rest]),
        (uns_dec_int_rest, &[DIGIT_, uns_dec_int_rest]),
        (uns_dec_int_rest, &[DIGIT_]),
        (DIGIT_, &[DIGIT]),
        (DIGIT_, &[underscore, DIGIT]),
        (hex_int, &[hex_prefix, HEXDIG, hex_int_rest]),
        (hex_int_rest, &[HEXDIG_, hex_int_rest]),
        (HEXDIG_, &[HEXDIG]),
        (HEXDIG_, &[underscore, HEXDIG]),
        (oct_int, &[oct_prefix, digitzero_seven_, oct_int_rest]),
        (oct_int_rest, &[digitzero_seven_, oct_int_rest]),
        (digitzero_seven_underscore, &[digitzero_seven_]),
        (digitzero_seven_underscore, &[underscore, digitzero_seven_]),
        (bin_int, &[bin_prefix, digitzero_one_, bin_int_rest]),
        (bin_int_rest, &[digitzero_one_underscore, bin_int_rest]),
        (digitzero_one_underscore, &[digitzero_one_]),
        (digitzero_one_underscore, &[underscore, digitzero_one_]),
        (float, &[float_int_part, float_rest]),
        (float, &[special_float]),
        (float_rest, &[exp]),
        (float_rest, &[frac, maybe_exp]),
        (maybe_exp, &[exp]),
        (float_int_part, &[dec_int]),
        (frac, &[decimal_point, zero_prefixable_int]),
        (decimal_point, &[t_dot]),
        (zero_prefixable_int, &[DIGIT, zero_prefixable_int_rest]),
        (
            zero_prefixable_int_rest,
            &[DIGIT_, zero_prefixable_int_rest],
        ),
        (exp, &[t_e, float_exp_part]),
        (float_exp_part, &[sign, zero_prefixable_int]),
        (special_float, &[sign, inf]),
        (special_float, &[sign, nan]),
        (inf, &[t_i, t_n, t_f]),
        (nan, &[t_n, t_a, t_n]),
        (boolean, &[sym_true]),
        (boolean, &[sym_false]),
        (sym_true, &[t_lc_t, t_r, t_u, t_e]),
        (sym_false, &[t_f, t_a, t_l, t_s, t_e]),
        (string, &[ml_basic_string]),
        (string, &[basic_string]),
        (string, &[ml_literal_string]),
        (string, &[literal_string]),
        (basic_string, &[quotation_mark, basic_chars, quotation_mark]),
        (basic_chars, &[basic_char, basic_chars]),
        (quotation_mark, &[t_dquot]),
        (basic_char, &[basic_unescaped]),
        (basic_char, &[escaped]),
        (basic_unescaped, &[wschar]),
        (basic_unescaped, &[t_bang]),
        (basic_unescaped, &[t_23_5b]),
        (basic_unescaped, &[t_5d_7e]),
        (basic_unescaped, &[non_ascii]),
        (escaped, &[escape, escape_seq_char]),
        (escape, &[t_backslash]),
        (escape_seq_char, &[t_22]),
        (escape_seq_char, &[t_5c]),
        (escape_seq_char, &[t_62]),
        (escape_seq_char, &[t_66]),
        (escape_seq_char, &[t_6e]),
        (escape_seq_char, &[t_72]),
        (escape_seq_char, &[t_74]),
        (escape_seq_char, &[t_75, four_HEXDIG]),
        (escape_seq_char, &[t_55, eight_HEXDIG]),
        (
            ml_basic_string,
            &[ml_basic_string_delim, ml_basic_body, ml_basic_string_delim],
        ),
        (
            ml_basic_string_delim,
            &[quotation_mark, quotation_mark, quotation_mark],
        ),
        (
            ml_basic_body,
            &[maybe_mlb_content, mlb_quotes_content, maybe_mlb_quotes],
        ),
        (maybe_mlb_quotes, &[mlb_quotes]),
        (one_maybe_mlb_content, &[mlb_content, one_maybe_mlb_content]),
        (one_maybe_mlb_content, &[mlb_content]),
        (
            mlb_quotes_content,
            &[mlb_quotes, one_maybe_mlb_content, mlb_quotes_content],
        ),
        (maybe_mlb_content, &[mlb_content, maybe_mlb_content]),
        (mlb_content, &[mlb_char]),
        (mlb_content, &[newline]),
        (mlb_content, &[mlb_escaped_nl]),
        (mlb_char, &[mlb_unescaped]),
        (mlb_char, &[escaped]),
        (mlb_quotes, &[one_star_two_quotation_mark]),
        (mlb_unescaped, &[wschar]),
        (mlb_unescaped, &[t_bang]),
        (mlb_unescaped, &[t_23_5b]),
        (mlb_unescaped, &[t_5d_7e]),
        (mlb_unescaped, &[non_ascii]),
        (mlb_escaped_nl, &[escape, ws, newline, wschar_nls]),
        (wschar_nl, &[wschar]),
        (wschar_nl, &[newline]),
        (wschar_nls, &[wschar_nl, wschar_nls]),
        (one_star_two_quotation_mark, &[t_dquot, t_dquot]),
        (one_star_two_quotation_mark, &[t_dquot]),
        (
            literal_string,
            &[apostrophe, maybe_literal_char, apostrophe],
        ),
        (maybe_literal_char, &[literal_char, maybe_literal_char]),
        (apostrophe, &[t_tick]),
        (literal_char, &[t_tab]),
        (literal_char, &[t_20_26]),
        (literal_char, &[t_28_7e]),
        (literal_char, &[non_ascii]),
        (
            ml_literal_string,
            &[
                ml_literal_string_delim,
                ml_literal_body,
                ml_literal_string_delim,
            ],
        ),
        (ml_literal_string_delim, &[three_apostrophe]),
        (
            ml_literal_body,
            &[maybe_mll_content, some_mll_quotes_content, maybe_mll_quotes],
        ),
        (three_apostrophe, &[apostrophe, apostrophe, apostrophe]),
        (maybe_mll_content, &[mll_content, maybe_mll_content]),
        (one_maybe_mll_content, &[mll_content, one_maybe_mll_content]),
        (one_maybe_mll_content, &[mll_content]),
        (maybe_mll_quotes, &[mll_quotes]),
        (
            some_mll_quotes_content,
            &[mll_quotes, one_maybe_mll_content, some_mll_quotes_content],
        ),
        (mll_content, &[mll_char]),
        (mll_content, &[newline]),
        (mll_char, &[t_tab]),
        (mll_char, &[t_20_26]),
        (mll_char, &[t_28_7e]),
        (mll_char, &[non_ascii]),
        (mll_quotes, &[apostrophe]),
        (mll_quotes, &[apostrophe, apostrophe]),
        (toml, &[expression]),
        (toml, &[expression, expressions]),
        (expressions, &[newline, expression, expressions]),
        (expressions, &[newline]),
        (expression, &[ws, maybe_comment]),
        (expression, &[ws, keyval, ws, maybe_comment]),
        (expression, &[ws, table, ws, maybe_comment]),
        (keyval, &[key, keyval_sep, val]),
        (key, &[simple_key]),
        (key, &[dotted_key]),
        (simple_key, &[quoted_key]),
        (simple_key, &[unquoted_key]),
        (unquoted_key, &[unquoted_key_char, unquoted_key]),
        (unquoted_key, &[unquoted_key_char]),
        (unquoted_key_char, &[ALPHA]),
        (unquoted_key_char, &[DIGIT]),
        (unquoted_key_char, &[t_2d]),
        (unquoted_key_char, &[t_5f]),
        (quoted_key, &[basic_string]),
        (quoted_key, &[literal_string]),
        (dotted_key, &[simple_key, dotted_key_rest]),
        (dotted_key_rest, &[dot_sep, simple_key, dotted_key_rest]),
        (dotted_key_rest, &[dot_sep, simple_key]),
        (dot_sep, &[ws, t_dot, ws]),
        (keyval_sep, &[ws, t_equal, ws]),
        (val, &[string]),
        (val, &[boolean]),
        (val, &[array]),
        (val, &[inline_table]),
        (val, &[date_time]),
        (val, &[float]),
        (val, &[integer]),
    ];

    pub struct Grammar {}

    use sesd::{char::CharMatcher, CompiledGrammar};
    impl CompiledGrammar<char, CharMatcher> for Grammar {
        fn start_symbol(&self) -> SymbolId {
            toml
        }

        fn rules_count(&self) -> usize {
            rules.len()
        }

        fn lhs(&self, rule: usize) -> SymbolId {
            rules[rule].0
        }

        fn rhs(&self, rule: usize) -> &[SymbolId] {
            rules[rule].1
        }

        fn nt_name(&self, nt: SymbolId) -> &str {
            nt_names[nt as usize]
        }

        fn nt_count(&self) -> SymbolId {
            nt_names.len() as SymbolId
        }

        fn t_count(&self) -> SymbolId {
            terminals.len() as SymbolId
        }

        fn nt_empty_count(&self) -> SymbolId {
            number_of_empty_nts
        }

        fn matcher(&self, term: SymbolId) -> CharMatcher {
            terminals[term as usize].clone()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use sesd::{char::CharMatcher, Parser, Verdict};

    #[test]
    fn table() {
        let compiled_grammar = super::CargoToml::Grammar {};

        let mut parser =
            Parser::<char, CharMatcher, super::CargoToml::Grammar>::new(compiled_grammar);
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
