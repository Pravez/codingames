use crate::game::game::Action;
use crate::game::input::parse_initial_inputs;

mod game;

fn main() {
    let mut game = parse_initial_inputs();

    loop {
        game.update();

        let actions = game.play();

        let output = actions.into_iter().map(|a| match a {
            Action::WAIT => String::from("WAIT"),
            Action::LINE(source_idx, target_idx, strength) => format!("LINE {} {} {}", source_idx, target_idx, strength),
            Action::BEACON(cell_idx, strength) => format!("BEACON {} {}", cell_idx, strength),
            Action::MESSAGE(text) => format!("MESSAGE {}", text),
        }).collect::<Vec<_>>().join(" | ");

        println!("{}", output);
    }
}
