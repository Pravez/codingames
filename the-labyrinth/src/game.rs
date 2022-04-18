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
