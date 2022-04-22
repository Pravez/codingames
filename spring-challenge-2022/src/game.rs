use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::hero::Hero;
use crate::lib::input::{Config, Turn};
use crate::lib::structures::UnitType;
use crate::lib::unit::Unit;
use crate::monster::Monster;
use crate::Vec2;

pub struct Game<'a> {
    pub base: Vec2<i32>,
    pub heroes: HashMap<i32, Hero<'a>>,
    pub monsters: HashMap<i32, Monster>,
}

impl<'a> Game<'a> {
    const HERO_VIEW_RADIUS: u32 = 3000;

    pub fn new(config: &Config) -> Self {
        Game {
            base: config.base.clone(),
            heroes: HashMap::new(),
            monsters: HashMap::new(),
        }
    }

    pub fn update(&mut self, turn: &Turn) {
        for x in &turn.units {
            match x.unit_type {
                UnitType::MONSTER => {
                    if !self.monsters.contains_key(&x.id) {
                        self.monsters.insert(x.id, Monster::new(x.id, x.position, x.health, x.threat_for == 1));
                    } else if x.health <= 0 {
                        self.monsters.remove(&x.id);
                    } else {
                        self.monsters.get_mut(&x.id).map(|m| m.update(&x.position, x.health));
                    }
                }
                UnitType::HERO => {
                    if !self.heroes.contains_key(&x.id) {
                        self.heroes.insert(x.id, Hero::new(x.id, x.position, x.health, Game::HERO_VIEW_RADIUS));
                    } else {
                        self.heroes.get_mut(&x.id).map(|h| h.update_position(&x.position));
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update_threats(&'a mut self) {
        let threats = self.monsters.values()
            .filter(|m| m.is_threat)
            .collect::<Vec<_>>();
        self.heroes.values_mut().map(|h| h.update_threats(&threats));
    }

    // In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
    pub fn play(&mut self, hero: &i32) -> String {
        self.heroes.get(hero).map(|h| h.next_move()).unwrap()
    }
}