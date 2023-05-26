use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::time::{Duration, Instant};
use crate::game::game::{Direction, Game, HexTile, TileType};

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
            tile_type: TileType::from(parse_input!(inputs[0], i32)),
            resources: parse_input!(inputs[1], i32),
            ants: 0,
            opponent_ants: 0,
            neighbours: (2..7).into_iter().map(|i| (Direction::from(i), parse_input!(inputs[i as usize], i32))).collect(),
            coordinates: (0, 0, 0),
        })
    }).collect()
}

pub fn update_tiles(tiles: &mut HashMap<i32, HexTile>) {
    (0..tiles.len()).into_iter().for_each(|index| {
        let data = read_line();
        let values = data.split(" ").collect::<Vec<_>>();
        tiles.get_mut(&(index as i32)).map(|t| t.update_data(parse_input!(values[0], i32), parse_input!(values[1], i32), parse_input!(values[2], i32)));
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
        crystals_indexes: vec![],
    }
}

pub fn add_coordinates(left: (i32, i32, i32), right: (i32, i32, i32)) -> (i32, i32, i32) {
    (left.0 + right.0, left.1 + right.1, left.2 + right.2)
}

pub fn build_map(tiles: &mut HashMap<i32, HexTile>, start: &mut HexTile) {
    start.coordinates = (0, 0, 0);

    let mut visited = HashMap::<i32, &str>::new();
    let mut to_visit = vec![start.id];

    let start = Instant::now();
    let mut index = 0;
    while visited.len() != tiles.len() {
        let next_visit = to_visit.iter().flat_map(|id| {
            let tile = tiles.get(&id).unwrap().to_owned();
            visited.insert(*id, "");

            let neighbours = tile.neighbours.iter()
                .filter(|(_, id)| !visited.contains_key(id) && *id != -1)
                .collect::<Vec<_>>();
            neighbours.iter().for_each(|(dir, id)| {
                tiles.get_mut(id).map(|m| m.update_coordinates(add_coordinates(tile.coordinates, dir.to_coordinates())));
            });
            neighbours.iter().map(|(_, id)| *id).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        eprintln!("{}/{}", visited.len(), tiles.len());
        to_visit.clear();
        to_visit.extend(next_visit.iter());
        index += 1;
    }
    eprintln!("Built map in {} steps ({:?})", index, start.elapsed());
}
