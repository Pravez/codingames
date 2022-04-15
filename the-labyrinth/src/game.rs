use crate::structures::Dimension;

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
}