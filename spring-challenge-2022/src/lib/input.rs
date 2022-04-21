use std::io;

use crate::{Vec2, vec2};
use crate::lib::structures::UnitType;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub struct Config {
    pub base: Vec2<u32>,
    pub heroes: i32,
}

pub struct Turn {
    pub me: Player,
    pub opponent: Player,
    pub units: Vec<InputUnit>,
}

#[derive(Copy, Clone)]
pub struct Player {
    pub health: i32,
    pub mana: i32,
}

pub struct InputUnit {
    pub id: i32,
    pub unit_type: UnitType,
    pub position: Vec2<u32>,
    pub health: i32,
    pub is_controlled: bool,
    pub shield_life: i32,
    pub trajectory: Vec2<i32>,
    pub near_base: bool,
    pub threat_for: i32,
}

pub fn init() -> Config {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let base_x = parse_input!(inputs[0], i32); // The corner of the map representing your base
    let base_y = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let heroes_per_player = parse_input!(input_line, i32); // Always 3
    Config {
        base: vec2!(base_x as u32, base_y as u32),
        heroes: heroes_per_player,
    }
}

pub fn parse_turn() -> Turn {
    let players: Vec<Player> = (0..2).map(|_| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let health = parse_input!(inputs[0], i32); // Your base health
        let mana = parse_input!(inputs[1], i32); // Ignore in the first league; Spend ten mana to cast a spell
        Player { health, mana }
    }).collect();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let entity_count = parse_input!(input_line, i32); // Amount of heros and monsters you can see
    let units = (0..entity_count).map(|_| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let id = parse_input!(inputs[0], i32); // Unique identifier
        let unit_type = parse_input!(inputs[1], i32); // 0=monster, 1=your hero, 2=opponent hero
        let x = parse_input!(inputs[2], i32); // Position of this entity
        let y = parse_input!(inputs[3], i32);
        let shield_life = parse_input!(inputs[4], i32); // Ignore for this league; Count down until shield spell fades
        let is_controlled = parse_input!(inputs[5], i32); // Ignore for this league; Equals 1 when this entity is under a control spell
        let health = parse_input!(inputs[6], i32); // Remaining health of this monster
        let vx = parse_input!(inputs[7], i32); // Trajectory of this monster
        let vy = parse_input!(inputs[8], i32);
        let near_base = parse_input!(inputs[9], i32); // 0=monster with no target yet, 1=monster targeting a base
        let threat_for = parse_input!(inputs[10], i32); // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither
        InputUnit {
            id,
            unit_type: UnitType::from_int(unit_type),
            position: vec2!(x as u32, y as u32),
            health,
            is_controlled: is_controlled == 1,
            shield_life,
            trajectory: vec2!(vx, vy),
            near_base: near_base == 1,
            threat_for
        }
    }).collect();

    Turn {
        me: players.first().unwrap().clone(),
        opponent: players.get(1).unwrap().clone(),
        units,
    }
}