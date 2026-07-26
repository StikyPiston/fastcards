use std::process::exit;

use uuid::Uuid;

use crate::{models::Deck, storage::save_deck};

pub fn create(name: String) {
    let deck = Deck {
        id: Uuid::new_v4(),
        name: name.clone(),
        cards: Vec::new(),
    };

    match save_deck(&deck) {
        true => println!("Deck created: {}", name),
        false => {
            println!("Failed to create deck");
            exit(1);
        }
    }
}
