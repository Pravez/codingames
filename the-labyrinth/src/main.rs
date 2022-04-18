use crate::game::Game;
use crate::input::{read_config, read_turn};

mod directions;
mod input;
mod game;
mod base;
mod pathfinding;


fn main() {
    let mut game = Game::new(read_config());

    loop {
        game.update(read_turn(game.config.rows));
        println!("{}", game.play());
    }
}
