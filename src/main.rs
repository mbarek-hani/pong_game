mod game;

use game::Game;

fn main() {
    let mut game = Game::new();
    let (mut rl, thread) = raylib::init()
        .size(Game::WIDTH, Game::HEIGHT)
        .title("Pong game")
        .vsync()
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        //updating

        game.update(&mut rl);

        //Drawing
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
}
