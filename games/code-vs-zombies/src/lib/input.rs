use std::fmt::{Display, Formatter};
use std::io;
use crate::lib::base::vec::Vec2;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub struct Setup {
    pub player: Vec2<i32>,
    pub humans: Vec<Human>,
    pub zombies: Vec<Zombie>,
}

#[derive(Clone, Copy, Debug)]
pub struct Human {
    pub id: i32,
    pub position: Vec2<i32>,
}

#[derive(Clone, Copy, Debug)]
pub struct Zombie {
    pub id: i32,
    pub position: Vec2<i32>,
    pub next_position: Vec2<i32>,
}

impl Display for Zombie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} {}", self.id, self.position.x, self.position.y)
    }
}

pub fn parse_inputs() -> Setup {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x = parse_input!(inputs[0], i32);
    let y = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let human_count = parse_input!(input_line, i32);
    let humans = (0..human_count).map(|h| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let human_id = parse_input!(inputs[0], i32);
        let human_x = parse_input!(inputs[1], i32);
        let human_y = parse_input!(inputs[2], i32);
        Human {
            id: human_id,
            position: Vec2::new(human_x, human_y),
        }
    }).collect::<Vec<_>>();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let zombie_count = parse_input!(input_line, i32);
    let zombies = (0..zombie_count).map(|z| {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let zombie_id = parse_input!(inputs[0], i32);
        let zombie_x = parse_input!(inputs[1], i32);
        let zombie_y = parse_input!(inputs[2], i32);
        let zombie_xnext = parse_input!(inputs[3], i32);
        let zombie_ynext = parse_input!(inputs[4], i32);
        Zombie {
            id: zombie_id,
            position: Vec2::new(zombie_x, zombie_y),
            next_position: Vec2::new(zombie_xnext, zombie_ynext),
        }
    }).collect::<Vec<_>>();
    Setup {
        player: Vec2::new(x, y),
        humans,
        zombies,
    }
}