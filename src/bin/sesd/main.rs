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

use sesd::{CharMatcher, CstIterItem, CstIterItemNode, SymbolId, SyncBlock};

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

    /// Redisplay the app. This copies the window to the terminal.
    Display,
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

    /// Try to move cursor and return true if it worked
    fn cursor_up(&mut self, n: usize) -> bool {
        let res = self.cursor_doc_line > 0;
        if res {
            let n = if self.cursor_doc_line >= n {
                self.cursor_doc_line -= n;
                n
            } else {
                let n = self.cursor_doc_line;
                self.cursor_doc_line = 0;
                n
            };
            if self.cursor_win_line >= n {
                self.cursor_win_line -= n;
            } else {
                self.cursor_win_line = 0;
            }
        }
        res
    }

    /// Try to move cursor and return true if it worked
    fn cursor_down(&mut self, n: usize, win: &Window) -> bool {
        let res = self.cursor_doc_line < self.document.len();
        if res {
            let n = if self.cursor_doc_line + n <= self.document.len() {
                self.cursor_doc_line += n;
                n
            } else {
                let n = self.document.len() - self.cursor_doc_line;
                self.cursor_doc_line = self.document.len();
                n
            };
            let h = (win.get_max_y() as usize) - 1;
            if self.cursor_win_line + n < h {
                self.cursor_win_line += n;
            } else {
                self.cursor_win_line = n - 1;
            }
        }
        res
    }

    fn line_len(&self) -> usize {
        if self.cursor_doc_line < self.document.len() {
            let mut n = 0;
            for s in self.document[self.cursor_doc_line].iter() {
                n += s.text.len();
            }
            n
        } else {
            0
        }
    }

    /// Move cursor to end of line
    fn cursor_end(&mut self) {
        self.cursor_col = self.line_len();
    }

    /// Process the input character
    ///
    /// Return true if a redraw is needed
    fn handle_input(&mut self, win: &Window, ch: Input) -> AppCmd {
        match ch {
            Input::KeyLeft => {
                if self.cursor_col == 0 {
                    if self.cursor_up(1) {
                        self.cursor_end();
                    }
                } else {
                    self.cursor_col -= 1;
                }
                AppCmd::Display
            }
            Input::KeyRight => {
                let n = self.line_len();
                if self.cursor_col < n {
                    self.cursor_col += 1;
                } else {
                    if self.cursor_down(1, win) {
                        self.cursor_col = 0;
                    }
                }
                AppCmd::Display
            }
            Input::KeyHome => {
                self.cursor_col = 0;
                AppCmd::Display
            }
            Input::KeyEnd => {
                self.cursor_end();
                AppCmd::Display
            }
            Input::KeyUp => {
                if self.cursor_up(1) {
                    let n = self.line_len();
                    if self.cursor_col > n {
                        self.cursor_col = n;
                    }
                }
                AppCmd::Display
            }
            Input::KeyDown => {
                if self.cursor_down(1, win) {
                    let n = self.line_len();
                    if self.cursor_col > n {
                        self.cursor_col = n;
                    }
                }
                AppCmd::Display
            }
            _ => AppCmd::Nothing,
        }
    }

    fn render_node(
        block: &Block,
        document: &mut Vec<Vec<SynElement>>,
        line_nr: &mut usize,
        line_len: &mut usize,
        width: usize,
        cst_node: CstIterItemNode,
        style: Style,
    ) {
        let mut text = block.span_string(cst_node.start, cst_node.end);
        trace!("text: {:?}", text);
        if style.line_break_before {
            *line_nr += 1;
            document.push(Vec::new());
            *line_len = 0;
        }
        // If text contains a newline, split accordingly, but keep the style.
        //
        // As the last newline is swallowed by the `lines` method, it needs to be
        // treated separately.
        text.push('\n');
        //
        // The first line is special as it ends the current line.
        let mut lines = text.lines();
        if let Some(l) = lines.next() {
            *line_len += l.len();
            if *line_len > width {
                *line_nr += 1;
                document.push(Vec::new());
                *line_len = l.len();
            }
            if l.len() != 0 {
                document[*line_nr].push(SynElement {
                    attr: style.attr,
                    text: l.to_string(),
                });
            }
        }
        // If there are multiple lines, place the items directly
        for l in lines {
            trace!("  line: {:?}", l);
            *line_nr += 1;
            document.push(Vec::new());
            document[*line_nr].push(SynElement {
                attr: style.attr,
                text: l.to_string(),
            });
            *line_len = l.len();
        }
        if style.line_break_after {
            *line_nr += 1;
            document.push(Vec::new());
            *line_len = 0;
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
                                Self::render_node(
                                    &self.block,
                                    &mut self.document,
                                    &mut line_nr,
                                    &mut line_len,
                                    width,
                                    cst_node,
                                    style,
                                );
                            }
                            LookedUp::Nothing => {
                                // Found nothing. Render with default style.
                                rendered_until = cst_node.end;
                                Self::render_node(
                                    &self.block,
                                    &mut self.document,
                                    &mut line_nr,
                                    &mut line_len,
                                    width,
                                    cst_node,
                                    self.style_sheet.default,
                                );
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

    /// Display the current state of the app to the window
    fn display(&self, win: &Window) {
        // First document line to display
        let start_doc_line = self.cursor_doc_line - self.cursor_win_line;
        win.clear();
        let win_height = win.get_max_y() as usize;
        let display_height = win_height - 1;
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
        win.mv(self.cursor_win_line as i32, self.cursor_col as i32);
    }
}

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
            match app.handle_input(&win, input) {
                AppCmd::Nothing => {
                    // Don't do anything
                }
                AppCmd::Quit => break,
                AppCmd::Display => {
                    app.display(&win);
                    win.refresh();
                }
            }
        }
    }

    endwin();
}
