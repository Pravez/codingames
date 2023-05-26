use std::collections::HashMap;
use crate::game::input::{parse_tiles, update_tiles};

pub struct Game {
    pub bases: Vec<i32>,
    pub opponent_bases: Vec<i32>,
    pub tiles: HashMap<i32, HexTile>
}

#[derive(Clone, Debug)]
pub struct HexTile {
    pub id: i32,
    pub tile_type: i32,
    pub resources: i32,
    pub ants: i32,
    pub opponent_ants: i32,
    pub neighbours: Vec<i32>,
}

pub enum Action {
    WAIT,
    LINE(i32, i32, i32),
    BEACON(i32, i32),
    MESSAGE(String)
}

impl Game {
    pub fn update(&mut self) {
        update_tiles(&mut self.tiles);
    }

    pub fn play(&self) -> Vec<Action> {
        vec![Action::WAIT]
    }
}

