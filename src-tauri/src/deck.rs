use crate::{card::Card, prelude::Result};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
}

pub fn read_decks_from_file(file_path: &str) -> Result<Vec<Deck>> {
    let file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            let file = std::fs::File::create(file_path).map_err(|e| e.to_string())?;
            serde_json::to_writer(file, &Vec::<Deck>::new()).map_err(|e| e.to_string())?;
            return Ok(Vec::new());
        }
    };
    let reader = std::io::BufReader::new(file);
    let decks = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(decks)
}
