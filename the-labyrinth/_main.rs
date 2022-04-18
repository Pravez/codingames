
pub struct Game {
    pub config: Config,
    pub console_found: bool,
    pub path: Path,

    pub k: Dimension,
    pub board: Board,
    pub initial_position: Dimension,
}

impl Game {
    const POSSIBILITIES: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    pub fn new(config: Config) -> Self {
        let initial_position = Default::default();
        let dimensions = Dimension { x: config.rows as usize, y: config.cols as usize };
        Game {
            config,
            k: Default::default(),
            board: Board::new(dimensions),
            initial_position,
            path: Path::new(initial_position),
            console_found: false,
        }
    }

    pub fn update(&mut self, (k, board, initial_position): (Dimension, Vec<Vec<char>>, Dimension)) {
        self.k = k;
        self.board.update(&board);
        self.initial_position = initial_position;
        eprintln!("Updated");
    }

    pub fn play(&mut self) -> String {
        eprintln!("Playing");
        if self.console_found {
            return self.go_back();
        }

        let action = match self.move_to_console() {
            None => self.try_move(),
            Some(it) => it
        };
        self.record(&action);
        action
    }

    fn record(&mut self, selected_move: &String) {
        self.path.entries.push(PathEntry {
            cell: self.board.get_cell(self.k.x, self.k.y).clone(),
            direction: selected_move.clone(),
        });
        eprintln!("Added record {}", self.path.entries.last().unwrap());
    }

    fn go_back(&mut self) -> String {
        let last = self.path.entries.pop().unwrap();
        opposite_from(&last.direction).unwrap()
    }

    fn try_move(&self) -> String {
        Game::POSSIBILITIES
            .to_vec()
            .iter()
            .find(|(x, y)| self.can_do(*x, *y))
            .map(|(x, y)| position_to_direction(*x, *y).unwrap())
            .unwrap()
    }

    fn move_to_console(&mut self) -> Option<String> {
        for (x, y) in Game::POSSIBILITIES.to_vec() {
            if self.relative_access(x, y) == 'C' {
                eprintln!("Found nearby console");
                self.console_found = true;
                match position_to_direction(x, y) {
                    Some(r) => return Some(r),
                    _ => {
                        self.console_found = false;
                    }
                }
            }
        }
        None
    }

    fn relative_access(&self, x: i32, y: i32) -> char {
        let (cx, cy) = (self.k.x as i32 + x, self.k.y as i32 + y);
        assert!(x < self.config.rows && y < self.config.cols);
        self.board.get(cx as usize, cy as usize)
    }

    fn can_do(&self, x: i32, y: i32) -> bool {
        let possible = match self.relative_access(x, y) {
            '#' => false,
            '.' => true,
            'T' => true,
            'C' => true,
            '?' => false,
            _ => false
        };
        let dejavu = self.path.exists(self.k.x + x as usize, self.k.y + y as usize);
        eprintln!("({}, {}) : {}, dejavu {}", x, y, possible, dejavu);
        possible && !dejavu
    }
}
use std::fmt::{Display, Formatter};

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

macro_rules! dim {
    ($x:expr, $y:expr) => (Dimension {x: $x, y: $y})
}

pub struct Pathfinding {
    pub start: Dimension,
    pub end: Dimension,

    pub cells: Vec<Vec<PCell>>,

    open_list: Vec<PCell>,
    closed_list: Vec<PCell>,
}

#[derive(Default, Clone)]
struct PCell {
    pos: Dimension,
    g: i32,
    h: i32,

    neighbours: Vec<&'static PCell>,
}

impl PCell {
    fn new(pos: Dimension) -> Self {
        PCell { pos, g: 0, h: 0, neighbours: vec![] }
    }

    fn f(&self) -> i32 { self.g + self.h }
}

impl Pathfinding {
    const NEIGHBOURS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    const EMPTY: char = '.';
    const WALL: char = '#';

    fn new(start: Dimension, end: Dimension, dimensions: Dimension) -> Self {
        let mut cells = vec![];
        for x in 0..dimensions.x {
            cells.push((0..dimensions.y).map(|y| PCell::new(dim!(x, y))).collect());
        }
        Pathfinding {
            start,
            end,
            cells,
            open_list: vec![],
            closed_list: vec![],
        }
    }

    fn populate_cells(&mut self, ncells: &'static Vec<Vec<char>>) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                Pathfinding::NEIGHBOURS
                    .iter()
                    .map(|(mx, my)| (x + *mx as usize, y + *my as usize))
                    .filter(|(nx, ny)| *nx > 0 as usize && *ny > 0 as usize && nx < &self.cells.len() && ny < &self.cells[x].len())
                    .for_each(|(nx, ny)| {
                        match ncells[nx][ny] {
                            EMPTY => self.cells[x][y].neighbours.push(&self.cells[nx][ny]),
                            _ => {}
                        }
                    })
            }
        }
    }
}#[derive(Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Config {
    pub rows: i32,
    pub cols: i32,
    pub alarm: i32,
}

#[derive(Default, Clone, Copy)]
pub struct Dimension {
    pub x: usize,
    pub y: usize,
}

pub fn position_to_direction(x: i32, y: i32) -> Option<String> {
    match (x, y) {
        (0, -1) => Some("LEFT"),
        (0, 1) => Some("RIGHT"),
        (-1, 0) => Some("UP"),
        (1, 0) => Some("DOWN"),
        _ => Option::None
    }.map(|r| String::from(r))
}

pub fn direction_to_position(x: usize, y: usize, direction: &String) -> (usize, usize) {
    match direction.as_str() {
        "LEFT" => (x, y - 1),
        "RIGHT" => (x, y + 1),
        "UP" => (x - 1, y),
        "DOWN" => (x + 1, y),
        _ => (x, y)
    }
}

pub fn opposite_from(direction: &String) -> Option<String> {
    match direction.as_str() {
        "LEFT" => Some("RIGHT"),
        "RIGHT" => Some("LEFT"),
        "UP" => Some("DOWN"),
        "DOWN" => Some("UP"),
        _ => None,
    }.map(|r| String::from(r))
}



fn main() {
    let mut game = Game::new(read_config());

    loop {
        game.update(read_turn(game.config.rows));
        println!("{}", game.play());
    }
}
use std::io;

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