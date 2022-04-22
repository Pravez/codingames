pub enum UnitType {
    MONSTER = 0,
    HERO = 1,
    OPPONENT = 2
}

impl UnitType {
    pub fn from_int(value: i32) -> UnitType {
        match value {
            0 => UnitType::MONSTER,
            1 => UnitType::HERO,
            2 => UnitType::OPPONENT,
            _ => panic!("Unkown value {}", value)
        }
    }
}