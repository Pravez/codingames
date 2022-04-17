use crate::board::{Board, Path, PathEntry};
use crate::Config;
use crate::structures::{Dimension, direction_to_position, opposite_from, position_to_direction};

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