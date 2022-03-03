use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::shot1::Shot1;
use std::time::Duration;
use crate::second_player::SecondPlayer;
use crate::shot2::Shot2;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot1>,
    pub isDead: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
            isDead: false
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_left2(&mut self) {
        if self.x > 2 {
            self.x -= 3;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn move_right2(&mut self) {
        if self.x < NUM_COLS - 3 {
            self.x += 3;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 && !self.isDead {
            self.shots.push(Shot1::new(self.x, self.y -1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta:Duration, enemy: &mut SecondPlayer) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
            shot.hit_player(enemy);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn killed(&mut self, shot: &Shot2) -> bool {
        if !self.isDead {
            if self.x == shot.x && self.y == shot.y {
                self.isDead = true;
                true
            }
            else{
                false
            }
        } else {
            false
        }
    }
}

impl Drawable for Player{
    fn draw(&self, frame: &mut Frame) {
        if self.isDead{
            frame[self.x][self.y] = "*";
        } else{
            frame[self.x][self.y] = "A";
        }
        for shot in self.shots.iter(){
            shot.draw(frame);
        }
    }
}