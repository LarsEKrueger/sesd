# Syntax directed editor

This program is meant to showcase the fatures of the interactive parser.

It allows you to edit files in rust's Cargo.toml format and maybe later, files
with user-defined syntax. It will display the parsed text using a stylesheet
(limited to the capabilities of the text terminal). It will also make
suggestions for some syntactical elements (in case of Cargo.toml, the keys and
values as well as the table names).

## Design

The program is designed after the Model-View-Controller pattern. The model is a
single buffer, which is reparsed on every edit operation. The controller is the
App itself. The view is realized by the pancurses libary with a single Window.

The distinction between the three components isn't that strict. The app will
cache some information (e.g. the flattened parse tree) to speed-up scrolling.
If the amount of view-related data becomes too large, the app needs to be
refactored to adhere more strictly to the MVC pattern.

### Display

The parse tree is traversed in pre-order. If a rule in the style sheet matches,
it is used to influence the rendering. If nothing matches, the text is
displayed as-is.

### Error Handling

If the input buffer cannot be parsed, the style sheet is used to select the
non-terminal symbol(s) that will be used to re-start the parsing.

### Input

All input will be added to the buffer as-is. The buffer will be reparsed from
the point of insertion to the end or until no error recovery is possible.

### Prediction

If the cursor is at the start of an item where the style sheet can predict multiple values,
this list is displayed to the user for selection. If the user selects an entry,
this will be inserted as if typed. Afterwards, the buffer will be reparsed
beginning at the first character of the inserted text.

The prediction can always be ignored, in which case the input is processed as
described above.
