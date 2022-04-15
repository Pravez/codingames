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

#[derive(Default)]
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