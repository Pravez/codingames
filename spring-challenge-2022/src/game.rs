use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Index;

use crate::{Vec2, vec2};
use crate::lib::base::alg::place_on_circle_at;
use crate::lib::constants::BASE_SIGHT_RADIUS;
use crate::lib::input::{Config, InputUnit, Turn};
use crate::lib::structures::UnitType;
use crate::hero::Hero;
use crate::monster::Monster;

pub struct Game {
    pub base: Vec2<i32>,
    pub heroes: HashMap<i32, Hero>,
    pub threats: Vec<Monster>,
    pub monsters: HashMap<i32, Monster>
}

impl Game {
    const HERO_VIEW_RADIUS: u32 = 3000;

    pub fn new(config: &Config) -> Self {
        Game {
            base: config.base.clone(),
            heroes: HashMap::new(),
            threats: Vec::new(),
            monsters: HashMap::new()
        }
    }

    fn update_units(&mut self, units: &Vec<InputUnit>) {
        for x in units {
            match x.unit_type {
                UnitType::MONSTER => {
                    if !self.monsters.contains_key(&x.id) {
                        self.monsters.insert(x.id, Monster::new(x.id, x.position, x.health, 0));
                    } else if x.health <= 0 {
                        self.monsters.remove(&x.id);
                    } else {
                        self.monsters.get_mut(&x.id).map(|m| {
                            m.update_position(&x.position);
                            m.health = x.health;
                        });
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
        self.threats = self.monsters.values().map(|m|m.)
    }

    fn update_sights(&mut self) {
        let monsters = self.threats.values().collect::<Vec<_>>();
        for mut h in self.heroes.values_mut() {
            h.update_range(&monsters);
        }
    }

    pub fn update(&mut self, turn: &Turn) {
        self.update_units(&turn.units);
        self.update_sights();
    }

    // In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
    pub fn play(&mut self, hero: &i32) -> String {
        self.heroes.get_mut(hero).map(|h| h.next_move()).unwrap()
    }
}