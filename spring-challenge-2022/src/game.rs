use crate::input::{Config, Turn};
use crate::Vec2;

pub struct Game {
    pub base: Vec2<usize>,
}

impl Game {
    // In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
    pub fn play(&self, hero: usize, turn: &Turn) -> String {
        String::from("WAIT")
    }
}