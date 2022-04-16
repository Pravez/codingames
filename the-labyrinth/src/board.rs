use std::collections::HashMap;
use crate::structures::{Dimension};

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<Cell>>,
}

pub struct Cell {
    pub pos: Dimension,
    pub value: char,
    pub is_wall: bool,

    pub directions: HashMap<String, Cell>,
}

pub struct Path {
    pub starting_position: &'static Dimension,
    pub entries: Vec<PathEntry>,
}

pub struct PathEntry {
    pub direction: String,
    pub cell: Cell,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut entries = Vec::new();
        for y in 0..rows {
            entries.push((0..rows).map(|x| {
                Cell { pos: Dimension { x, y }, value: '?', is_wall: true, directions: HashMap::default() }
            }).collect::<Vec<Cell>>())
        }
        for y in 0..rows {
            for x in 0..cols {
                let mut directions: Vec<(String, &Cell)> = Vec::new();
                match y {
                    y if y > 0 => directions.push((String::from("LEFT"), &entries[y - 1][x])),
                    y if y < rows - 1 => directions.push((String::from("RIGHT"), &entries[y + 1][x])),
                    _ => {}
                }
                match x {
                    x if x > 0 => directions.push((String::from("TOP"), &entries[y][x - 1])),
                    x if x < cols - 1 => directions.push((String::from("BOTTOM"), &entries[y][x + 1])),
                    _ => {}
                }
            }
        }
        Board { rows, cols, cells: entries }
    }

    pub fn update(&mut self, board: &Vec<Vec<char>>) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                self.cells[x][y].update(board[x][y]);
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        return self.cells[x][y].value;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        return &self.cells[x][y];
    }
}

impl Cell {
    pub fn update(&mut self, value: char) {
        self.value = value;
        if let '#' = value { self.is_wall = true }
    }
}

impl Path {
    pub fn new(starting_position: &'static Dimension) -> Self {
        Path {
            starting_position,
            entries: Vec::new(),
        }
    }
}