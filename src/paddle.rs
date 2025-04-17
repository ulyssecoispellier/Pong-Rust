use crate::{NUM_COLUMNS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Paddle {
    x_pos: usize,
    y_pos: usize,
}

impl Paddle {
    pub fn new(is_left_player: bool) -> Self {
        if is_left_player {
            Self { x_pos: 4, y_pos: (NUM_ROWS-1)/2, }
        } else {
            Self {x_pos: NUM_COLUMNS-5, y_pos: (NUM_ROWS-1)/2}
        }
    }

    pub fn move_up (&mut self) -> Self{
        if self.y_pos > 0 {
            self.y_pos -= 1;
        }
        
        Self {x_pos: self.x_pos, y_pos: self.y_pos}
    }
    pub fn move_down (&mut self) -> Self {
        if self.y_pos < NUM_ROWS - 1 {
            self.y_pos += 1;
        }
        
        Self {x_pos: self.x_pos, y_pos: self.y_pos}
    }
}

impl Drawable for Paddle {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x_pos][self.y_pos] = "|"
    }
}