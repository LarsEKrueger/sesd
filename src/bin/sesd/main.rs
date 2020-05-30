use sesd::SyncBlock;

use pancurses::{endwin, initscr, noecho, Input};

fn main() {
    let win = initscr();

    noecho();

    let block = SyncBlock::<char>::new();

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
