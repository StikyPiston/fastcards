use std::{
    fs::{create_dir_all, read_dir, read_to_string, write},
    path::PathBuf,
};

use uuid::Uuid;

use crate::models::{Deck, Xp};

fn base_dir() -> PathBuf {
    let h = dirs::home_dir();
    let home = h
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();

    PathBuf::from(home).join(".fastcards")
}

pub fn decks_dir() -> PathBuf {
    base_dir().join("decks")
}

pub fn archive_dir() -> PathBuf {
    base_dir().join("archive")
}

pub fn data_dir() -> PathBuf {
    base_dir().join("data")
}

pub fn ensure_dirs() {
    create_dir_all(decks_dir()).unwrap();
    create_dir_all(archive_dir()).unwrap();
    create_dir_all(data_dir()).unwrap();
}

pub fn deck_path(name: String) -> PathBuf {
    decks_dir().join(name)
}

fn xp_path() -> PathBuf {
    data_dir().join("xp.json")
}

pub fn save_deck(deck: &Deck) -> bool {
    ensure_dirs();

    match serde_json::to_string_pretty(deck) {
        Ok(json) => write(deck_path(deck.name.clone() + ".json"), json).is_ok(),
        Err(_) => false,
    }
}

pub fn load_deck(name: String) -> (Option<Deck>, bool) {
    let deck: Option<Deck> = read_to_string(deck_path(name))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok());

    match deck {
        Some(deck) => (Some(deck), false),
        None => (None, true),
    }
}

pub fn list_deck_names() -> Vec<String> {
    let dir = decks_dir();
    let entries = read_dir(dir).unwrap();

    let mut names: Vec<String> = Vec::new();

    for e in entries {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            continue;
        }

        let name = e.file_name().into_string().unwrap();
        names.push(name)
    }

    names
}

pub fn list_archived_deck_names() -> Vec<String> {
    let dir = archive_dir();
    let entries = read_dir(dir).unwrap();

    let mut names: Vec<String> = Vec::new();

    for e in entries {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            continue;
        }

        let name = e.file_name().into_string().unwrap();
        names.push(name)
    }

    names
}

pub fn create_deck(name: String) -> String {
    let path = deck_path(name.clone() + ".json");

    if !path.exists() {
        let deck = Deck {
            id: Uuid::new_v4(),
            name: name.clone(),
            cards: Vec::new(),
        };

        save_deck(&deck);
    }

    path.to_str().unwrap().to_string()
}

// MARK: xp stuff
pub fn save_xp(xp: u32) {
    ensure_dirs();

    let x = Xp { xp };

    match serde_json::to_string_pretty(&x) {
        Ok(json) => write(xp_path(), json).unwrap(),
        Err(_) => panic!("Failed to save XP"),
    }
}

pub fn load_xp() -> u32 {
    let xp: Xp = read_to_string(xp_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap();

    xp.xp
}
