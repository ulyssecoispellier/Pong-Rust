use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo as MoveCursorTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn renderer(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::White)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for (x, column) in current_frame.iter().enumerate() {
        for (y, s) in column.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveCursorTo(x as u16, y as u16)).unwrap();
                print!("\x1B[40m{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}