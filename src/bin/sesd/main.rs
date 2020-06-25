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

#[macro_use]
extern crate log;
extern crate flexi_logger;

use libc;
use std::fs::OpenOptions;
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use pancurses::{endwin, initscr, noecho, Input, Window};
use structopt::StructOpt;

use sesd::{char::CharMatcher, CstIterItem, CstIterItemNode, SymbolId, SyncBlock};

mod cargo_toml;
mod style_sheet;
use style_sheet::{LookedUp, Style};

#[derive(Debug, StructOpt)]
#[structopt(name = "sesd", about = "Syntax directed text editor")]
struct CommandLine {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

type Block = SyncBlock<char, CharMatcher>;

/// Syntactical element to be displayed
struct SynElement {
    attr: pancurses::Attributes,
    text: String,
    /// Index into block where the element starts
    start: usize,
}

/// All state of the edit app
struct App {
    /// Editable block of text in memory
    block: Block,

    /// Style sheet
    style_sheet: style_sheet::StyleSheet,

    /// Cache for rendering syntax items
    ///
    /// Outer dimension is per line, inner dimension is a syntactical element.
    document: Vec<Vec<SynElement>>,

    /// Cursor position in the document: line
    cursor_doc_line: usize,

    /// Cursor position on the screen: line
    cursor_win_line: usize,

    /// Cursor positon in the document and on screen
    cursor_col: usize,

    /// Last error message
    error: String,
}

enum AppCmd {
    /// Nothing to do.
    Nothing,

    /// Quit the app
    Quit,

    /// Buffer has been changed, redraw document and cursor
    Document,

    /// Cursor position has been changed, redraw screen if scrolled
    Cursor,
}

impl App {
    /// Load the input file into the block if it exists.
    ///
    /// Internal helper method that returns the error message
    fn load_input_internal(&mut self, cmd_line: &CommandLine) -> std::io::Result<()> {
        // Delete everything in case this is used for reverting all changes
        self.block.clear();

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_EXCL)
            .open(&cmd_line.input)?;

        let mut temp = String::new();
        let _ = file.read_to_string(&mut temp)?;

        self.block.append_iter(temp.chars());
        self.block.move_start();

        Ok(())
    }

    /// Set error message on Err, clear it on Ok
    fn set_error<T, E>(&mut self, res: Result<T, E>)
    where
        E: std::error::Error,
    {
        match res {
            Err(e) => self.error = e.to_string(),
            Ok(_) => self.error.clear(),
        }
    }

    /// Load the input file into the block if it exists. Sets error message
    fn load_input(&mut self, cmd_line: &CommandLine) {
        let res = self.load_input_internal(cmd_line);
        self.set_error(res);
    }

    /// Process the input character
    ///
    /// Return true if a redraw is needed
    fn handle_input(&mut self, ch: Input) -> AppCmd {
        match ch {
            Input::KeyLeft => {
                self.block.move_backward(1);
                AppCmd::Cursor
            }
            Input::KeyRight => {
                self.block.move_forward(1);
                AppCmd::Cursor
            }
            Input::KeyHome => {
                self.block.skip_backward(sesd::char::start_of_line);
                AppCmd::Cursor
            }
            Input::KeyEnd => {
                self.block.skip_forward(sesd::char::end_of_line);
                AppCmd::Cursor
            }
            Input::KeyUp => {
                let col = self.cursor_col;
                if let Some(this_start) = self
                    .block
                    .search_backward(self.block.cursor(), sesd::char::start_of_line)
                {
                    if this_start > 0 {
                        let prev_end = this_start - 1;
                        if let Some(prev_start) = self
                            .block
                            .search_backward(prev_end, sesd::char::start_of_line)
                        {
                            if prev_start <= prev_end && prev_end < self.block.cursor() {
                                self.block.set_cursor(if prev_start + col <= prev_end {
                                    prev_start + col
                                } else {
                                    prev_end
                                });
                                return AppCmd::Cursor;
                            }
                        }
                    }
                }
                AppCmd::Nothing
            }
            Input::KeyDown => {
                let col = self.cursor_col;
                if let Some(this_end) = self
                    .block
                    .search_forward(self.block.cursor(), sesd::char::end_of_line)
                {
                    let next_start = this_end + 1;
                    if let Some(next_end) = self
                        .block
                        .search_forward(next_start, sesd::char::end_of_line)
                    {
                        if next_start <= next_end && self.block.cursor() < next_start {
                            self.block.set_cursor(if next_start + col <= next_end {
                                next_start + col
                            } else {
                                next_end
                            });
                            return AppCmd::Cursor;
                        }
                    }
                }
                AppCmd::Nothing
            }
            Input::KeyBackspace => {
                if self.block.move_backward(1) {
                    self.block.delete(1);
                }
                AppCmd::Document
            }
            Input::KeyDC => {
                self.block.delete(1);
                AppCmd::Document
            }
            Input::Character(c) => {
                self.block.enter(c);
                AppCmd::Document
            }
            _ => AppCmd::Nothing,
        }
    }

