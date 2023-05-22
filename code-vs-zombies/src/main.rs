mod lib;
mod game;

/**
 * Save humans, destroy zombies!
 **/
fn main() {

    loop {
        game::play(lib::input::parse_inputs());
    }
}
