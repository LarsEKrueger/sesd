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

#![recursion_limit = "256"]

#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate itertools;

use libc;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[cfg(target_family = "unix")]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(target_family = "windows")]
use std::os::windows::fs::OpenOptionsExt;

use std::path::PathBuf;

use pancurses::{endwin, initscr, noecho, Input, Window};
use structopt::StructOpt;

#[macro_use]
extern crate sesd;

use sesd::{char::CharMatcher, CompiledGrammar, CstIterItem, SymbolId, SynchronousEditor};

mod cargo_toml;
mod look_and_feel;
use look_and_feel::{LookAndFeel, LookedUp, Style};

#[derive(Debug, StructOpt)]
#[structopt(name = "sesd", about = "Syntax directed text editor")]
struct CommandLine {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

type Editor = SynchronousEditor<char, CharMatcher, cargo_toml::cargo_toml::Grammar>;

/// Syntactical element to be displayed
struct SynElement {
    attr: pancurses::Attributes,
    text: String,
    /// Buffer position where the element starts
    start: usize,
}

/// All state of the edit app
struct App {
    /// Editor in memory
    editor: Editor,

    /// Language-specific look and feel
    look_and_feel: LookAndFeel,

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

    /// Predictions
    predictions: Vec<String>,

    /// Selected prediction
    selected_predition: Option<usize>,

    /// Last error message
    error: String,

    /// Name of file being edited
    filename: PathBuf,
}

#[derive(Debug)]
enum AppCmd {
    /// Nothing to do.
    Nothing,

    /// Quit the app
    Quit,

    /// Buffer has been changed, redraw document and cursor
    Document,

    /// Cursor position has been changed, redraw screen if scrolled
    Cursor,

