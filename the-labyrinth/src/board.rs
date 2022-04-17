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

    //pub directions: HashMap<String, Cell>,
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

impl Board {
    pub fn new(dims: Dimension) -> Self {
        let mut entries = Vec::new();
        for y in 0..dims.y {
            entries.push((0..dims.x).map(|x| {
                Cell { pos: Dimension { x, y }, value: '?', is_wall: true }
            }).collect::<Vec<Cell>>())
        }
        for y in 0..dims.y {
            for x in 0..dims.x {
                let mut directions: Vec<(String, &Cell)> = Vec::new();
                match y {
                    y if y > 0 => directions.push((String::from("LEFT"), &entries[y - 1][x])),
                    y if y < dims.y - 1 => directions.push((String::from("RIGHT"), &entries[y + 1][x])),
                    _ => {}
                }
                match x {
                    x if x > 0 => directions.push((String::from("TOP"), &entries[y][x - 1])),
                    x if x < dims.x - 1 => directions.push((String::from("BOTTOM"), &entries[y][x + 1])),
                    _ => {}
                }
            }
        }
        Board { rows: dims.y, cols: dims.x, cells: entries }
    }

    pub fn update(&mut self, board: &Vec<Vec<char>>) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                self.cells[y][x].update(board[y][x]);
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
        for cell in self.entries.as_slice() {
            if cell.cell.pos.x == x && cell.cell.pos.y == y {
                return true
            }
        }
        return false;
    }
}
