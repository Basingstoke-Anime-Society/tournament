mod model;
mod mock;

use model::*;
use mock::*;

use rand::{thread_rng, Rng};

fn main() {
    // start with a mock model
    let mut model = MockModel::new();
    model.add_mock_anime();
    println!("Model has {} anime", model.get_anime().len());

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

    // simulate the tournament
    let mut rng = thread_rng();

    while !tournament.is_finished() {
        let decision = tournament.next_decision();
        match decision {
            Some(mut decision) => {
                let left = model.get_anime_by_id(decision.left).unwrap();
                let right = model.get_anime_by_id(decision.right).unwrap();
                println!("Decision: {} vs {}", left.name, right.name);

                let pick: bool = rng.gen();
                decision.pick = match pick {
                    false => Pick::Left,
                    true => Pick::Right,
                };
                println!(" => {}", decision.pick.name());
                
                tournament.add_decision(decision);
            },
            None => ()
        }
    }
    let winner = tournament.get_winner().unwrap();
    let winner_anime = model.get_anime_by_id(winner).unwrap();
    println!("WINNER: {} {}", winner, winner_anime.name);
}
