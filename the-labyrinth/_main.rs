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
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let k = Dimension { y: parse_input!(inputs[0], i32) as usize, x: parse_input!(inputs[1], i32) as usize };
    let mut board = Vec::new();
    let mut initial_position = Dimension::default();
    for x in 0..rows as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim().chars().collect::<Vec<_>>();
        if let Some(y) = row.iter().position(|&r| r == 'T') {
            initial_position = Dimension { y, x };
        }
        board.push(row);
    }
    eprintln!("Parsed");
    (k, board, initial_position)
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
        (-1, 0) => Some("LEFT"),
        (1, 0) => Some("RIGHT"),
        (0, -1) => Some("TOP"),
        (0, 1) => Some("BOTTOM"),
        _ => Option::None
    }.map(|r| String::from(r))
}

pub fn direction_to_position(x: usize, y: usize, direction: &String) -> (usize, usize) {
    match direction.as_str() {
        "LEFT" => (x-1, y),
        "RIGHT" => (x+1, y),
        "TOP" => (x, y-1),
        "BOTTOM" => (x, y+1),
        _ => (x, y)
    }
}

pub fn opposite_from(direction: &String) -> Option<String> {
    match direction.as_str() {
        "LEFT" => Some("RIGHT"),
        "RIGHT" => Some("LEFT"),
        "TOP" => Some("BOTTOM"),
        "BOTTOM" => Some("TOP"),
        _ => None,
    }.map(|r| String::from(r))
}

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



fn main() {
    let mut game = Game::new(read_config());

    loop {
        game.update(read_turn(game.config.rows));
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", game.play()); // Rick's next move (UP DOWN LEFT or RIGHT).
    }
}

pub struct Game {
    pub config: Config,
    pub console_found: bool,
    pub path: Path,

    pub k: Dimension,
    pub board: Board,
    pub initial_position: Dimension,
}

impl Game {
    const POSSIBILITIES: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    pub fn new(config: Config) -> Self {
        let initial_position = Default::default();
        let dimensions = Dimension { x: config.cols as usize, y: config.rows as usize };
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
        let npos = direction_to_position(self.k.x, self.k.y, selected_move);
        self.path.entries.push(PathEntry {
            cell: self.board.get_cell(npos.0, npos.1).clone(),
            direction: selected_move.clone()
        });
    }

    fn go_back(&self) -> String {
        opposite_from(self.path.entries.last().map(|c| &c.direction).unwrap()).unwrap()
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
        assert!(x < self.config.cols && y < self.config.rows);
        self.board.get(cy as usize, cx as usize)
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
        let dejavu = self.path.exists(x as usize, y as usize);
        possible && dejavu
    }
}