    /// Render a node of the parse tree.
    ///
    /// Return None, if the cursor is not inside this node. Return the line and column of the
    /// document if it is inside.
    fn render_node(
        block: &Block,
        document: &mut Vec<Vec<SynElement>>,
        line_nr: &mut usize,
        line_len: &mut usize,
        width: usize,
        cst_node: CstIterItemNode,
        cursor_index: usize,
        style: Style,
    ) -> Option<(usize, usize)> {
        let mut res = None;

        let mut text = block.span_string(cst_node.start, cst_node.end);
        if style.line_break_before {
            *line_nr += 1;
            document.push(Vec::new());
            *line_len = 0;
        }
        // If text contains a newline, split accordingly, but keep the style.
        //
        // As the last newline is swallowed by the `lines` method, it needs to be
        // treated separately. Thus, always adding a newline ensures that a single newline will
        // result in two lines.
        text.push('\n');
        trace!("text: {:?}", text);
        // The first line is special as it possibly wraps the current line.
        // TODO: Wrap correctly when l is longer than width.
        let mut lines = text.lines();
        if let Some(l) = lines.next() {
            trace!("first line: {:?}", l);
            if (*line_len + l.len()) >= width {
                *line_nr += 1;
                document.push(Vec::new());
                *line_len = 0;
                trace!("wrapped line");
            }
            // If the line is empty, this was just a line break. Since the line break is done in
            // the loop, nothing needs to be done here.
            if !l.is_empty() {
                let se = SynElement {
                    attr: style.attr,
                    text: l.to_string(),
                    start: cst_node.start,
                };
                if se.spans(cursor_index) {
                    res = Some((*line_nr, cursor_index - se.start));
                }
                document[*line_nr].push(se);
            }
        }
        // If there are multiple lines, place the items directly
        for l in lines {
            trace!("another line: {:?}", l);
            // We need a place to put the cursor, thus print a marker.
            let offs = (l.as_ptr() as usize) - (text.as_ptr() as usize);
            let nl = SynElement {
                attr: style.attr,
                text: String::from("¶"),
                start: cst_node.start + offs - 1,
            };
            if nl.spans(cursor_index) {
                res = Some((*line_nr, cursor_index - nl.start));
            }
            document[*line_nr].push(nl);

            // Go to the next line
            *line_nr += 1;
            document.push(Vec::new());

            // If the line contains some text, place it here.
            if !l.is_empty() {
                trace!("Something to place on new line");
                let se = SynElement {
                    attr: style.attr,
                    text: l.to_string(),
                    start: cst_node.start + offs,
                };
                if se.spans(cursor_index) {
                    res = Some((*line_nr, cursor_index - se.start));
                }
                document[*line_nr].push(se);
                *line_len = l.len();
            }
        }
        if style.line_break_after {
            *line_nr += 1;
            document.push(Vec::new());
            *line_len = 0;
        }
        res
    }

    /// Compute the cached cursor position on screen from the cursor position in the block.
    fn update_cursor(&mut self, win: &Window) {
        let old_doc_line = self.cursor_doc_line;
        let cursor_index = self.block.cursor();
        'outer: for (line_nr, line) in self.document.iter().enumerate() {
            let mut line_len = 0;
            for se in line.iter() {
                if se.spans(cursor_index) {
                    self.cursor_doc_line = line_nr;
                    self.cursor_col = line_len + cursor_index - se.start;
                    break 'outer;
                }
                line_len += se.text.chars().count();
            }
        }

