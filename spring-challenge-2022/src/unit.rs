use std::fmt::format;

use crate::{Vec2, vec2};
use crate::lib::base::generics::VecOps;

pub struct Unit {
    pub id: i32,
    pub position: Vec2<u32>,
    pub view_radius: u32,
    pub in_range: Vec<i32>,
    pub pursuing: Option<(i32, Vec2<u32>)>,
    pub stand_position: Vec2<u32>,
}

impl Unit {
    pub fn new(id: i32, position: Vec2<u32>, view_radius: u32) -> Self {
        Unit {
            id,
            position,
            view_radius,
            in_range: Vec::new(),
            pursuing: None,
            stand_position: vec2!(),
        }
    }

    pub fn set_standing_at(&mut self, position: Vec2<u32>) {
        self.stand_position = position;
    }

    pub fn can_see(&self, other: &Unit) -> bool {
        (other.position - self.position).len() <= self.view_radius as f32
    }

    pub fn update_position(&mut self, position: &Vec2<u32>) {
        self.position = position.clone();
    }

    pub fn update_range(&mut self, units: &Vec<&Unit>) {
        self.in_range.clear();
        let mut sight = units.iter().filter(|u| self.can_see(u)).collect::<Vec<_>>();
        sight.sort_by(|a, b| (self.position - a.position).len().partial_cmp(&(self.position - b.position).len()).unwrap());
        self.in_range.extend(units.iter().map(|u| u.id));
    }

    pub fn is_pursuing(&self) -> bool { self.pursuing.is_some() }

    pub fn pursue(&mut self, unit: &Unit) {
        self.pursuing = Some((unit.id, unit.position));
    }

    pub fn stop_pursuing(&mut self) {
        self.pursuing = None;
    }

    pub fn next_move(&self) -> String {
        match self.pursuing {
            None => {
                if self.position != self.stand_position {
                    format!("MOVE {} {}", self.stand_position.x, self.stand_position.y)
                } else {
                    String::from("WAIT")
                }
            }
            Some(x) => { format!("MOVE {} {}", x.1.x, x.1.y) }
        }
    }
}

