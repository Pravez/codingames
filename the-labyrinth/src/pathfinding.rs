use crate::structures::Dimension;

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
}