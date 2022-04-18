use std::collections::HashMap;

use crate::priority_queue::{PriorityQueue, PriorityQueueTrait};
use crate::structures::Dimension;

macro_rules! dim {
    ($x:expr, $y:expr) => (Dimension {x: $x, y: $y})
}

pub struct Pathfinding {
    pub start: Dimension,
    pub end: Dimension,

    cells: Vec<Vec<PCell>>,
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
struct PCell {
    pos: Dimension,

    neighbours: Vec<Dimension>,
}

impl PCell {
    fn new(pos: Dimension) -> Self {
        PCell { pos, neighbours: vec![] }
    }
}

impl Pathfinding {
    const NEIGHBOURS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    pub fn new(start: Dimension, end: Dimension, dimensions: Dimension) -> Self {
        let mut cells = vec![];
        for x in 0..dimensions.x {
            cells.push((0..dimensions.y).map(|y| PCell::new(dim!(x, y))).collect());
        }
        Pathfinding {
            start,
            end,
            cells,
        }
    }

    pub fn update_start(&mut self, nstart: Dimension) {
        eprintln!("Start is {}, end is {}", self.start, self.end);
        self.start = nstart;
    }

    pub fn populate_cells(&mut self, ncells: &Vec<Vec<char>>) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                match ncells[x][y] {
                    '#'|'?' => continue,
                    _ => {}
                }
                self.cells[x][y].neighbours.clear();
                Pathfinding::NEIGHBOURS
                    .iter()
                    .map(|(mx, my)| (x + *mx as usize, y + *my as usize))
                    .filter(|(nx, ny)| *nx > 0 as usize && *ny > 0 as usize && nx < &self.cells.len() && ny < &self.cells[x].len())
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|(nx, ny)| {
                        match ncells[nx][ny] {
                            '.' | 'T' | 'C' => self.cells[x][y].neighbours.push(dim!(nx, ny)),
                            _ => {}
                        }
                    });
            }
        }
    }

    pub fn next_move(&self) -> Dimension {
        let mut frontier = PriorityQueue::new();
        frontier.insert(self.start, 0);

        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();
        let mut to = HashMap::new();
        cost_so_far.insert(self.start, 0);

        while !frontier.is_empty() {
            match frontier.peek() {
                None => {}
                Some(x) => { if *x == self.end { break; } }
            }

            let current = frontier.pop().unwrap();

            for next in self.cells[current.x][current.y].neighbours.as_slice() {
                let new_cost = cost_so_far[&current] + 1;
                if !cost_so_far.contains_key(next) || new_cost < cost_so_far[next] {
                    cost_so_far.insert(*next, new_cost);
                    let priority = new_cost + Pathfinding::heuristic(self.end, next.clone());
                    frontier.insert(next.clone(), priority as u8);
                    came_from.insert(*next, current);
                    to.insert(current, *next);
                }
            }
        }

        let mut current = self.end;
        let mut previous = self.end;
        while current != self.start {
            previous = current;
            current = came_from[&current];
        }
        came_from.iter().for_each(|(k, v)| eprintln!("{} -> {}", k, v));
        eprintln!("k {}, v {}", &self.start, to[&self.start]);

        return previous;
    }

    fn heuristic(a: Dimension, b: Dimension) -> usize {
        let dx = (a.x as i32 - b.x as i32).abs();
        let dy = (a.y as i32 - b.y as i32).abs();
        (dx + dy) as usize
    }
}