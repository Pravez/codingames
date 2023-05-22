use crate::lib::base::generics::VecOps;
use crate::lib::base::itertools::display_vec;
use crate::lib::base::vec::Vec2;
use crate::lib::input::Setup;

const MAX_X: i32 = 16000;
const MAX_Y: i32 = 9000;

pub fn heuristic(setup: Setup) -> Vec2<i32> {
    let mut zombies = setup.zombies.to_vec();
    zombies.sort_by(|a, b| {
        a.next_position.distance(setup.player).partial_cmp(&b.next_position.distance(setup.player)).unwrap()
    });
    let mut dangerous_zombies = setup.zombies.to_vec();
    dangerous_zombies.sort_by(|a, b| {
        setup.humans
            .iter()
            .map(|h| h.position.distance(a.next_position))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .partial_cmp(&setup.humans
                .iter()
                .map(|h| h.position.distance(b.next_position))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap())
            .unwrap()
    });
    eprintln!("{}", display_vec(&zombies));

    dangerous_zombies.first().unwrap().position
}

pub fn play(setup: Setup) {
    let position = heuristic(setup);
    println!("{} {}", position.x, position.y); // Your destination coordinates
}