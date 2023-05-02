use macroquad::{prelude::{Color, PINK, Circle, Vec2, vec2}, shapes::draw_circle};

pub const SPEED:f32 = 8f32;
pub const RADIUS:f32 = 15f32;
pub const COLOR:Color = PINK;

#[derive(Copy, Clone)]
pub struct Ball {
    circle: Circle,
    dir: Vec2
}

impl Ball {
    pub fn new(circle:Circle) -> Self {
        Self {
            circle,
            dir:vec2(0.,1.)
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, COLOR);
    }
}