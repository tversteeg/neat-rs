extern crate ncurses;

use ncurses::*;
use std::thread;
use std::time::Duration;
use std::cmp::Ordering;

const POPULATION_SIZE: usize = 100;
const PIPE_AMOUNT: usize = 100;

const PIPE_OFFSET: i32 = 5;

const LEVEL_HEIGHT: i32 = 24;
const UP_KEY_VELOCITY: f32 = -0.5;
const GRAVITY: f32 = 0.05;

const WINDOW_HEIGHT: i32 = 22;
const WINDOW_WIDTH: i32 = 78;

struct Level {
    population: Population,
    pipes: Vec<Pipe>
}

impl Level {
    pub fn new() -> Level {
        let mut pipes = Vec::new();

        for i in 1 .. PIPE_AMOUNT + 1 {
            pipes.push(Pipe::new(i as i32 * PIPE_OFFSET));
        }

        Level {
            population: Population::new(),
            pipes: pipes
        }
    }

    pub fn tick(&mut self) {
        self.population.tick();
    }

    pub fn draw(&self, win: WINDOW) {
        let farthest_x = self.population.farthest_bird().unwrap().x;
        let cam_x = farthest_x - WINDOW_WIDTH / 2;

        self.population.draw(win, cam_x);

        for pipe in self.pipes.iter() {
            pipe.draw(win, cam_x);
        }
    }
}

struct Pipe {
    x: i32,
    y_center: i32,
    opening_height: f32
}

impl Pipe {
    pub fn new(x: i32) -> Pipe {
        Pipe {
            x: x,
            y_center: LEVEL_HEIGHT / 2,
            opening_height: 0.2
        }
    }

    pub fn draw(&self, win: WINDOW, x_offset: i32) {
        let x = self.x - x_offset;

        let opening_height = (LEVEL_HEIGHT as f32 * self.opening_height) as i32;

        // Draw top part of the pipe
        let top_y = LEVEL_HEIGHT - self.y_center - opening_height;
        mvwvline(win, 1, x, 0, top_y - 1);

        // Draw bottom part of the pipe
        let bottom_y = LEVEL_HEIGHT - self.y_center + opening_height;
        mvwvline(win, bottom_y, x, 0, LEVEL_HEIGHT - bottom_y);
    }
}

struct Population {
    birds: Vec<Bird>
}

impl Population {
    pub fn new() -> Population {
        let mut birds = Vec::new();

        for i in 0 .. POPULATION_SIZE {
            birds.push(Bird::new(i as i32));
        }

        Population {
            birds: birds
        }
    }

    pub fn tick(&mut self) {
        for bird in self.birds.iter_mut() {
            bird.x += 1;
            bird.last_time += 1;

            let height = bird.height();
            if height < 0 || height > LEVEL_HEIGHT as i32 {
                bird.reset();
            }
        }
    }

    pub fn draw(&self, win: WINDOW, cam_x: i32) {
        for bird in self.birds.iter() {
            bird.draw(win, cam_x);
        }
    }

    pub fn farthest_bird(&self) -> Option<&Bird> {
        self.birds.iter().max()
    }
}

#[derive(Eq)]
struct Bird {
    last_height: i32,
    last_time: i32,
    x: i32
}

impl Bird {
    pub fn new(height: i32) -> Bird {
        Bird {
            last_height: height,
            last_time: 0,
            x: 0
        }
    }

    pub fn height(&self) -> i32 {
        let time_squared = (self.last_time * self.last_time) as f32;
        let gravity_since_last = GRAVITY * 0.5 * time_squared;
        let velocity_since_last = UP_KEY_VELOCITY * self.last_time as f32;

        self.last_height + (velocity_since_last + gravity_since_last) as i32
    }

    pub fn reset(&mut self) {
        self.last_height = 0;
        self.last_time = 0;
        self.x = 0;
    }

    pub fn draw(&self, win: WINDOW, x_offset: i32) {
        let x = self.x - x_offset;
        let y = LEVEL_HEIGHT - self.height() - 1;

        // Draw a bird depending on if it just flapped
        if self.last_time < 10 && (self.last_time / 4) % 2 == 0 {
            mvwprintw(win, y, x, "-o-");
        } else {
            mvwprintw(win, y, x, "\\o/");
        }
    }
}

impl Ord for Bird {
    fn cmp(&self, other: &Bird) -> Ordering {
        // Order the birds by x distance
        self.x.cmp(&other.x)
    }
}

impl PartialOrd for Bird {
    fn partial_cmp(&self, other: &Bird) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {
        self.x == other.x
    }
}

fn init_ncurses() -> WINDOW {
    initscr();
    raw();
    // Allow the extended keyboard
    keypad(stdscr(), true);
    noecho();

    // Hide the cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);

    // Get the screen bounds
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // Start the window in the center
    let start_y = (max_y - WINDOW_HEIGHT) / 2;
    let start_x = (max_x - WINDOW_WIDTH) / 2;
    let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn draw_ncurses(win: WINDOW) {
    // Draw the window edges
    box_(win, 0, 0);

    wrefresh(win);
    refresh();
}

fn close_ncurses(win: WINDOW) {
    delwin(win);
    endwin();
}

fn main() {
    let win = init_ncurses();

    let mut level = Level::new();

    let mut ch = getch();
    while ch != 'q' as i32 {
        level.tick();

        // Clear the window
        werase(win);
        level.draw(win);

        draw_ncurses(win);
    
        thread::sleep(Duration::from_millis(1000 / 30));
        ch = getch();
    }

    close_ncurses(win);
}
