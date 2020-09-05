use crate::schema::*;

#[derive(Debug)]
pub enum Slot {
    First,
    Second,
    Third
}

#[derive(Debug)]
pub enum Pick {
    Left,
    Right,
    Undecided
}

impl Pick {
    pub fn to_pick(opt: Option<bool>) -> Pick {
        match opt {
            Some(true) => Pick::Left,
            Some(false) => Pick::Right,
            None => Pick::Undecided
        }
    }

    pub fn to_opt(pick: Pick) -> Option<bool> {
        match pick {
            Pick::Left => Some(true),
            Pick::Right => Some(false),
            Pick::Undecided => None
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            Pick::Left => "left",
            Pick::Right => "right",
            Pick::Undecided => "undecided"
        }
    }
}

#[derive(Queryable, Insertable,Debug)]
#[table_name="anime"]
pub struct Anime {
    pub id: i32,
    pub name: String,
    pub episodes: Option<i32>,
    pub slot1: bool,
    pub slot2: bool,
    pub slot3: bool,
}

#[derive(Queryable, Insertable,Debug)]
pub struct Tournament {
    pub id: i32,
}

#[derive(Queryable,Insertable,Debug)]
#[table_name="tournament_anime"]
pub struct TournamentAnime {
    pub tournament: i32,
    pub anime: i32
}

#[derive(Queryable,Insertable,Debug)]
pub struct Decision {
    pub tournament: i32,
    pub left_anime: i32,
    pub right_anime: i32,
    pub pick: Option<bool>
}
