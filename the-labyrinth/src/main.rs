use crate::input::{read_config, read_turn};
use crate::structures::{Config, Position};

mod structures;
mod input;
mod game;


fn main() {
    let config = read_config();

    loop {
        let turn = read_turn(config.rows);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", turn.play()); // Rick's next move (UP DOWN LEFT or RIGHT).
    }
}
