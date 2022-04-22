use lib::input::{init, parse_turn};

use crate::game::Game;
use crate::lib::base::vec::Vec2;

mod game;
mod lib;
mod hero;
mod monster;

fn main() {
    let config = init();
    let mut game = Game::new(&config);
    loop {
        game.update(&parse_turn());
        for hero in 0..config.heroes {
            println!("{}", game.play(&hero));
        }
    }
}