        // If the cursor only moved horizontally, just move it
        if old_doc_line == self.cursor_doc_line {
            self.move_cursor(win);
            return;
        }

        let display_height = self.display_height(win);
        // If the document cursor moved forward, check if the win cursor can also be moved forward
        if old_doc_line < self.cursor_doc_line {
            let lines = self.cursor_doc_line - old_doc_line;
            if self.cursor_win_line + lines < display_height {
                self.cursor_win_line += lines;
                self.move_cursor(win);
            } else {
                // Cursor would be outside the display. Place it on the last line and redraw.
                self.cursor_win_line = display_height - 1;
                self.display(win);
            }
            return;
        }

        // Document cursor has moved backwards. Can the win cursor just moved too?
        {
            let lines = old_doc_line - self.cursor_doc_line;
            if self.cursor_win_line >= lines {
                self.cursor_win_line -= lines;
                self.move_cursor(win);
            } else {
                // Cursor would be outside the display. Place it on the first line and redraw.
                self.cursor_win_line = 0;
                self.display(win);
            }
        }
    }

    /// Update the cached syntax tree
    fn update_document(&mut self, width: usize) {
        self.document.clear();

        // Log the parse tree
        if log_enabled!(log::Level::Trace) {
            trace!("update_document CST");
            for cst_node in self.block.cst_iter() {
                match cst_node {
                    sesd::CstIterItem::Parsed(item) => {
                        if item.end - item.start > 0 {
                            trace!(
                                "{}, {}-{}",
                                self.block
                                    .grammar()
                                    .dotted_rule_to_string(&item.dotted_rule)
                                    .unwrap(),
                                item.start,
                                item.end
                            );
                            for n in item.path_iter() {
                                let dr = self.block.parser().dotted_rule(n);
                                trace!(
                                    "   {}",
                                    self.block.grammar().dotted_rule_to_string(&dr).unwrap()
                                );
                            }
                        }
                    }
                    sesd::CstIterItem::Unparsed(start) => {
                        trace!("Unparsed: {} - {}", start, self.block.len());
                    }
                }
            }
        }

        // Compute the cursor position on the fly.
        let cursor_index = self.block.cursor();

        // Traverse the parse tree. If there are items that have no style in the style sheet, draw
        // them and mark until which index, the input has been drawn already. Skip all entries that
        // begin before the current end. This prevents multiple occurrances of the same text.
        let mut line_nr = 0;
        let mut line_len = 0;
        let mut rendered_until = 0;
        trace!("update_document render");
        for cst_node in self.block.cst_iter() {
            match cst_node {
                CstIterItem::Parsed(cst_node) => {
                    trace!(
                        "{}: {}, {}-{}",
                        rendered_until,
                        self.block
                            .grammar()
                            .dotted_rule_to_string(&cst_node.dotted_rule)
                            .unwrap(),
                        cst_node.start,
                        cst_node.end
                    );

                    if cst_node.end != cst_node.start && cst_node.start >= rendered_until {
                        if line_nr == self.document.len() {
                            self.document.push(Vec::new());
                        }

                        // Convert the path to a list of SymbolIds
                        let mut path: Vec<SymbolId> = cst_node
                            .path
                            .0
                            .iter()
                            .map(|n| {
                                let dr = self.block.parser().dotted_rule(&n);
                                self.block.grammar().lhs(dr.rule)
                            })
                            .collect();
                        path.push(self.block.grammar().lhs(cst_node.dotted_rule.rule));

                        let looked_up = self.style_sheet.lookup(&path);
                        trace!("{:?}", looked_up);
                        match looked_up {
                            LookedUp::Parent => {
                                // Do nothing now. Render later.
                            }
                            LookedUp::Found(style) => {
                                // Found an exact match. Render with style.
                                rendered_until = cst_node.end;
                                if let Some((row, col)) = Self::render_node(
                                    &self.block,
                                    &mut self.document,
                                    &mut line_nr,
                                    &mut line_len,
                                    width,
                                    cst_node,
                                    cursor_index,
                                    style,
                                ) {
                                    trace!("Cursor to ({},{})", row, col);
                                    self.cursor_doc_line = row;
                                    self.cursor_col = col;
                                }
                            }
                            LookedUp::Nothing => {
                                // Found nothing. Render with default style.
                                rendered_until = cst_node.end;
                                if let Some((row, col)) = Self::render_node(
                                    &self.block,
                                    &mut self.document,
                                    &mut line_nr,
                                    &mut line_len,
                                    width,
                                    cst_node,
                                    cursor_index,
                                    self.style_sheet.default,
                                ) {
                                    trace!("Cursor to ({},{})", row, col);
                                    self.cursor_doc_line = row;
                                    self.cursor_col = col;
                                }
                            }
                        }
                    }
                }
                CstIterItem::Unparsed(unparsed) => {
                    // TODO: Render the unparsed part with defualt syle
                }
            }
        }
    }

    fn display_height(&self, win: &Window) -> usize {
        let win_height = win.get_max_y() as usize;
        // Leave one line for the error message
        win_height - 1
    }

    /// Display the current state of the app to the window
    fn display(&self, win: &Window) {
        // First document line to display
        let start_doc_line = self.cursor_doc_line - self.cursor_win_line;
        win.clear();
        let display_height = self.display_height(win);
        for win_line in 0..display_height {
            if win_line + start_doc_line < self.document.len() {
                win.mv(win_line as i32, 0);

                for elem in self.document[start_doc_line + win_line].iter() {
                    win.attrset(elem.attr);
                    win.addstr(&elem.text);
                }
            } else {
                break;
            }
        }
        win.attron(pancurses::A_REVERSE);
        win.mvaddnstr(display_height as i32, 0, &self.error, win.get_max_x());
        win.attroff(pancurses::A_REVERSE);
        self.move_cursor(win);
    }

    fn move_cursor(&self, win: &Window) {
        win.mv(self.cursor_win_line as i32, self.cursor_col as i32);
    }
}

