use crate::game::Game;
use crate::input::{read_config, read_turn};
use crate::structures::Config;

mod structures;
mod input;
mod game;
mod board;
mod pathfinding;


fn main() {
    let mut game = Game::new(read_config());

    loop {
        game.update(read_turn(game.config.rows));
        println!("{}", game.play());
    }
}
