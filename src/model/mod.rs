use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::sql_types::{BigInt};

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

use rand::{thread_rng};
use rand::seq::SliceRandom;

// use crate::model::*;

pub mod schema;
pub mod models;

use models::*;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub struct Model {
    pub connection: MysqlConnection
}

#[derive(Debug,QueryableByName)]
struct InsertId {
    #[sql_type="BigInt"]
    pub id: i64
}

impl Model {
    pub fn connect() -> Model {
        let connection = establish_connection();

        Model {
            connection: connection
        }
    }

    fn get_last_inserted_id(&self) -> i32 {
        let result: Result<Vec<InsertId>,_> = diesel::sql_query("SELECT LAST_INSERT_ID() 'id';").load(&self.connection);
        let insertid: i64 = match result {
            Ok(results) => results.first().unwrap().id,
            _ => 0
        };
        insertid as i32
    }

    pub fn add_anime(&self, name: &str, slot1: bool, slot2: bool, slot3: bool) -> Anime {
        use schema::anime;

        let new_anime = Anime {
            id: 0,
            name: name.to_string(),
            episodes: None,
            slot1, 
            slot2,
            slot3
        };

        diesel::insert_into(anime::table)
            .values(&new_anime)
            .execute(&self.connection)
            .expect("Error saving new post");

        new_anime
    }

    pub fn get_anime_by_id(&self, anime_id: i32) -> Option<Anime> {
        use schema::anime::dsl::*;

        anime.filter(id.eq(anime_id))
            .first::<Anime>(&self.connection)
            .optional().ok()?
    }

    pub fn get_anime_for_slot(&self, slot: Slot) -> Vec<Anime> {
        use schema::anime::dsl::*;

        match slot {
            Slot::First => anime.filter(slot1.eq(true))
                .load::<Anime>(&self.connection)
                .expect("Error loading anime"),
            Slot::Second => anime.filter(slot2.eq(true))
                .load::<Anime>(&self.connection)
                .expect("Error loading anime"),
            Slot::Third => anime.filter(slot3.eq(true))
                .load::<Anime>(&self.connection)
                .expect("Error loading anime"),
        }
    }
    
    pub fn add_tournament(&self, slot: Slot) -> Tournament {
        use schema::tournaments;

        // create the tournament
        let mut new_tournament = Tournament {
            id: 0,
        };
        diesel::insert_into(tournaments::table)
            .values(&new_tournament)
            .execute(&self.connection)
            .expect("Error saving new tournament");

        new_tournament.id = self.get_last_inserted_id();

        // add the anime for the given slot
        let anime = self.get_anime_for_slot(slot);
        for a in &anime {
            self.add_anime_to_tournament(&new_tournament, a);
        }

        new_tournament
    }
    
    // Tournament
    pub fn is_tournament_finished(&self, t: &Tournament) -> bool {
        let rem = self.tournament_remaining_anime(t);
        // println!("[is finished] Remaining anime: {:?}", rem);
        rem.len() <= 1
    }

    pub fn get_tournament_anime(&self, t: &Tournament) -> Vec<Anime> {
        use schema::tournament_anime::dsl::*;
        
        let tuples = tournament_anime.filter(tournament.eq(t.id))
            .load::<TournamentAnime>(&self.connection)
            .expect("Error loading tournament anime");

        let mut found: Vec<Anime> = Vec::with_capacity(tuples.len());
        for tuple in &tuples {
            let a = self.get_anime_by_id(tuple.anime);
            match a {
                Some(a) => found.push(a),
                None => ()
            }
        }

        found
    }

    pub fn add_anime_to_tournament(&self, t: &Tournament, a: &Anime) {
        use schema::tournament_anime;

        let new_tournament_anime = TournamentAnime {
            tournament: t.id,
            anime: a.id
        };
        diesel::insert_into(tournament_anime::table)
            .values(&new_tournament_anime)
            .execute(&self.connection)
            .expect("Error adding anime to tournament");
    }

    pub fn get_tournament_decisions(&self, t: &Tournament) -> Vec<Decision> {
        use schema::decisions::dsl::*;

        decisions.filter(tournament.eq(t.id))
            .load::<Decision>(&self.connection)
            .expect("Error loading decisions")
    }

    fn tournament_remaining_anime(&self, t: &Tournament) -> Vec<i32> {
        // list of anime IDs
        let tournament_anime = self.get_tournament_anime(t);
        let decisions: Vec<Decision> = self.get_tournament_decisions(t);
        let mut remaining: Vec<i32> = tournament_anime.iter().map(|anime| anime.id).collect();

        // exclude anime based on picks
        let mut remove: Vec<i32> = vec![];
        for decision in &decisions {
            match Pick::to_pick(decision.pick) {
                Pick::Left => remove.push(decision.right_anime),
                Pick::Right => remove.push(decision.left_anime),
                Pick::Undecided => ()
            };
        }

        remaining.retain(|item| !remove.contains(item));
        remaining
    }
    
