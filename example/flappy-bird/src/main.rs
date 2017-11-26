extern crate ncurses;

use ncurses::*;
use std::thread;
use std::time::Duration;

fn init_ncurses() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);
}

fn close_ncurses() {
    endwin();
}

fn main() {
    init_ncurses();

    let mut ch = getch();
    while ch != 'q' as i32 {
        ch = getch();

        refresh();
    
        thread::sleep(Duration::from_millis(1000 / 30));
    }

    close_ncurses();
}
