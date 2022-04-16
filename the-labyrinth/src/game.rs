use crate::board::{Board, Path};
use crate::Config;
use crate::structures::{Dimension, position_to_direction};

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
        Game {
            config,
            k: Default::default(),
            board: Board::new(config.rows.clone() as usize, config.cols.clone() as usize),
            initial_position,
            path: Path::new(&initial_position), console_found: false
        }
    }

    pub fn update(&mut self, (k, board, initial_position): (Dimension, Vec<Vec<char>>, Dimension)) {
        self.k = k;
        self.board.update(&board);
        self.initial_position = initial_position;
    }

    pub fn play(&self) -> String {
        match self.move_to_console() {
            None => Game::POSSIBILITIES.iter().find(|(&x, &y)| self.can_do(x, y)).map(|(&x, &y)| position_to_direction(x, y).unwrap()),
            Some(it) => it
        }
    }

    fn record(&self, selected_move: String) {

    }

    fn try_move(&self) -> String {
        return String::from("");
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
        return self.board.get(cy as usize, cx as usize);
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
}