use crate::board::{Board, Path, PathEntry};
use crate::Config;
use crate::pathfinding::Pathfinding;
use crate::structures::{Dimension, direction_to_position, movement_between, opposite_from, position_to_direction};

pub struct Game {
    pub config: Config,
    pub console_found: bool,
    pub path: Path,

    pub k: Dimension,
    pub board: Board,
    pub initial_position: Dimension,

    pathfinder: Option<Pathfinding>,
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
            pathfinder: None,
        }
    }

    pub fn update(&mut self, (k, board, initial_position): (Dimension, Vec<Vec<char>>, Dimension)) {
        self.k = k;
        self.board.update(&board);
        self.initial_position = initial_position;
        self.pathfinder.as_mut().map(|p| {
            p.update_start(k);
            p.populate_cells(&board);
        });
    }

    pub fn play(&mut self) -> String {
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
    }

    fn go_back(&self) -> String {
        let next_move = self.pathfinder.as_ref().map(|p| p.next_move()).unwrap();
        eprintln!("Next move towards {}", next_move);
        opposite_from(&movement_between(self.k, next_move).unwrap()).unwrap()
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
                self.pathfinder = Option::from(Pathfinding::new(self.k, self.initial_position, Dimension { x: self.config.rows as usize, y: self.config.cols as usize }));
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
            '.' | 'T' | 'C' => true,
            _ => false
        };
        let dejavu = self.path.exists(self.k.x + x as usize, self.k.y + y as usize);
        eprintln!("({}, {}) : {}, dejavu {}", x, y, possible, dejavu);
        possible && !dejavu
    }
}
