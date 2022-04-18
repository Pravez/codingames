use std::fmt::{Display, Formatter};
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
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        return Cell {
            pos: self.pos.clone(),
            value: self.value,
            is_wall: self.is_wall,
        }
    }
}

pub struct Path {
    pub starting_position: Dimension,
    pub entries: Vec<PathEntry>,
}

pub struct PathEntry {
    pub direction: String,
    pub cell: Cell,
}

impl Display for PathEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> ({}, {})", self.direction, self.cell.pos.x, self.cell.pos.y)
    }
}

impl Board {
    pub fn new(dims: Dimension) -> Self {
        let mut entries = Vec::new();
        for x in 0..dims.x {
            entries.push((0..dims.y).map(|y| {
                Cell { pos: Dimension { x, y }, value: '?', is_wall: true }
            }).collect::<Vec<Cell>>())
        }
        Board { rows: dims.x, cols: dims.y, cells: entries }
    }

    pub fn update(&mut self, board: &Vec<Vec<char>>) {
        for x in 0..self.rows {
            for y in 0..self.cols {
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
    pub fn new(starting_position: Dimension) -> Self {
        Path {
            starting_position,
            entries: Vec::new(),
        }
    }

    pub fn exists(&self, x: usize, y: usize) -> bool {
        eprintln!("Checking ({}, {})...", x, y);
        for cell in self.entries.as_slice() {
            if cell.cell.pos.x == x && cell.cell.pos.y == y {
                return true
            }
        }
        return false;
    }
}
