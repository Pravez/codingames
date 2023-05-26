use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use crate::game::game::{Game, HexTile};

#[warn(dead_code)]
pub fn extract_inputs() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        eprintln!("{}", line.unwrap());
    }

    eprintln!("---------------------------");
    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            eprintln!("{}", line.unwrap());
        }
    }
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn parse_tiles(number_of_cells: i32) -> HashMap<i32, HexTile> {
    (0..number_of_cells).into_iter().map(|index| {
        let input_line = read_line();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        (index, HexTile {
            id: index as i32,
            tile_type: parse_input!(inputs[0], i32),
            resources: parse_input!(inputs[1], i32),
            ants: 0,
            opponent_ants: 0,
            neighbours: (2..7).into_iter().map(|i| parse_input!(inputs[i], i32)).collect(),
        })
    }).collect()
}

pub fn update_tiles(tiles: &mut HashMap<i32, HexTile>) {
    (0..tiles.len()).into_iter().for_each(|index| {
        let data = read_line();
        eprintln!("{}", data);
        let mut tile = tiles.get(&(index as i32)).unwrap().to_owned();
        let values = data.split(" ").collect::<Vec<_>>();
        tile.resources = parse_input!(values[0], i32);
        tile.ants = parse_input!(values[1], i32);
        tile.opponent_ants = parse_input!(values[2], i32);
    })
}

pub fn parse_initial_inputs() -> Game {
    let number_of_cells = parse_input!(read_line(), i32); // amount of hexagonal cells in this map
    let tiles = parse_tiles(number_of_cells);
    let _ = parse_input!(read_line(), i32);
    let bases: Vec<_> = read_line().split_whitespace().map(|i| parse_input!(i, i32)).collect();
    let opponent_bases: Vec<_> = read_line().split_whitespace().map(|i| parse_input!(i, i32)).collect();
    Game {
        bases,
        opponent_bases,
        tiles,
    }
}

