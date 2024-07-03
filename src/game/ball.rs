use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

pub struct Ball {
    pub radius: f32,
    pub center_x: i32,
    pub center_y: i32,
    pub speed_x: i32,
    pub speed_y: i32,
}

impl Ball {
    pub fn new(screen_w: i32, screen_h: i32) -> Self {
        let x_direction = {
            if rand::random::<bool>() {
                -1
            } else {
                1
            }
        };
        let y_direction = {
            if rand::random::<bool>() {
                1
            } else {
                -1
            }
        };

        Self {
            radius: 15.0,
            center_x: screen_w / 2,
            center_y: screen_h / 2,
            speed_x: x_direction * 6,
            speed_y: y_direction * 6,
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_circle(self.center_x, self.center_y, self.radius, color);
    }

    pub fn update(&mut self, screen_h: i32) {
        self.center_x += self.speed_x;
        self.center_y += self.speed_y;

        if self.center_y + self.radius as i32 >= screen_h || self.radius >= self.center_y as f32 {
            self.speed_y *= -1;
        }
    }
}
