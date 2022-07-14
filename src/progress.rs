use std::io::{stdout, Write};

use crate::constants::{OTHER_CHARS_COUNT, MOUSE_TICK_DELTA};

pub struct Bar {
    pub current: usize,
    total: usize,
    width: usize,
}

impl Bar {
    pub const fn default() -> Self {
        Bar {
            current: 0,
            total: 0,
            width: 0,
        }
    }
    pub fn new(total: usize) -> Self {
        let (width, _) = term_size::dimensions().unwrap();
        let bar = Bar {
            current: 0,
            total,
            width,
        };
        bar.draw();
        bar
    }

    pub fn inc(&mut self) {
        self.current += 1;
        self.draw();
    }

    pub fn step_complete(&self) -> bool {
        self.current % MOUSE_TICK_DELTA == 0
    }

    pub fn complete(&self) -> bool {
        self.current == self.total
    }

    fn draw(&self) {
        let percent = (self.current as f64) / (self.total as f64);
        let bar_length = self.width - OTHER_CHARS_COUNT;
        let fill_count = (percent * (bar_length as f64)) as usize;
        let space_count = bar_length - fill_count;
        let progress_bar = "#".repeat(fill_count) + &" ".repeat(space_count);
        print!("\r[{:3.0}%] [{:}]", percent * 100., progress_bar);
        stdout().flush().unwrap();
    }
}
