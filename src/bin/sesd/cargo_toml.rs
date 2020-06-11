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

//   toml = expression
//   toml = expression expressions
//   expressions = newline expression expressions
//   expressions = newline
//
//   expression = ws maybe_comment
//   expression = ws keyval ws maybe_comment
//   expression = ws table ws maybe_comment
//
//   ;; Whitespace
//
//   ws = wschar ws
//   ws =
//   wschar =  ' '  ; Space
//   wschar =  '\t'  ; Horizontal tab
//
//   ;; Newline
//
//   newline =  %x0A       ; LF
//   newline =  %x0D %x0A  ; CRLF
//
//   ;; Comment
//
//   comment-start-symbol = '#'
//   non-ascii = %x80-D7FF 
//   non-ascii = %xE000-10FFFF
//   non-eol = %x09 
//   non-eol = %x20-7F 
//   non-eol = non-ascii
//
//   comment = comment-start-symbol non-eols
//   non-eols = non-eol non-eols
//   non-eols =
//
//   maybe_comment = comment
//   maybe_comment =
//
//   ;; Key-Value pairs
//
//   keyval = key keyval-sep val
//
//   key = simple-key 
//   key = dotted-key
//   simple-key = quoted-key 
//   simple-key = unquoted-key
//
//   unquoted-key = unquoted-key-char unquoted-key
//   unquoted-key = unquoted-key-char
//   unquoted-key-char = ALPHA 
//   unquoted-key-char = DIGIT 
//   unquoted-key-char = %x2D 
//   unquoted-key-char = %x5F
//   quoted-key = basic-string 
//   quoted-key = literal-string
//   dotted-key = simple-key dotted-key-rest
//   dotted-key-rest = dot-sep simple-key dotted-key-rest
//   dotted-key-rest = dot-sep simple-key
//
//   dot-sep   = ws '.' ws
//   keyval-sep = ws '=' ws
//
//   val = string 
//   val = boolean 
//   val = array 
//   val = inline-table 
//   val = date-time 
//   val = float 
//   val = integer
//
//   ;; String
//
//   string = ml-basic-string 
//   string = basic-string 
//   string = ml-literal-string 
//   string = literal-string
//
//   ;; Basic String
//
//   basic-string = quotation-mark basic-chars quotation-mark
//   basic-chars = basic-char basic-chars
//   basic-chars =
//
//   quotation-mark = '"'
//
//   basic-char = basic-unescaped 
//   basic-char = escaped
//   basic-unescaped = wschar 
//   basic-unescaped = %x21 
//   basic-unescaped = %x23-5B 
//   basic-unescaped = %x5D-7E 
//   basic-unescaped = non-ascii
//   escaped = escape escape-seq-char
//
//   escape = '\\'
//   escape-seq-char = %x22         ; "    quotation mark  U+0022
//   escape-seq-char = %x5C         ; \    reverse solidus U+005C
//   escape-seq-char = %x62         ; b    backspace       U+0008
//   escape-seq-char = %x66         ; f    form feed       U+000C
//   escape-seq-char = %x6E         ; n    line feed       U+000A
//   escape-seq-char = %x72         ; r    carriage return U+000D
//   escape-seq-char = %x74         ; t    tab             U+0009
//   escape-seq-char = %x75 4HEXDIG ; uXXXX                U+XXXX
//   escape-seq-char = %x55 8HEXDIG ; UXXXXXXXX            U+XXXXXXXX
//
//   ;; Multiline Basic String
//
//   ml-basic-string = ml-basic-string-delim ml-basic-body ml-basic-string-delim
//   ml-basic-string-delim = quotation-mark quotation-mark quotation-mark
//   ml-basic-body = *mlb-content *( mlb-quotes 1*mlb-content ) [ mlb-quotes ]
//
//   mlb-content = mlb-char 
//   mlb-content = newline 
//   mlb-content = mlb-escaped-nl
//   mlb-char = mlb-unescaped 
//   mlb-char = escaped
//   mlb-quotes = 1*2quotation-mark
//   mlb-unescaped = wschar 
//   mlb-unescaped = %x21 
//   mlb-unescaped = %x23-5B 
//   mlb-unescaped = %x5D-7E 
//   mlb-unescaped = non-ascii
//   mlb-escaped-nl = escape ws newline *( wschar / newline )
//
//   ;; Literal String
//
//   literal-string = apostrophe *literal-char apostrophe
//
//   apostrophe = %x27 ; ' apostrophe
//
//   literal-char = %x09 
//   literal-char = %x20-26 
//   literal-char = %x28-7E 
//   literal-char = non-ascii
//
//   ;; Multiline Literal String
//
//   ml-literal-string = ml-literal-string-delim ml-literal-body ml-literal-string-delim
//   ml-literal-string-delim = 3apostrophe
//   ml-literal-body = *mll-content some_mll-quotes-content [mll-quotes]
//
//   *mll-content = mll-content *mll-content
//   *mll-content =
//
//   1*mll-content = mll-content 1*mll-content
//   1*mll-content = mll-content
//
//   [mll-quotes] = mll-quotes
//   [mll-quotes] =
//
//   some_mll-quotes-content = mll-quotes 1*mll-content some_mll-quotes-content
//   some_mll-quotes-content = 
//
//   mll-content = mll-char 
//   mll-content = newline
//   mll-char = %x09 
//   mll-char = %x20-26 
//   mll-char = %x28-7E 
//   mll-char = non-ascii
//   mll-quotes = apostrophe
//   mll-quotes = apostrophe apostrophe
//
//   ;; Integer
//
//   integer = dec-int 
//   integer = hex-int 
//   integer = oct-int 
//   integer = bin-int
//
//   minus = %x2D                       ; -
//   plus = %x2B                        ; +
//   underscore = %x5F                  ; _
//   digit1-9 = %x31-39                 ; 1-9
//   digit0-7 = %x30-37                 ; 0-7
//   digit0-1 = %x30-31                 ; 0-1
//
//   hex-prefix = %x30 %x78               ; 0x
//   oct-prefix = %x30 %x6f               ; 0o
//   bin-prefix = %x30 %x62               ; 0b
//
//   dec-int = sign unsigned-dec-int
//   sign = minus
//   sign = plus
//   sign =
//   unsigned-dec-int = DIGIT 
//   unsigned-dec-int = digit1-9 uns-dec-int-rest
//
//   uns-dec-int-rest = DIGIT_  uns-dec-int-rest
//   uns-dec-int-rest = DIGIT_ 
//
//   DIGIT_ = DIGIT 
//   DIGIT_ = underscore DIGIT
//
//   hex-int = hex-prefix HEXDIG hex-int-rest 
//   hex-int-rest = HEXDIG_ hex-int-rest
//   hex-int-rest =
//   HEXDIG_ = HEXDIG 
//   HEXDIG_ = underscore HEXDIG
//
//   oct-int = oct-prefix digit0-7 oct-int-rest
//   oct-int-rest = digit0-7_ oct-int-rest 
//   oct-int-rest = 
//   digit0-7_ = digit0-7
//   digit0-7_ = underscore digit0-7
//
//   bin-int = bin-prefix digit0-1 bin-int-rest
//   bin-int-rest = digit0-1_ bin-int-rest
//   bin-int-rest = 
//   digit0-1_ = digit0-1
//   digit0-1_ = underscore digit0-1
//
//   ;; Float
//
//   float = float-int-part float_rest 
//   float = special-float
//
//   float_rest = exp
//   float_rest = frac [exp]
//
//   [exp] = exp
//   [exp] =
//
//   float-int-part = dec-int
//   frac = decimal-point zero-prefixable-int
//   decimal-point = %x2E               ; .
//   zero-prefixable-int = DIGIT zero-prefixable-int-rest 
//   zero-prefixable-int-rest = DIGIT_ zero-prefixable-int-rest
//   zero-prefixable-int-rest =
//
//   exp = "e" float-exp-part
//   float-exp-part = sign zero-prefixable-int
//
//   special-float = sign inf 
//   special-float = sign nan
//   inf = %x69 %x6e %x66  ; inf
//   nan = %x6e %x61 %x6e  ; nan
//
//   ;; Boolean
//
//   boolean = true
//   boolean = false
//
//   true    = %x74 %x72 %x75.65     ; true
//   false   = %x66 %x61 %x6C %x73 %x65  ; false
//
//   ;; Date and Time (as defined in RFC 3339)
//
//   date-time      = offset-date-time 
//   date-time = local-date-time 
//   date-time = local-date 
//   date-time = local-time
//
//   date-fullyear  = 4DIGIT
//   date-month     = 2DIGIT  ; 01-12
//   date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
//   time-delim     = 'T' 
//   time-delim     = 't' 
//   time-delim = %x20
//   time-hour      = DIGIT DIGIT  ; 00-23
//   time-minute    = DIGIT DIGIT  ; 00-59
//   time-second    = DIGIT DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
//   time-secfrac   = '.' 1*DIGIT
//
//   1*DIGIT = DIGIT 1*DIGIT
//   1*DIGIT = DIGIT
//
//   time-numoffset = sign time-hour ":" time-minute
//   time-offset    = 'Z' 
//   time-offset    = 'z' 
//   time-offset    = time-numoffset
//
//   partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
//   [time-secfrac] = time-secfrac
//   [time-secfrac] =
//   full-date      = date-fullyear "-" date-month "-" date-mday
//   full-time      = partial-time time-offset
//
//   ;; Offset Date-Time
//
//   offset-date-time = full-date time-delim full-time
//
//   ;; Local Date-Time
//
//   local-date-time = full-date time-delim partial-time
//
//   ;; Local Date
//
//   local-date = full-date
//
//   ;; Local Time
//
//   local-time = partial-time
//
//   ;; Array
//
//   array = array-open [array-values] ws-comment-newline array-close
//
//   [array-values] = array-values
//   [array-values] =
//
//   array-open =  %x5B ; [
//   array-close = %x5D ; ]
//
//   array-values =  ws-comment-newline val ws array-sep array-values
//   array-values = ws-comment-newline val ws [array-sep]
//
//   [array-sep] = array-sep
//   [array-sep] =
//
//   array-sep = %x2C  ; , Comma
//
//   ws-comment-newline = wscn ws-comment-newline
//   ws-comment-newline =
//
//   wscn = wschar 
//   wscn = [comment] newline
//   [comment = comment
//   [comment] = 
//
//   ;; Table
//
//   table = std-table 
//   table = array-table
//
//   ;; Standard Table
//
//   std-table = std-table-open key std-table-close
//
//   std-table-open  = %x5B ws     ; [ Left square bracket
//   std-table-close = ws %x5D     ; ] Right square bracket
//
//   ;; Inline Table
//
//   inline-table = inline-table-open [inline-table-keyvals] inline-table-close
//
//   inline-table-open  = %x7B ws     ; {
//   inline-table-close = ws %x7D     ; }
//   inline-table-sep   = ws %x2C ws  ; , Comma
//
//   [inline-table-keyvals] = inline-table-keyvals
//   [inline-table-keyvals] = 
//   inline-table-keyvals = keyval [inline-table-sepinline-table-keyvals]
//
//   [inline-table-sepinline-table-keyvals] = inline-table-sep inline-table-keyvals
//   [inline-table-sepinline-table-keyvals] =
//
//   ;; Array Table
//
//   array-table = array-table-open key array-table-close
//
//   array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
//   array-table-close = ws %x5D.5D  ; ]] Double right square bracket
//
//   ;; Built-in ABNF terms, reproduced here for clarity
//
//   ALPHA = %x41-5A 
//   ALPHA = %x61-7A ; A-Z / a-z
//   DIGIT = %x30-39 ; 0-9
//   HEXDIG = DIGIT 
//   HEXDIG = 'A'
//   HEXDIG = 'B'
//   HEXDIG = 'C'
//   HEXDIG = 'D'
//   HEXDIG = 'E'
//   HEXDIG = 'F'
//   HEXDIG = 'a'
//   HEXDIG = 'b'
//   HEXDIG = 'c'
//   HEXDIG = 'd'
//   HEXDIG = 'e'
//   HEXDIG = 'f'

use sesd::{CompiledGrammar, Grammar};

pub fn grammar() -> CompiledGrammar<char> {
    let mut grammar = Grammar::<char>::new();

    grammar.set_start( "toml".to_string());

    grammar
        .compile()
        .expect("compiling built-in grammar should not fail")
}
