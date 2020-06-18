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

use libc;
use std::fs::OpenOptions;
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use pancurses::{endwin, initscr, noecho, Input, Window};
use structopt::StructOpt;

use sesd::{CharMatcher, CstIterItem, SyncBlock};

mod cargo_toml;
mod style_sheet;

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

    /// Process the input character
    ///
    /// Return true if a redraw is needed
    fn handle_input(&mut self, ch: Input) -> AppCmd {
        AppCmd::Nothing
    }

    /// Update the cached syntax tree
    fn update_document(&mut self, width: usize) {
        self.document.clear();

        let mut line_nr = 0;
        let mut line_len = 0;
        let mut cst_iter = self.block.cst_iter();
        for cst_node in cst_iter {
            match cst_node {
                CstIterItem::Parsed(cst_node) => {
                    if line_nr == self.document.len() {
                        self.document.push(Vec::new());
                    }
                    let style = self.style_sheet.lookup(&cst_node.path);
                    let text = self.block.span_string(cst_node.start, cst_node.end);
                    line_len += text.len();
                    if style.line_break_before || line_len > width {
                        line_nr += 1;
                        self.document.push(Vec::new());
                        line_len = text.len();
                    }
                    self.document[line_nr].push(SynElement {
                        attr: style.attr,
                        text,
                    });
                    if style.line_break_after {
                        line_nr += 1;
                        self.document.push(Vec::new());
                        line_len = 0;
                    }
                }
                CstIterItem::Unparsed(unparsed) => {}
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
    let cmd_line = CommandLine::from_args();
    eprintln!("{:?}", cmd_line);

    let mut app = App {
        block: Block::new(cargo_toml::grammar()),
        error: String::new(),
        document: Vec::new(),
        style_sheet: cargo_toml::style_sheet(),
        cursor_doc_line: 0,
        cursor_win_line: 0,
        cursor_col: 0,
    };

    // Load the file in the buffer if it exists
    app.load_input(&cmd_line);
    pancurses::set_title(&format!("{} -- sesd", cmd_line.input.to_string_lossy()));

    let win = initscr();
    noecho();
    win.keypad(true);

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
                AppCmd::Display => {
                    app.display(&win);
                    win.refresh();
                }
            }
        }
    }

    endwin();
}
