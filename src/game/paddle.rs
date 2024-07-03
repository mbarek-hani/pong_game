use std::marker::PhantomData;

use raylib::consts::KeyboardKey::{KEY_DOWN, KEY_UP};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Rectangle,
    RaylibHandle,
};

pub struct Human;
pub struct Cpu;

pub struct Paddle<State> {
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: i32,
    pub height: i32,
    pub speed: i32,
    state: PhantomData<State>,
}

impl Paddle<Human> {
    pub fn new(screen_h: i32, width: i32, height: i32) -> Self {
        Self {
            pos_x: 10,
            pos_y: screen_h / 2 - 2 * width,
            width,
            height,
            speed: 4,
            state: PhantomData::<Human>,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, screen_h: i32) {
        if rl.is_key_down(KEY_UP) {
            self.pos_y -= self.speed;
        }
        if rl.is_key_down(KEY_DOWN) {
            self.pos_y += self.speed;
        }

        self.restrict_movement(screen_h);
    }
}

impl Paddle<Cpu> {
    pub fn new(screen_h: i32, screen_w: i32, width: i32, height: i32) -> Self {
        Self {
            pos_x: screen_w - 30,
            pos_y: screen_h / 2 - 2 * width,
            width,
            height,
            speed: 4,
            state: PhantomData::<Cpu>,
        }
    }

    pub fn update(&mut self, ball_center_y: i32, screen_h: i32) {
        if self.pos_y + self.height / 2 > ball_center_y {
            self.pos_y -= self.speed - 1;
        }
        if self.pos_y + self.height / 2 < ball_center_y {
            self.pos_y += self.speed - 1;
        }
        self.restrict_movement(screen_h);
    }
}

impl<State> Paddle<State> {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let rec = Rectangle {
            x: self.pos_x as f32,
            y: self.pos_y as f32,
            width: self.width as f32,
            height: self.height as f32,
        };
        d.draw_rectangle_rounded(rec, 0.8, 0, Color::WHITE);
    }

    pub fn restrict_movement(&mut self, screen_h: i32) {
        if self.pos_y <= 0 {
            self.pos_y = 0;
        }
        if self.pos_y >= screen_h - self.height {
            self.pos_y = screen_h - self.height;
        }
    }
}
