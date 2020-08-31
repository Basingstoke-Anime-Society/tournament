mod model;
mod mock;

use model::*;
use mock::*;

use dialoguer::Input;

fn main() {
    // start with a mock model
    let mut model = MockModel::new();
    model.add_mock_anime();

    // make a tournament
    let items = model.get_anime_for_slot(Slot::First);
    let len = items.len();
    let mut tournament = Tournament {
        slot: Slot::First,
        items: items,
        decisions: Vec::with_capacity(len)
    };
    let slot = tournament.slot.name();
    println!("Tournament has {} anime for {} slot", tournament.items.len(), slot);

    // run the tournament
    while !tournament.is_finished() {
        let decision = tournament.next_decision();
        match decision {
            Some(mut decision) => {
                let left = model.get_anime_by_id(decision.left).unwrap();
                let right = model.get_anime_by_id(decision.right).unwrap();

                decision.pick = ask_pick(left.name, right.name);

                println!(" => {}", decision.pick.name());
                
                tournament.add_decision(decision);
            },
            None => ()
        }
    }
    let winner = tournament.get_winner().unwrap();
    let winner_anime = model.get_anime_by_id(winner).unwrap();
    println!("WINNER: {}", winner_anime.name);
}

fn ask_pick(left: String, right: String) -> Pick {
    println!("Decision: {} vs {}", left, right);

    let input = Input::<String>::new().with_prompt("[L/R] ").interact().unwrap();
    match input.as_str() {
        "l"|"L" => Pick::Left,
        "r"|"R" => Pick::Right,
        _ => ask_pick(left, right)
    }
}
