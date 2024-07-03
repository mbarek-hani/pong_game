mod ball;
mod paddle;

use ball::Ball;
use paddle::{Cpu, Human, Paddle};
use raylib::prelude::*;

pub struct Game {
    ball: Ball,
    paddle_player: Paddle<Human>,
    paddle_cpu: Paddle<Cpu>,
    player_score: u32,
    cpu_score: u32,
}

impl Game {
    pub const WIDTH: i32 = 800;
    pub const HEIGHT: i32 = 500;
    const GREEN: Color = Color::new(83, 185, 154, 255);
    const DARK_GREEN: Color = Color::new(20, 160, 133, 255);
    const LIGHT_GREEN: Color = Color::new(129, 204, 184, 255);
    const YELLOW: Color = Color::new(243, 213, 91, 255);

    pub fn new() -> Self {
        Self {
            ball: Ball::new(Self::WIDTH, Self::HEIGHT),
            paddle_player: Paddle::<Human>::new(Self::HEIGHT, 20, 80),
            paddle_cpu: Paddle::<Cpu>::new(Self::HEIGHT, Self::WIDTH, 20, 80),
            player_score: 0,
            cpu_score: 0,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Self::DARK_GREEN);
        d.draw_rectangle(
            Self::WIDTH / 2,
            0,
            Self::WIDTH / 2,
            Self::HEIGHT,
            Self::GREEN,
        );
        d.draw_circle(Self::WIDTH / 2, Self::HEIGHT / 2, 100.0, Self::LIGHT_GREEN);
        self.ball.draw(d, Self::YELLOW);
        self.paddle_player.draw(d);
        self.paddle_cpu.draw(d);
        d.draw_line(
            Self::WIDTH / 2,
            0,
            Self::WIDTH / 2,
            Self::HEIGHT,
            Color::WHITE,
        );
        d.draw_text(
            &self.player_score.to_string(),
            Self::WIDTH / 4,
            10,
            30,
            Color::WHITE,
        );
        d.draw_text(
            &self.cpu_score.to_string(),
            3 * Self::WIDTH / 4,
            10,
            30,
            Color::WHITE,
        );
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let player_rec = Rectangle {
            x: self.paddle_player.pos_x as f32,
            y: self.paddle_player.pos_y as f32,
            width: self.paddle_player.width as f32,
            height: self.paddle_player.height as f32,
        };
        let cpu_rec = Rectangle {
            x: self.paddle_cpu.pos_x as f32,
            y: self.paddle_cpu.pos_y as f32,
            width: self.paddle_cpu.width as f32,
            height: self.paddle_cpu.height as f32,
        };
        if player_rec.check_collision_circle_rec(
            Vector2 {
                x: self.ball.center_x as f32,
                y: self.ball.center_y as f32,
            },
            15.0,
        ) {
            self.ball.speed_x *= -1;
        }
        if cpu_rec.check_collision_circle_rec(
            Vector2 {
                x: self.ball.center_x as f32,
                y: self.ball.center_y as f32,
            },
            15.0,
        ) {
            self.ball.speed_x *= -1;
        }
        if self.ball.center_x + self.ball.radius as i32 >= Self::WIDTH {
            self.player_score += 1;
            self.game_over();
        }
        if self.ball.radius >= self.ball.center_x as f32 {
            self.cpu_score += 1;
            self.game_over();
        }
        self.ball.update(Self::HEIGHT);
        self.paddle_player.update(rl, Self::HEIGHT);
        self.paddle_cpu.update(self.ball.center_y, Self::HEIGHT);
    }

    fn game_over(&mut self) {
        self.ball = Ball::new(Self::WIDTH, Self::HEIGHT);
        self.paddle_cpu = Paddle::<Cpu>::new(Self::HEIGHT, Self::WIDTH, 20, 80);
        self.paddle_player = Paddle::<Human>::new(Self::HEIGHT, 20, 80);
    }
}
