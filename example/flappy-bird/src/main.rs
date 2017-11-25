extern crate ncurses;

use ncurses::*;

fn init_ncurses() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);
}

fn main() {
    init_ncurses();

    println!("Hello, world!");
}
