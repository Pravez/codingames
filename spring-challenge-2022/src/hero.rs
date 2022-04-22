
use crate::{impl_unit, Vec2};
use crate::lib::base::generics::VecOps;
use crate::lib::unit::Unit;
use crate::monster::Monster;

impl_unit!(Hero, 'a);

#[derive(Eq, Debug)]
pub struct Hero<'a> {
    pub id: i32,
    pub position: Vec2<u32>,
    pub health: i32,
    pub view_radius: u32,
    pub in_range: Vec<i32>,
    pub pursuing: Option<&'a i32>,
}

impl<'a> Hero<'a> {
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

    pub fn update_threats(&mut self, units: &Vec<&'a Monster>) {
        self.in_range.clear();
        let mut sight = units.clone();
        sight.sort_by(|a, b| (self.position - a.position).len().partial_cmp(&(self.position - b.position).len()).unwrap());
        self.in_range.extend(sight.iter().map(|m|m.id));
        if self.pursuing.is_some() && !self.in_range.contains(&self.pursuing.unwrap()) {
            self.pursuing = None;
        }
        if self.pursuing.is_none() && self.in_range.len() > 0 {
            self.pursuing = self.in_range.first();
            eprintln!("(Hero {}) Found {} to pursue !", self.id, self.pursuing.unwrap());
        }
        eprintln!("Hero {} : {:?} monsters", self.id, self.in_range);
    }

    pub fn next_move(&self) -> String {
        if self.pursuing.is_some() {
            eprintln!("(Hero {}) Pursuing {} ({})", self.id, self.pursuing.unwrap().id, self.pursuing.unwrap().position);
            return format!("MOVE {} {}", self.pursuing.unwrap().position.x, self.pursuing.unwrap().position.y);
        }

        return String::from("WAIT");
    }
}

