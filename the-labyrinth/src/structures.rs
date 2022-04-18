#[derive(Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Config {
    pub rows: i32,
    pub cols: i32,
    pub alarm: i32,
}

#[derive(Default, Clone, Copy)]
pub struct Dimension {
    pub x: usize,
    pub y: usize,
}

pub fn position_to_direction(x: i32, y: i32) -> Option<String> {
    match (x, y) {
        (0, -1) => Some("LEFT"),
        (0, 1) => Some("RIGHT"),
        (-1, 0) => Some("UP"),
        (1, 0) => Some("DOWN"),
        _ => Option::None
    }.map(|r| String::from(r))
}

pub fn direction_to_position(x: usize, y: usize, direction: &String) -> (usize, usize) {
    match direction.as_str() {
        "LEFT" => (x, y - 1),
        "RIGHT" => (x, y + 1),
        "UP" => (x - 1, y),
        "DOWN" => (x + 1, y),
        _ => (x, y)
    }
}

pub fn opposite_from(direction: &String) -> Option<String> {
    match direction.as_str() {
        "LEFT" => Some("RIGHT"),
        "RIGHT" => Some("LEFT"),
        "UP" => Some("DOWN"),
        "DOWN" => Some("UP"),
        _ => None,
    }.map(|r| String::from(r))
}
