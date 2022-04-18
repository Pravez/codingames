use std::io;
use crate::{Config};
use crate::structures::Dimension;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn read_config() -> Config {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let rows = parse_input!(inputs[0], i32); // number of rows.
    let cols = parse_input!(inputs[1], i32); // number of columns.
    let alarm = parse_input!(inputs[2], i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.
    Config {
        rows,
        cols,
        alarm,
    }
}

pub fn read_turn(rows: i32) -> (Dimension, Vec<Vec<char>>, Dimension) {
    eprintln!("Parsing ...");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let k = Dimension { x: parse_input!(inputs[0], i32) as usize, y: parse_input!(inputs[1], i32) as usize };
    let mut board = Vec::new();
    let mut initial_position = Dimension::default();
    for x in 0..rows as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim().chars().collect::<Vec<_>>();
        if let Some(y) = row.iter().position(|&r| r == 'T') {
            initial_position = Dimension { x, y };
        }
        board.push(row);
    }
    (k, board, initial_position)
}