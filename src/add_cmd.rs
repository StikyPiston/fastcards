use crate::{
    models::Card,
    storage::{load_deck, save_deck},
};
use uuid::Uuid;

pub fn add(deck: String, front: String, back: String) {
    let mut d = load_deck(deck);
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
