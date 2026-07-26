use crate::{
    models::{Card, Deck},
    storage::{load_deck, save_deck},
};
use std::process::exit;
use uuid::Uuid;

pub fn add(deck: String, front: String, back: String) {
    let deck = deck + ".json";
    let (dx, err) = load_deck(deck.clone());
    let mut d: Deck;
    if err {
        println!("Deck {deck} not found");
        exit(1);
    }
    match dx {
        Some(dx) => d = dx,
        None => {
            println!("Deck {deck} not found");
            exit(1);
        }
    }

    let card = Card {
        id: Uuid::new_v4(),
        front: front,
        back: back,
        state: 1,
        last_reviewed: None,
    };

    d.cards.push(card);

    match save_deck(&d) {
        true => println!("Card added"),
        false => println!("Failed to save deck"),
    }
}