    // find the anime with the fewest picks
    fn next_lowest_pick(&self, t: &Tournament, anime: Vec<i32>) -> Option<i32> {
        let picks: Vec<i32> = self.get_tournament_decisions(t).iter().flat_map(|decision| {
            match Pick::to_pick(decision.pick) {
                Pick::Left => Some(decision.left_anime),
                Pick::Right => Some(decision.right_anime),
                Pick::Undecided => None
            }
        }).collect();

        let mut num_decisions: HashMap<i32, i32> = HashMap::new();
        for anime in &anime {
            num_decisions.insert(*anime, 0);
        }

        for pick in picks {
            let num = match num_decisions.get(&pick) {
                Some(num) => num + 1,
                None => 0
            };

            if num > 0 {
                num_decisions.insert(pick, num);
            }
        }

        let lowest_pick_num: i32 = {
            let mut lowest = 1000;
            for pick_num in num_decisions.values() {
                if *pick_num < lowest {
                    lowest = *pick_num;   
                }
            }
            lowest
        };

        let lowest_pick_anime: Vec<i32> = anime.iter().filter(|a| num_decisions[*a] == lowest_pick_num).map(|a| *a).collect();
        
        // None
        let mut rng = thread_rng();
        let pick = lowest_pick_anime.choose(&mut rng).map(|a| *a);
        pick
    }

    pub fn next_tournament_decision(&self, t: &Tournament) -> Option<Decision> {
        let mut anime = self.tournament_remaining_anime(t);
        if anime.len() < 2 {
            return None
        }

        let left = self.next_lowest_pick(t, anime.clone())?;
        anime.retain(|a| *a != left);
        let right = self.next_lowest_pick(t, anime)?;

        Some(Decision{
            tournament: t.id,
            left_anime: left,
            right_anime: right,
            pick: Pick::to_opt(Pick::Undecided)
        })
    }
    
    pub fn save_tournament_decision(&self, t: &Tournament, new_decision: Decision) {
        use schema::decisions;

        // let new_decision = Decision {
        //     tournament: t.id,
        //     anime: a.id
        // };
        diesel::insert_into(decisions::table)
            .values(&new_decision)
            .execute(&self.connection)
            .expect("Error adding decision to tournament");
    }

    pub fn get_tournament_winner(&self, t: &Tournament) -> Option<i32> {
        let anime: Vec<i32> = self.tournament_remaining_anime(t);
        if anime.len() > 1 {
            return None;
        }

        anime.first().map(|a| *a)
    }
}





/*
impl Tournament {


    
    pub fn remaining_anime(&self) -> Vec<i32> {
        // list of anime IDs
        let mut remaining: Vec<i32> = self.items.iter().map(|anime| anime.id).collect();

        // exclude anime based on picks
        let mut remove: Vec<i32> = vec![];
        for decision in &self.decisions {
            match decision.pick {
                Pick::Left => remove.push(decision.right),
                Pick::Right => remove.push(decision.left),
                Pick::Undecided => ()
            };
        }

        // println!(" * Remaining anime: {:?} - {:?}", remaining, remove);
        remaining.retain(|item| !remove.contains(item));
        remaining
    }

    pub fn is_finished(&self) -> bool {
        let rem = self.remaining_anime();
        // println!("[is finished] Remaining anime: {:?}", rem);
        rem.len() <= 1
    }

    // find the anime with the fewest picks
    fn next_lowest_pick(&self, anime: Vec<i32>) -> Option<i32> {
        let picks: Vec<i32> = self.decisions.iter().flat_map(|decision| {
            match decision.pick {
                Pick::Left => Some(decision.left),
                Pick::Right => Some(decision.right),
                Pick::Undecided => None
            }
        }).collect();

        let mut num_decisions: HashMap<i32, i32> = HashMap::new();
        for anime in &anime {
            num_decisions.insert(*anime, 0);
        }

        for pick in picks {
            let num = match num_decisions.get(&pick) {
                Some(num) => num + 1,
                None => 0
            };

            if num > 0 {
                num_decisions.insert(pick, num);
            }
        }

        let lowest_pick_num: i32 = {
            let mut lowest = 1000;
            for pick_num in num_decisions.values() {
                if *pick_num < lowest {
                    lowest = *pick_num;   
                }
            }
            lowest
        };

        let lowest_pick_anime: Vec<i32> = anime.iter().filter(|a| num_decisions[*a] == lowest_pick_num).map(|a| *a).collect();
        
        // None
        let mut rng = thread_rng();
        let pick = lowest_pick_anime.choose(&mut rng).map(|a| *a);
        pick
    }

    pub fn next_decision(&self) -> Option<Decision> {
        let mut anime = self.remaining_anime();
        if anime.len() < 2 {
            return None
        }

        let left = self.next_lowest_pick(anime.clone())?;
        anime.retain(|a| *a != left);
        let right = self.next_lowest_pick(anime)?;

        Some(Decision{
            left: left,
            right: right,
            pick: Pick::Undecided
        })
    }

    pub fn add_decision(&mut self, decision: Decision) {
        self.decisions.push(decision);
    }

    pub fn get_winner(&self) -> Option<i32> {
        let anime: Vec<i32> = self.remaining_anime();
        if anime.len() > 1 {
            return None;
        }

        anime.first().map(|a| *a)
    }
}

*/
