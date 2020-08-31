use crate::model::*;

pub struct MockModel {
    anime: Vec<Anime>
}

impl MockModel {
    pub fn new() -> MockModel {
        MockModel{
            anime: vec![]
        }
    }

    pub fn add_mock_anime(&mut self) {
        let items = [
            Anime {
                id: 1,
                name: String::from("Aldnoah.Zero"),
                episodes: 24,
                slot: [true, false, false],
            },
            Anime {
                id: 2,
                name: String::from("Bloom Into You"),
                episodes: 13,
                slot: [true, true, false],
            },
            Anime {
                id: 3,
                name: String::from("Fairy Gone"),
                episodes: 12,
                slot: [true, false, false],
            },
            Anime {
                id: 4,
                name: String::from("Iroduku: The World In Colors"),
                episodes: 13,
                slot: [true, false, false],
            },
            Anime {
                id: 5,
                name: String::from("Bounen no Xamdou"),
                episodes: 26,
                slot: [true, true, false],
            },
            Anime {
                id: 6,
                name: String::from("Kabaneri of the Iron Fortress"),
                episodes: 12,
                slot: [true, false, false],
            },
            Anime {
                id: 7,
                name: String::from("Plastic Memories"),
                episodes: 13,
                slot: [true, false, false],
            },
            Anime {
                id: 8,
                name: String::from("Kaguya S2"),
                episodes: 12,
                slot: [false, false, true],
            }
        ];

        for item in &items {
            self.add_anime(item.clone());
        }
    }
}

impl TournamentModel for MockModel {
    fn add_anime(&mut self, anime: Anime) {
        self.anime.push(anime);
    }

    fn get_anime(&self) -> Vec<Anime> {
        self.anime.clone()
    }

    fn get_anime_by_id(&self, id: i32) -> Option<Anime> {
        self.anime.iter().filter(|anime| anime.id == id).next().map(|anime| anime.clone())
    }

    fn get_anime_for_slot(&self, slot: Slot) -> Vec<Anime> {
        self.anime.iter().filter(|anime| anime.matches_slot(&slot)).cloned().collect()
    }
}