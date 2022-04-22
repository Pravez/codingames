use std::fmt::format;

use crate::{impl_unit, Vec2, vec2};
use crate::lib::base::generics::VecOps;
use crate::lib::unit::Unit;

impl_unit!(Hero);

pub struct Hero {
    pub id: i32,
    pub position: Vec2<u32>,
    pub health: i32,
    pub view_radius: u32,
    pub in_range: Vec<(i32, Vec2<u32>)>,
    pub pursuing: Option<(i32, Vec2<u32>)>,
}

impl Hero {
    pub fn new(id: i32, position: Vec2<u32>, health: i32, view_radius: u32) -> Self {
        Hero {
            id,
            position,
            health,
            view_radius,
            in_range: Vec::new(),
            pursuing: None,
        }
    }

    pub fn update_range(&mut self, units: &Vec<&Hero>) {
        self.in_range.clear();
        let mut sight = units.iter().filter(|u| self.can_see(u) && u.health > 0).collect::<Vec<_>>();
        sight.sort_by(|a, b| (self.position - a.position).len().partial_cmp(&(self.position - b.position).len()).unwrap());
        self.in_range.extend(sight.iter().map(|u| (u.id, u.position)));
        if self.pursuing.is_some() && !self.in_range.contains(&self.pursuing.unwrap()) {
            self.pursuing = None;
        }
        eprintln!("Hero {} : {:?} monsters", self.id, self.in_range);
    }

    pub fn next_move(&mut self) -> String {
        if self.pursuing.is_none() {
            if self.in_range.len() > 0 {
                self.pursuing = self.in_range.get(0).map(|e| e.clone());
                eprintln!("(Hero {}) Found {} to pursue ! ({})", self.id, self.pursuing.unwrap().0, self.pursuing.unwrap().1);
            } else {
                return String::from("WAIT");
            }
        }

        if self.pursuing.is_some() {
            eprintln!("(Hero {}) Pursuing {} ({})", self.id, self.pursuing.unwrap().0, self.pursuing.unwrap().1);
            return format!("MOVE {} {}", self.pursuing.unwrap().1.x, self.pursuing.unwrap().1.y);
        }
        panic!("Impossible case")
    }
}

