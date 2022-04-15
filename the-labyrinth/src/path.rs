use std::collections::HashMap;
use crate::structures::{Dimension};

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