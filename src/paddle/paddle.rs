use macroquad::{window::{screen_height}, prelude::{Color, DARKBLUE, Rect, KeyCode, is_key_down}, shapes::draw_rectangle};

use crate::ball::RADIUS;

pub const WIDTH:f32 = 20f32;
pub const HEIGHT:f32 = 80f32;
pub const COLOR:Color = DARKBLUE;
pub const SPEED:f32 = 10f32;

#[derive(Copy, Clone)]
pub struct Paddle {
    pub rect: Rect
}

impl Paddle {
   pub fn new(rect:Rect) -> Self {
    Self {
        rect
    }
   }

   pub fn draw(&self) {
    let r = self.rect;
    draw_rectangle(r.x, r.y, r.w, r.h, COLOR);
   }

   pub fn movement(&mut self, up:KeyCode, down:KeyCode) {
    if is_key_down(up) {
        self.rect.y -= 1.*SPEED;
    } else if is_key_down(down) {
        self.rect.y += 1.*SPEED;
    }
    if self.rect.y < 0. {
        self.rect.y = 0.;
    } else if self.rect.y > screen_height() - HEIGHT {
        self.rect.y = screen_height() - HEIGHT;
    }
   }
}