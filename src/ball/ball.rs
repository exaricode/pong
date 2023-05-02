use macroquad::{prelude::{Color, PINK, Circle, Vec2, vec2, Rect}, shapes::draw_circle, window::screen_height};
use rand::{thread_rng, Rng};

pub const SPEED:f32 = 8f32;
pub const RADIUS:f32 = 15f32;
pub const COLOR:Color = PINK;

#[derive(Copy, Clone)]
pub struct Ball {
    pub circle: Circle,
    dir: Vec2
}

impl Ball {
    pub fn new(circle:Circle) -> Self {
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let dir_x: f32 = rng.gen_range(-1.0..1.);
        let dir_y: f32 = rng.gen_range(-1.0..1.);
        Self {
            circle,
            dir:vec2(dir_x,dir_y)
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, COLOR);
    }

    pub fn movement(&mut self) {
        self.circle.x += self.dir.x * SPEED;
        self.circle.y += self.dir.y * SPEED;

        if self.circle.y > screen_height() - RADIUS || self.circle.y <= 0. {
            self.dir.y = -self.dir.y;
        }
    }

    pub fn collision(&mut self, paddle_1: &Rect, paddle_2: &Rect) {
        let rect = Rect::new(self.circle.x, self.circle.y, RADIUS, RADIUS);
        if rect.intersect(*paddle_1).is_some() || rect.intersect(*paddle_2).is_some() {
            self.dir.x = -self.dir.x;
        }
   }
}