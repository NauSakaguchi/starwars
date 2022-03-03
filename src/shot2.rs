use rusty_time::timer::Timer;
use std::time::Duration;
use crate::frame::{Drawable, Frame};
use crossterm::terminal;
use crate::NUM_ROWS;
use crate::player::Player;

pub struct Shot2 {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub is_end: bool,
    timer: Timer,
}

impl Shot2{
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            is_end: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y < NUM_ROWS - 1 {
                self.y += 1;
            } else if self.y == NUM_ROWS - 1 {
                self.is_end = true;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.is_end)
    }

    pub fn hit_player(&mut self, player: &mut Player) -> bool {
        if player.killed(&self) {
            self.exploding = true;
            true
        } else {
            false
        }
    }
}


impl Drawable for Shot2 {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding {"*"} else {"|"};
    }
}