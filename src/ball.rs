use std::io;
use crossterm::QueueableCommand;
use crossterm::style::Color::{Red, White};
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use crate::frame::{Drawable, Frame};
use crate::{NUM_COLUMNS, NUM_ROWS};

pub struct Ball {
    pub x_pos: i32,
    pub y_pos: i32,
}

impl Ball {
    pub fn new(x_pos: i32, y_pos: i32) -> Self {
        Self {
            x_pos,
            y_pos,
        }
    }
    pub fn move_ball(&mut self, mut x_dir: i32, mut y_dir: i32, frame: &Frame) -> (i32, i32) {
        if (self.x_pos + x_dir > NUM_COLUMNS as i32 -1|| self.x_pos + x_dir < 0)
            && (self.x_pos != 0 && self.x_pos != NUM_COLUMNS as i32 -1) {

            self.x_pos += NUM_COLUMNS as i32-1 - self.x_pos;

        } else if (self.x_pos + x_dir < NUM_COLUMNS as i32 - 1 || self.x_pos + x_dir > 0)
            && (self.x_pos != 0 && self.x_pos != NUM_COLUMNS as i32 - 1){

            if frame[(self.x_pos + x_dir) as usize][self.y_pos as usize] != "\x1B[46m\x1B[37m█" {
                self.x_pos += x_dir;
            } else {
                x_dir = self.collision_calc(x_dir);
                self.x_pos += x_dir;
            }

        } else {
            x_dir = 0;
            y_dir = 0;

            let mut stdout = io::stdout();

            stdout.queue(SetBackgroundColor(Red)).unwrap();
            stdout.queue(SetForegroundColor(White)).unwrap();
            
            print!("\x1B[2J\x1B[1;{}H", NUM_COLUMNS/2);
            println!("You Lose !");
        }

        if (self.y_pos + y_dir > NUM_ROWS as i32 - 1 || self.y_pos + y_dir < 0)
            && (self.y_pos != 0 && self.y_pos != NUM_ROWS as i32 - 1) {

            self.y_pos += NUM_ROWS as i32 - 1 - self.y_pos;
            //println!("{:?}", self.y_pos);

        } else if (self.y_pos + y_dir < NUM_ROWS as i32 - 1 || self.y_pos + y_dir > 0)
            && (self.y_pos != 0 && self.y_pos != NUM_ROWS as i32 - 1){
            
            if frame[self.x_pos as usize][(self.y_pos + y_dir) as usize] != "\x1B[46m\x1B[37m█" {
                self.y_pos += y_dir;
            } else {
                y_dir = self.collision_calc(y_dir);
                self.y_pos += y_dir;
            }

        } else {
            y_dir = self.collision_calc(y_dir);
            self.y_pos += y_dir;
            //println!("{:?}", self.y_pos);
        }

        (x_dir, y_dir)
    }

    fn collision_calc(&mut self, direction: i32) -> i32 {
        -direction
    }
}

impl Drawable for Ball {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x_pos as usize][self.y_pos as usize] = "⬤";
    }
}