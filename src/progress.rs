use std::io::{stdout, Write};

const OTHER_CHARS_COUNT: usize = 9;

pub fn bar(current: usize, total: usize, width: usize) {
    let percent = (current as f64) / (total as f64);
    let bar_length = width - OTHER_CHARS_COUNT;
    let fill_count = (percent * (bar_length as f64)) as usize;
    let space_count = bar_length - fill_count;
    let progress_bar = "#".repeat(fill_count) + &" ".repeat(space_count);
    print!("\r[{:3.0}%] [{:}]", percent * 100., progress_bar);
    stdout().flush().unwrap();
}
