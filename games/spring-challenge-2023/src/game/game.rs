use std::collections::HashMap;
use crate::game::input::{parse_tiles, update_tiles};

pub struct Game {
    pub bases: Vec<i32>,
    pub opponent_bases: Vec<i32>,
    pub tiles: HashMap<i32, HexTile>,
    pub crystals_indexes: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Empty = 0,
    Eggs = 1,
    Crystals = 2,
}

impl From<i32> for TileType {
    fn from(value: i32) -> Self {
        match value {
            0 => TileType::Empty,
            1 => TileType::Eggs,
            2 => TileType::Crystals,
            _ => panic!("Invalid tile type")
        }
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    SMINUS = 2,
    SPLUS = 5,
    QMINUS = 6,
    QPLUS = 3,
    RMINUS = 4,
    RPLUS = 7,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        match value {
            2 => Direction::SMINUS,
            5 => Direction::SPLUS,
            6 => Direction::QMINUS,
            3 => Direction::QPLUS,
            4 => Direction::RMINUS,
            7 => Direction::RPLUS,
            _ => panic!("Invalid direction")
        }
    }
}

impl Direction {
    pub fn to_coordinates(&self) -> (i32, i32, i32) {
        match self {
            Direction::SMINUS => (0, 0, -1),
            Direction::SPLUS => (0, 0, 1),
            Direction::QMINUS => (-1, 0, 0),
            Direction::QPLUS => (1, 0, 0),
            Direction::RMINUS => (0, -1, 0),
            Direction::RPLUS => (0, 1, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HexTile {
    pub id: i32,
    pub tile_type: TileType,
    pub resources: i32,
    pub ants: i32,
    pub opponent_ants: i32,
    pub neighbours: Vec<(Direction, i32)>,
    pub coordinates: (i32, i32, i32),
}

pub enum Action {
    WAIT,
    LINE(i32, i32, i32),
    BEACON(i32, i32),
    MESSAGE(String),
}

impl Game {
    pub fn update(&mut self) {
        self.crystals_indexes = self.tiles.values().filter(|t| t.tile_type == TileType::Crystals).map(|t| t.id).collect();
    }

    pub fn dist(&self, a: i32, b: i32) -> i32 {
        (a - b).abs()
    }

    pub fn play(&self) -> Vec<Action> {
        vec![Action::WAIT]
    }
}

