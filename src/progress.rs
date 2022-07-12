use std::io::{stdout, Write};

const OTHER_CHARS_COUNT: usize = 9;

pub struct Bar {
    current: usize,
    total: usize,
    width: usize,
}

impl Bar {
    pub fn new(total: usize, width: usize) -> Self {
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
