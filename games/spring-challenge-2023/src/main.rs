use crate::game::game::Action;
use crate::game::input::{build_map, parse_initial_inputs, update_tiles};

mod game;

fn main() {
    let mut game = parse_initial_inputs();
    let mut initial_tile = game.tiles.get(&game.bases[0]).unwrap().to_owned();
    build_map(&mut game.tiles, &mut initial_tile);

    loop {
        update_tiles(&mut game.tiles);
        game.update();

        let output = game.play().iter().map(|a| match a {
            Action::WAIT => String::from("WAIT"),
            Action::LINE(source_idx, target_idx, strength) => format!("LINE {} {} {}", source_idx, target_idx, strength),
            Action::BEACON(cell_idx, strength) => format!("BEACON {} {}", cell_idx, strength),
            Action::MESSAGE(text) => format!("MESSAGE {}", text),
        }).collect::<Vec<_>>().join(" | ");

        println!("{}", output);
    }
}
