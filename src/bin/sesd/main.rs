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

use pancurses::{endwin, initscr, noecho, Input};
use structopt::StructOpt;

use sesd::SyncBlock;

mod bash;

#[derive(Debug, StructOpt)]
#[structopt(name = "sesd", about = "Syntax directed text editor")]
struct CommandLine {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

type Block = SyncBlock<char>;

/// All state of the edit app
struct App {
    /// Editable block of text in memory
    block: Block,

    /// Last error message
    error: String,
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
}

fn main() {
    let cmd_line = CommandLine::from_args();
    eprintln!("{:?}", cmd_line);

    let mut app = App {
        block: Block::new(bash::grammar()),
        error: String::new(),
    };

    // Load the file in the buffer if it exists
    app.load_input(&cmd_line);

    let win = initscr();

    noecho();

    loop {
        win.refresh();
        match win.getch() {
            Some(Input::Character(c)) => {
                win.addch(c);
            }
            Some(Input::KeyDC) => break,
            Some(input) => {
                win.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }

    endwin();
}
