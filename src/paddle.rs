use crate::{NUM_COLUMNS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

pub struct Paddle {
    x_pos: usize,
    y_pos: usize,
}

impl Paddle {
    pub fn new(is_left_player: bool) -> Self {
        if is_left_player {
            Self { x_pos: 10, y_pos: (NUM_ROWS-1)/2, }
        } else {
            Self {x_pos: NUM_COLUMNS-11, y_pos: (NUM_ROWS-1)/2}
        }
    }

    pub fn move_up (&mut self){
        if self.y_pos > 8 {
            self.y_pos -= 1;
        }
    }
    pub fn move_down (&mut self) {
        if self.y_pos < NUM_ROWS - 9 {
            self.y_pos += 1;
        }
    }
}

impl Drawable for Paddle {
    fn draw(&self, frame: &mut Frame) {
        for i in self.y_pos-8..=self.y_pos+8{
            frame[self.x_pos][i] = "\x1B[46m\x1B[37m█";
        }
        // Separator (Not sure where I could put it)
        for i in 0..NUM_ROWS {
            frame[NUM_COLUMNS/2][i] = "|";
        }
    }
}