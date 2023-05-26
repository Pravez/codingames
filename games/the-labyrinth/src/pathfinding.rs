use std::collections::HashMap;

use crate::base::generics::AbsDiff;
use crate::base::queue::{PriorityQueue, PriorityQueueTrait};
use crate::base::vec::Vec2;
use crate::vec2;

pub struct Pathfinding {
    pub start: Vec2<usize>,
    pub end: Vec2<usize>,

    cells: Vec<Vec<Cell>>,
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
struct Cell {
    pos: Vec2<usize>,
    neighbours: Vec<Vec2<usize>>,
}

impl Cell {
    fn new(pos: Vec2<usize>) -> Self {
        Cell { pos, neighbours: vec![] }
    }
}

impl Pathfinding {
    const NEIGHBOURS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    pub fn new(start: Vec2<usize>, end: Vec2<usize>, dimensions: Vec2<usize>) -> Self {
        let mut cells = vec![];
        for x in 0..dimensions.x {
            cells.push((0..dimensions.y).map(|y| Cell::new(vec2!(x, y))).collect());
        }
        Pathfinding {
            start,
            end,
            cells,
        }
    }

    pub fn update_start(&mut self, nstart: Vec2<usize>) {
        eprintln!("Start is {}, end is {}", self.start, self.end);
        self.start = nstart;
    }

    pub fn populate_cells(&mut self, ncells: &Vec<Vec<bool>>) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                match ncells[x][y] {
                    false => continue,
                    true => {}
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
                            true => self.cells[x][y].neighbours.push(vec2!(nx, ny)),
                            false => {}
                        }
                    });
            }
        }
    }

    pub fn find_path(&self) -> Vec<Vec2<usize>> {
        let mut frontier = PriorityQueue::new();
        frontier.insert(self.start, 0);

        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();
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
                }
            }
        }

        let mut current = self.end;
        let mut previous = self.end;
        let mut path: Vec<Vec2<usize>> = Vec::new();
        while current != self.start {
            path.add(current);
            previous = current;
            current = came_from[&current];
        }
        path.reverse();

        return path;
    }

    fn heuristic(a: Vec2<usize>, b: Vec2<usize>) -> usize {
        a.abs_diff(b).components_sum()
    }
}