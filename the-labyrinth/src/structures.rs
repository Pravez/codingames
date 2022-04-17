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
        (-1, 0) => Some("LEFT"),
        (1, 0) => Some("RIGHT"),
        (0, -1) => Some("TOP"),
        (0, 1) => Some("BOTTOM"),
        _ => Option::None
    }.map(|r| String::from(r))
}

pub fn direction_to_position(x: usize, y: usize, direction: &String) -> (usize, usize) {
    match direction.as_str() {
        "LEFT" => (x-1, y),
        "RIGHT" => (x+1, y),
        "TOP" => (x, y-1),
        "BOTTOM" => (x, y+1),
        _ => (x, y)
    }
}

pub fn opposite_from(direction: &String) -> Option<String> {
    match direction.as_str() {
        "LEFT" => Some("RIGHT"),
        "RIGHT" => Some("LEFT"),
        "TOP" => Some("BOTTOM"),
        "BOTTOM" => Some("TOP"),
        _ => None,
    }.map(|r| String::from(r))
}
