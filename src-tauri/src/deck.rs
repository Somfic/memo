use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    card::{Answer, Card, Prompt, Repititions},
    prelude::Result,
};
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Type)]
pub struct Deck {
    pub name: String,
    pub description: String,
    pub cards: Vec<Card>,
}

#[tauri::command]
#[specta::specta]
pub fn read_decks_from_file(file_path: &str) -> Result<Vec<Deck>> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let decks = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(decks)
}

#[tauri::command]
#[specta::specta]
pub fn read_decks_from_anki_file(file_path: &str) -> Result<Vec<Deck>> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut db_file = archive
        .by_name("collection.anki2")
        .map_err(|e| e.to_string())?;

    let mut db_bytes = Vec::new();
    db_file
        .read_to_end(&mut db_bytes)
        .map_err(|e| e.to_string())?;

    let temp_path = std::env::temp_dir();
    let temp_path = temp_path.join("temp.anki2");

    std::fs::write(&temp_path, &db_bytes).map_err(|e| e.to_string())?;
    let conn = rusqlite::Connection::open(&temp_path).map_err(|e| e.to_string())?;

    conn.execute_batch("PRAGMA journal_mode = OFF;")
        .map_err(|e| e.to_string())?;
    conn.execute_batch("BEGIN;").map_err(|e| e.to_string())?;
    conn.execute_batch(
        "ATTACH DATABASE ':memory:' AS deck_db; 
        CREATE TABLE temp.main AS SELECT * FROM deck_db.sqlite_master;",
    )
    .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT n.flds
            FROM cards c
            JOIN notes n ON c.nid = n.id",
        )
        .map_err(|e| e.to_string())?;

    let mut cards: Vec<Card> = vec![];

    for result in stmt
        .query_map([], |row| {
            let flds: String = row.get(0)?;
            let parts: Vec<&str> = flds.split('\u{1f}').collect();
            let prompt = Prompt::Text(parts[0].to_string());
            let answer = Answer::Text(parts.get(1).unwrap_or(&"").to_string());
            let next_review = chrono::Utc::now();
            let repititions = Repititions {
                successful: 0,
                total: 0,
            };
            let ease_factor = 1.3;
            Ok(Card {
                prompt,
                answer,
                next_review,
                repititions,
                ease_factor,
            })
        })
        .map_err(|e| e.to_string())?
    {
        let card = result.map_err(|e| e.to_string())?;
        cards.push(card);
    }

    // randomly sort the cards to shuffle them
    use rand::seq::SliceRandom;
    let cards = {
        let mut rng = rand::rng();
        cards.as_mut_slice().shuffle(&mut rng);
        cards
    };

    Ok(vec![Deck {
        name: "main".to_string(),
        cards,
        description: "".to_string(),
    }])
}
