extern crate ncurses;

use ncurses::*;
use std::thread;
use std::time::Duration;
use std::cmp::Ordering;

const POPULATION_SIZE: usize = 100;

const LEVEL_HEIGHT: u32 = 10;
const UP_KEY_VELOCITY: f32 = -0.5;
const GRAVITY: f32 = 0.05;

struct Population {
    birds: Vec<Bird>
}

impl Population {
    pub fn new(bird_amount: usize) -> Population {
        let mut birds = Vec::new();

        for i in 0 .. bird_amount {
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

    pub fn get_farthest_bird(&self) -> Option<&Bird> {
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

    pub fn draw(&self) {
        let height = self.height();

        // Draw a bird depending on if it just flapped
        if self.last_time < 10 && (self.last_time / 4) % 2 == 0 {
            mvprintw(self.x, height, "-o-");
        } else {
            mvprintw(self.x, height, "\\o/");
        }
    }
}

// Order the birds by x distance
impl Ord for Bird {
    fn cmp(&self, other: &Bird) -> Ordering {
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

    let mut pop = Population::new(POPULATION_SIZE);

    let mut ch = getch();
    while ch != 'q' as i32 {
        pop.tick();

        refresh();
    
        thread::sleep(Duration::from_millis(1000 / 30));
        ch = getch();
    }

    close_ncurses();
}