    /// Something else has changed, e.g. selection. Redisplay.
    Display,
}

const PREDICTION_SHOW_RAD: usize = 2;
const MAX_PREDICTIONS_SHOWN: usize = 2 * PREDICTION_SHOW_RAD + 1;

impl App {
    /// Load the input file into the editor if it exists.
    ///
    /// Internal helper method that returns the error message
    fn load_input_internal(&mut self, cmd_line: &CommandLine) -> std::io::Result<()> {
        // Delete everything in case this is used for reverting all changes
        self.editor.clear();

        let mut file = OpenOptions::new();
        file.read(true);

        #[cfg(target_family = "unix")]
        file.custom_flags(libc::O_EXCL);
        #[cfg(target_family = "windows")]
        file.share_mode(0);

        let mut file = file.open(&cmd_line.input)?;

        let mut temp = String::new();
        let _ = file.read_to_string(&mut temp)?;

        self.editor.enter_iter(temp.chars());
        self.editor.move_start();

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

    /// Load the input file into the editor if it exists. Sets error message
    fn load_input(&mut self, cmd_line: &CommandLine) {
        let res = self.load_input_internal(cmd_line);
        self.set_error(res);
    }

    /// Overwrite the given file with the current buffer content
    fn save_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new();
        file.write(true);

        #[cfg(target_family = "unix")]
        file.custom_flags(libc::O_EXCL);
        #[cfg(target_family = "windows")]
        file.share_mode(0);

        let mut file = file.open(&self.filename).map_err(|e| e.to_string())?;
        file.write(self.editor.as_string().as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Process the input character
    ///
    /// Return true if a redraw is needed
    fn handle_input(&mut self, ch: Input) -> AppCmd {
        trace!("{:?}", ch);
        match ch {
            Input::KeyLeft => {
                self.editor.move_backward(1);
                AppCmd::Cursor
            }
            Input::KeyRight => {
                self.editor.move_forward(1);
                AppCmd::Cursor
            }
            Input::KeyHome => {
                self.editor.skip_backward(sesd::char::start_of_line);
                AppCmd::Cursor
            }
            Input::KeyEnd => {
                self.editor.skip_forward(sesd::char::end_of_line);
                AppCmd::Cursor
            }
            Input::KeyUp => {
                let col = self.cursor_col;
                if let Some(this_start) = self
                    .editor
                    .search_backward(self.editor.cursor(), sesd::char::start_of_line)
                {
                    if this_start > 0 {
                        let prev_end = this_start - 1;
                        if let Some(prev_start) = self
                            .editor
                            .search_backward(prev_end, sesd::char::start_of_line)
                        {
                            if prev_start <= prev_end && prev_end < self.editor.cursor() {
                                self.editor.set_cursor(if prev_start + col <= prev_end {
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
                    .editor
                    .search_forward(self.editor.cursor(), sesd::char::end_of_line)
                {
                    let next_start = this_end + 1;
                    if let Some(next_end) = self
                        .editor
                        .search_forward(next_start, sesd::char::end_of_line)
                    {
                        if next_start <= next_end && self.editor.cursor() < next_start {
                            self.editor.set_cursor(if next_start + col <= next_end {
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
                if self.editor.move_backward(1) {
                    self.editor.delete(1);
                }
                AppCmd::Document
            }
            Input::KeyDC => {
                self.editor.delete(1);
                AppCmd::Document
            }

            Input::KeyNPage => {
                if let Some(selected) = &mut self.selected_predition {
                    if *selected + 1 < self.predictions.len() {
                        *selected += 1;
                        return AppCmd::Display;
                    }
                } else {
                    if !self.predictions.is_empty() {
                        self.selected_predition = Some(0);
                        return AppCmd::Display;
                    }
                }
                AppCmd::Nothing
            }

            Input::KeyPPage => {
                if let Some(selected) = &mut self.selected_predition {
                    if *selected > 0 {
                        *selected -= 1;
                        return AppCmd::Display;
                    }
                } else {
                    if !self.predictions.is_empty() {
                        self.selected_predition = Some(0);
                        return AppCmd::Display;
                    }
                }
                AppCmd::Nothing
            }
            Input::KeyBTab | Input::KeySTab => {
                if let Some(selected) = self.selected_predition {
                    self.editor.enter_iter(self.predictions[selected].chars());
                    return AppCmd::Document;
                }
                AppCmd::Nothing
            }

            Input::KeyF2 => {
                self.error = match self.save_file() {
                    Ok(_) => format!(
                        "Successfully saved »{}«.",
                        self.filename.to_string_lossy()
                    ),
                    Err(msg) => format!(
                        "Error saving file »{}«: {}",
                        self.filename.to_string_lossy(),
                        msg
                    ),
                };
                AppCmd::Display
            }

            Input::KeyF10 => AppCmd::Quit,

            Input::Character(c) => {
                self.editor.enter(c);
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
        editor: &Editor,
        document: &mut Vec<Vec<SynElement>>,
        line_nr: &mut usize,
        line_len: &mut usize,
        width: usize,
        start: usize,
        end: usize,
        cursor_index: usize,
        style: &Style,
    ) -> Option<(usize, usize)> {
        let mut res = None;

        let mut text = editor.span_string(start, end);
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
                    start,
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
                start: start + offs - 1,
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
                    start: start + offs,
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

    /// Compute the cached cursor position on screen from the cursor position in the editor.
    ///
    /// Return true if a full redisplay is required. Return false if only the cursor needs to move.
    fn update_cursor(&mut self, win: &Window) -> bool {
        let old_doc_line = self.cursor_doc_line;
        let cursor_index = self.editor.cursor();
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
            return false;
        }

        let display_height = self.display_height(win);
        // If the document cursor moved forward, check if the win cursor can also be moved forward
        if old_doc_line < self.cursor_doc_line {
            let lines = self.cursor_doc_line - old_doc_line;
            if self.cursor_win_line + lines < display_height {
                self.cursor_win_line += lines;
                return false;
            } else {
                // Cursor would be outside the display. Place it on the last line and redraw.
                self.cursor_win_line = display_height - 1;
                return true;
            }
        }

        // Document cursor has moved backwards. Can the win cursor just moved too?
        {
            let lines = old_doc_line - self.cursor_doc_line;
            if self.cursor_win_line >= lines {
                self.cursor_win_line -= lines;
                return false;
            } else {
                // Cursor would be outside the display. Place it on the first line and redraw.
                self.cursor_win_line = 0;
                return true;
            }
        }
    }

    /// Update the cached syntax tree
    fn update_document(&mut self, width: usize) {
        self.document.clear();

        // Log the parse tree
        if log_enabled!(log::Level::Trace) {
            trace!("update_document CST");
            for cst_node in self.editor.cst_iter() {
                match cst_node {
                    sesd::CstIterItem::Parsed(item) => {
                        if item.end - item.start > 0 {
                            trace!(
                                "{}, {}-{}",
                                self.editor
                                    .parser()
                                    .dotted_rule_to_string(&item.dotted_rule)
                                    .unwrap(),
                                item.start,
                                item.end
                            );
                            for n in item.path_iter() {
                                let dr = self.editor.parser().dotted_rule(n);
                                trace!(
                                    "   {}",
                                    self.editor.parser().dotted_rule_to_string(&dr).unwrap()
                                );
                            }
                        }
                    }
                    sesd::CstIterItem::Unparsed(start) => {
                        trace!("Unparsed: {} - {}", start, self.editor.len());
                    }
                }
            }
        }

        // Compute the cursor position on the fly.
        let cursor_index = self.editor.cursor();

        // Traverse the parse tree. If there are items that have no style in the style sheet, draw
        // them and mark until which index the input has been drawn already. Skip all entries that
        // begin before the current end. This prevents multiple occurrances of the same text.
        let mut line_nr = 0;
        let mut line_len = 0;
        let mut rendered_until = 0;
        trace!("update_document render");
        for cst_node in self.editor.cst_iter() {
            match cst_node {
                CstIterItem::Parsed(cst_node) => {
                    trace!(
                        "{}: {}, {}-{}",
                        rendered_until,
                        self.editor
                            .parser()
                            .dotted_rule_to_string(&cst_node.dotted_rule)
                            .unwrap(),
                        cst_node.start,
                        cst_node.end
                    );

                    // If a rule contains a terminal in the middle, and no style has been defined,
                    // it is possible that rendered_until is larger than cst_node.start. Thus, the
                    // buffer needs to be rendered from rendered_until to cst_node.end.
                    if cst_node.end != cst_node.start && cst_node.end > rendered_until {
                        if line_nr == self.document.len() {
                            self.document.push(Vec::new());
                        }

                        // Convert the path to a list of SymbolIds
                        let mut path: Vec<SymbolId> = cst_node
                            .path
                            .0
                            .iter()
                            .map(|n| {
                                let dr = self.editor.parser().dotted_rule(&n);
                                self.editor.grammar().lhs(dr.rule as usize)
                            })
                            .collect();
                        path.push(
                            self.editor
                                .grammar()
                                .lhs(cst_node.dotted_rule.rule as usize),
                        );

                        // Log the lookup path as readable
                        if log_enabled!(log::Level::Trace) {
                            trace!("lookup: {:?}", path);
                            for p in path.iter() {
                                trace!("  {:?}", self.editor.grammar().nt_name(*p));
                            }
                        }

                        let looked_up = self.look_and_feel.lookup(&path);
                        trace!("{:?}", looked_up);
                        match looked_up {
                            LookedUp::Parent => {
                                // Do nothing now. Render later.
                            }
                            LookedUp::Found(style) => {
                                // Found an exact match. Render with style.
                                if let Some((row, col)) = Self::render_node(
                                    &self.editor,
                                    &mut self.document,
                                    &mut line_nr,
                                    &mut line_len,
                                    width,
                                    rendered_until,
                                    cst_node.end,
                                    cursor_index,
                                    style,
                                ) {
                                    trace!("Cursor to ({},{})", row, col);
                                    self.cursor_doc_line = row;
                                    self.cursor_col = col;
                                }
                                rendered_until = cst_node.end;
                            }
                            LookedUp::Nothing => {
                                // Found nothing. Render with default style.
                                if let Some((row, col)) = Self::render_node(
                                    &self.editor,
                                    &mut self.document,
                                    &mut line_nr,
                                    &mut line_len,
                                    width,
                                    rendered_until,
                                    cst_node.end,
                                    cursor_index,
                                    &self.look_and_feel.default,
                                ) {
                                    trace!("Cursor to ({},{})", row, col);
                                    self.cursor_doc_line = row;
                                    self.cursor_col = col;
                                }
                                rendered_until = cst_node.end;
                            }
                        }
                    }
                }
                CstIterItem::Unparsed(_unparsed) => {
                    if line_nr == self.document.len() {
                        self.document.push(Vec::new());
                    }
                    // Render the unparsed part with defualt syle
                    if let Some((row, col)) = Self::render_node(
                        &self.editor,
                        &mut self.document,
                        &mut line_nr,
                        &mut line_len,
                        width,
                        rendered_until,
                        self.editor.len(),
                        cursor_index,
                        &self.look_and_feel.default,
                    ) {
                        trace!("Cursor to ({},{})", row, col);
                        self.cursor_doc_line = row;
                        self.cursor_col = col;
                    }
                    rendered_until = self.editor.len();
                }
            }
        }
    }

    /// Compute the list of predictions at the cursor position
    ///
    /// Return true, if a complete redisplay is required. Return false, if only the cursor position
    /// needs to be changed.
    fn update_prediction(&mut self) -> bool {
        let symbols = self.editor.predictions_at_cursor();
        // Get possible prediction strings from style sheet
        let predictions = symbols
            .iter()
            .flat_map(|sym| self.look_and_feel.predictions(*sym))
            .collect();

        let res = self.predictions != predictions;
        if res {
            self.predictions = predictions;
            self.selected_predition = None;
        }
        res
    }

    fn display_height(&self, win: &Window) -> usize {
        let win_height = win.get_max_y() as usize;

        // If there are predictions, show some and a separator
        if self.predictions.is_empty() {
            // Leave one line for the error message
            win_height - 1
        } else {
            // Leave one line for the error message, one for the separator and some for the predictions
            win_height - 2 - MAX_PREDICTIONS_SHOWN
        }
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

        // Show predictions
        let mut error_line = display_height;
        if !self.predictions.is_empty() {
            // Draw a separator with instructions
            win.mv(display_height as i32, 0);
            win.attron(pancurses::A_REVERSE);
            win.addstr( "Suggested input: (Press Page Up / Page Down to select. Press Shift-Tab to insert.)");
            win.hline(' ', win.get_max_x());
            win.attroff(pancurses::A_REVERSE);
            error_line += MAX_PREDICTIONS_SHOWN;

            // If no prediction is selected, draw the first few.
            let (start, end, highlight) = if let Some(selected) = self.selected_predition {
                let start = if selected > PREDICTION_SHOW_RAD {
                    selected - PREDICTION_SHOW_RAD
                } else {
                    0
                };
                let end = std::cmp::min(self.predictions.len(), start + MAX_PREDICTIONS_SHOWN);
                let highlight = selected - start;
                (start, end, highlight)
            } else {
                (
                    0,
                    std::cmp::min(self.predictions.len(), MAX_PREDICTIONS_SHOWN),
                    MAX_PREDICTIONS_SHOWN,
                )
            };

            for i in start..end {
                let offs = i - start;
                let is_selection = offs == highlight;
                win.mv((display_height + 1 + offs) as i32, 0);
                if is_selection {
                    win.attron(pancurses::A_UNDERLINE);
                }
                win.addstr(&self.predictions[i]);
                if is_selection {
                    win.attroff(pancurses::A_UNDERLINE);
                }
            }
        }

        win.attron(pancurses::A_REVERSE);
        win.mvaddnstr(error_line as i32, 0, &self.error, win.get_max_x());
        win.attroff(pancurses::A_REVERSE);
    }

    fn move_cursor(&self, win: &Window) {
        trace!("Cursor to ({},{})", self.cursor_win_line, self.cursor_col);
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
    debug!("{:?}", cmd_line);
    let grammar = cargo_toml::cargo_toml::grammar();
    let look_and_feel = cargo_toml::look_and_feel();

    // Set the locale so that UTF-8 codepoints appear correctly
    unsafe { libc::setlocale(libc::LC_ALL, NUL_BYTE_ARRAY[..].as_ptr()) };

    // Deactivate pressing Ctrl-C
    #[cfg(target_family = "unix")]
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN)
    };

    let mut app = App {
        editor: Editor::new(grammar),
        error: String::new(),
        document: Vec::new(),
        look_and_feel,
        cursor_doc_line: 0,
        cursor_win_line: 0,
        cursor_col: 0,
        predictions: Vec::new(),
        selected_predition: None,
        filename: cmd_line.input.clone(),
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
    let _ = app.update_prediction();
    app.display(&win);
    app.move_cursor(&win);
    win.refresh();

    loop {
        if let Some(input) = win.getch() {
            app.error = String::new();
            let app_cmd = app.handle_input(input);
            trace!("{:?}", app_cmd);
            match app_cmd {
                AppCmd::Nothing => {
                    // Don't do anything
                }
                AppCmd::Quit => break,
                AppCmd::Display => {
                    app.display(&win);
                    app.move_cursor(&win);
                    win.refresh();
                }
                AppCmd::Cursor => {
                    let pred_redisplay = app.update_prediction();
                    let scroll_redisplay = app.update_cursor(&win);
                    if pred_redisplay || scroll_redisplay {
                        app.display(&win);
                    }
                    app.move_cursor(&win);
                    win.refresh();
                }
                AppCmd::Document => {
                    app.update_document(win.get_max_x() as usize);
                    let _ = app.update_prediction();
                    let _ = app.update_cursor(&win);
                    app.display(&win);
                    app.move_cursor(&win);
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
