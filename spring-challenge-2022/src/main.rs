use crate::base::vec::Vec2;
use crate::game::Game;
use crate::input::{init, parse_turn};

mod base;
mod input;
mod structures;
mod game;
mod constants;

fn main() {
    let config = init();
    let game: Game = Game { base: config.base };
    loop {
        let turn = parse_turn();
        for hero in 0..config.heroes as usize {
            println!("{}", game.play(hero, &turn));
        }
    }
}