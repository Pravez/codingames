


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

    pub k: Dimension,
    pub board: Vec<Vec<char>>,
    pub initial_position: Dimension,
}

impl Game {
    const POSSIBILITIES: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    pub fn new(config: Config) -> Self {
        Game { config, k: Default::default(), board: Vec::new(), initial_position: Default::default(), console_found: false }
    }

    pub fn update(&mut self, (k, board, initial_position): (Dimension, Vec<Vec<char>>, Dimension)) {
        self.k = k;
        self.board = board;
        self.initial_position = initial_position;
    }

    pub fn play(&self) -> String {
        match self.move_to_console() {
            None => Game::POSSIBILITIES.iter().find(|(&x, &y)| self.can_do(x, y)).map(|(&x, &y)| position_to_direction(x, y).unwrap()),
            Some(it) => it
        }
    }

    fn move_to_console(&mut self) -> Option<String> {
        for (x, y) in Game::POSSIBILITIES {
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
        return None;
    }

    fn relative_access(&self, x: i32, y: i32) -> char {
        let (cx, cy) = (self.k.x as i32 + x, self.k.y as i32 + y);
        assert!(x < self.config.cols && y < self.config.rows);
        return self.board[cy as usize][cx as usize];
    }

    fn can_do(&self, x: i32, y: i32) -> bool {
        match self.relative_access(x, y) {
            '#' => false,
            '.' => true,
            'T' => true,
            'C' => true,
            '?' => false,
            _ => false
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

#[derive(Default)]
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
}use std::io;

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
    (k, board, initial_position)
}use std::collections::HashMap;

pub struct Path {
    pub entries: Vec<Vec<PathEntry>>
}

pub struct PathEntry {
    pub pos: Dimension,
    pub value: char,
    pub is_wall: bool,

    pub directions: HashMap<String, PathEntry>
}

impl Path {
    pub fn new(dimensions: Dimension) -> Self {
        let mut entries = Vec::new();
        for y in 0..dimensions.y {
            entries.push((0..dimensions.x).map(|x| {
                PathEntry { pos: Dimension { x, y }, value: '?', is_wall: true, directions: HashMap::default()}
            }).collect::<Vec<PathEntry>>())
        }
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
                let mut directions: Vec<(String, &PathEntry)> = Vec::new();
                match y {
                    y if y > 0 => directions.push((String::from("LEFT"), &entries[y-1][x])),
                    y if y < dimensions.y => directions.push((String::from("RIGHT"), &entries[y+1][x])),
                    _ => {}
                }
                match x {
                    x if x > 0 => directions.push((String::from("TOP"), &entries[y][x-1])),
                    x if x < dimensions.x => directions.push((String::from("BOTTOM"), &entries[y][x+1])),
                    _ => {}
                }
            }
        }
        Path { entries }
    }
}