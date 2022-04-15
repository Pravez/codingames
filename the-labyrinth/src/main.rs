use crate::game::Game;
use crate::input::{read_config, read_turn};
use crate::structures::{Config};

mod structures;
mod input;
mod game;
mod path;


fn main() {
    let mut game = Game::new(read_config());

    loop {
        game.update(read_turn(game.config.rows));
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", game.play()); // Rick's next move (UP DOWN LEFT or RIGHT).
    }
}
