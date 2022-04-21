use std::collections::HashMap;

use crate::{Vec2, vec2};
use crate::lib::input::{Config, InputUnit, Turn};
use crate::lib::structures::UnitType;
use crate::unit::Unit;

pub struct Game {
    pub base: Vec2<u32>,
    pub heroes: HashMap<i32, Unit>,
    pub monsters: HashMap<i32, Unit>,
}

impl Game {
    const HERO_VIEW_RADIUS: u32 = 2000;

    pub fn new(config: &Config) -> Self {
        Game {
            base: config.base.clone(),
            heroes: HashMap::new(),
            monsters: HashMap::new(),
        }
    }

    fn update_units(&mut self, units: &Vec<InputUnit>) {
        for x in units {
            match x.unit_type {
                UnitType::MONSTER => {
                    if !self.monsters.contains_key(&x.id) {
                        self.monsters.insert(x.id, Unit::new(x.id, x.position, 0));
                    } else {
                        self.monsters.get_mut(&x.id).map(|m| m.update_position(&x.position));
                    }
                }
                UnitType::HERO => {
                    if !self.heroes.contains_key(&x.id) {
                        let hero = Unit::new(x.id, x.position, Game::HERO_VIEW_RADIUS);
                        self.heroes.insert(x.id, hero);
                    } else {
                        self.heroes.get_mut(&x.id).map(|h| h.update_position(&x.position));
                    }
                }
                _ => {}
            }
        }
    }

    fn update_sights(&mut self) {
        let monsters = self.monsters.values().collect::<Vec<_>>();
        for mut h in self.heroes.values_mut() {
            h.update_range(&monsters);
        }
    }

    pub fn update(&mut self, turn: &Turn) {
        self.update_units(&turn.units);
        self.update_sights();
    }

    // In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
    pub fn play(&self, hero: &i32) -> String {
        self.heroes.get(hero).map(|h|h.next_move()).unwrap()
    }
}