const NUL_BYTE_ARRAY: [libc::c_char; 1] = [0];

fn main() {
    // Initialise env_logger first
    let _ = std::env::var("SESD_LOG").and_then(|log| {
        let _ = flexi_logger::Logger::with_str(log)
            .format(flexi_logger::with_thread)
            .log_to_file()
            .start();
        info!("Logging is ready");
        Ok(())
    });

    let cmd_line = CommandLine::from_args();
    eprintln!("{:?}", cmd_line);
    let grammar = cargo_toml::grammar();
    let style_sheet = cargo_toml::style_sheet(&grammar);

    // Set the locale so that UTF-8 codepoints appear correctly
    unsafe { libc::setlocale(libc::LC_ALL, NUL_BYTE_ARRAY[..].as_ptr()) };

    let mut app = App {
        block: Block::new(grammar),
        error: String::new(),
        document: Vec::new(),
        style_sheet,
        cursor_doc_line: 0,
        cursor_win_line: 0,
        cursor_col: 0,
    };

    // Load the file in the buffer if it exists
    app.load_input(&cmd_line);

    let win = initscr();
    noecho();
    win.keypad(true);

    pancurses::set_title(&format!("{} -- sesd", cmd_line.input.to_string_lossy()));
    pancurses::start_color();
    trace!("has_colors: {:?}", pancurses::has_colors());
    trace!("COLORS: {}", pancurses::COLORS());
    trace!("COLOR_PAIRS: {}", pancurses::COLOR_PAIRS());

    // Color pairs
    for f in 0..8 {
        for b in 0..8 {
            let c = (f << 3) + b;
            let r = pancurses::init_pair(c, f, b);
            trace!("init_pair(p={}, f={}, b={}) = {}", c, f, b, r);
        }
    }

    app.update_document(win.get_max_x() as usize);
    app.display(&win);
    win.refresh();

    loop {
        if let Some(input) = win.getch() {
            match app.handle_input(input) {
                AppCmd::Nothing => {
                    // Don't do anything
                }
                AppCmd::Quit => break,
                AppCmd::Cursor => {
                    app.update_cursor(&win);
                    win.refresh();
                }
                AppCmd::Document => {
                    app.update_document(win.get_max_x() as usize);
                    app.display(&win);
                    win.refresh();
                }
            }
        }
    }

    endwin();
}

impl SynElement {
    fn spans(&self, index: usize) -> bool {
        self.start <= index && (index < (self.start + self.text.chars().count()))
    }
}
