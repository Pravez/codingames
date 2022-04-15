


fn main() {
    let config = read_config();

    loop {
        let turn = read_turn(config.rows);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", turn.play()); // Rick's next move (UP DOWN LEFT or RIGHT).
    }
}

pub struct Turn {
    pub k: Dimension,
    pub board: Vec<Vec<char>>,
    pub initial_position: Dimension,
}

impl Turn {
    pub fn play(&self) -> String {
        match self.move_to_console() {
            None => String::new(),
            Some(it) => it
        }
    }

    fn move_to_console(&self) -> Option<String> {
        for x in self.k.x - 1..self.k.x + 1 {
            for y in self.k.y - 1..self.k.y + 1 {
                if self.board[x][y] == 'C' {
                    testsddsf
                }
            }
        }
        return Some(String::new());
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
    pub y: usize
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

pub fn read_turn(rows: i32) -> Turn {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let k = Position { x: parse_input!(inputs[0], i32), y: parse_input!(inputs[1], i32) };
    let mut board = Vec::new();
    let mut initial_position = Position::default();
    for x in 0..rows as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim().chars().collect::<Vec<_>>();
        if let Some(y) = row.iter().position(|&r| r == 'T') {
            initial_position = Position { x: x as i32, y: y as i32 };
        }
        board.push(row);
    }
    Turn {
        k,
        board,
        initial_position,
    }
}