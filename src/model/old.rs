use std::collections::HashMap;

use rand::{thread_rng};
use rand::seq::SliceRandom;


pub enum Slot {
    First,
    Second,
    Third
}

impl Slot {
    pub fn name(&self) -> &str {
        match self {
            Slot::First => "first",
            Slot::Second => "second",
            Slot::Third => "third",
        }
    }
}

#[derive(Clone)]
pub struct Anime {
    pub id: i32,
    pub name: String,
    pub episodes: i32,
    pub slot1: bool,
    pub slot2: bool,
    pub slot3: bool,
}

impl Anime {
    pub fn matches_slot(&self, slot: &Slot) -> bool {
        match slot {
            Slot::First => self.slot1,
            Slot::Second => self.slot2,
            Slot::Third => self.slot3
        }
    }
}

// pub struct AnimeTag {
//     pub tag: String
// }

pub struct Tournament {
    pub slot: Slot,
    pub items: Vec<Anime>,
    pub decisions: Vec<Decision>
}

#[derive(Debug)]
pub enum Pick {
    Left,
    Right,
    Undecided
}

impl Pick {
    pub fn name(&self) -> &str {
        match self {
            Pick::Left => "left",
            Pick::Right => "right",
            Pick::Undecided => "undecided"
        }
    }
}

#[derive(Debug)]
pub struct Decision {
    pub left: i32,
    pub right: i32,
    pub pick: Pick,
}

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

pub trait TournamentModel {
    fn add_anime(&mut self, anime: Anime);
    
    fn get_anime(&self) -> Vec<Anime>;

    fn get_anime_by_id(&self, id: i32) -> Option<Anime>;

    fn get_anime_for_slot(&self, slot: Slot) -> Vec<Anime>;
}