#[macro_use]
extern crate diesel;
extern crate dotenv;

mod model;

use model::*;
use model::models::*;
// use diesel::prelude::*;

/*
mod model;
use model::*;

mod mock;
use mock::*;
*/

use dialoguer::Input;


fn main() {
    let model = Model::connect();
    
    // let items = model.get_anime_for_slot(Slot::First);
    let tournament = model.add_tournament(Slot::First);
    let items = model.get_tournament_anime(&tournament);
    println!("Tournament #{} has {} anime for {} slot", tournament.id, items.len(), "first");


    if !model.is_tournament_finished(&tournament) {
        println!("Not yet!");
    }


    // run the tournament
    while !model.is_tournament_finished(&tournament) {
        let decision = model.next_tournament_decision(&tournament);
        match decision {
            Some(mut decision) => {
                let left = model.get_anime_by_id(decision.left_anime).unwrap();
                let right = model.get_anime_by_id(decision.right_anime).unwrap();

                decision.pick = Pick::to_opt(ask_pick(left.name, right.name));

                println!(" => {}", Pick::to_pick(decision.pick).name());
                
                model.save_tournament_decision(&tournament, decision);
            },
            None => ()
        }
    }
    let winner = model.get_tournament_winner(&tournament).unwrap();
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
