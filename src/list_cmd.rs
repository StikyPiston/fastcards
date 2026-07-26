use std::process::exit;

use crate::{
    models::Deck,
    scheduler::is_due,
    storage::{list_deck_names, load_deck},
};

pub fn list() {
    let names = list_deck_names();

    if names.is_empty() {
        println!("No decks found.")
    }

    for name in names {
        let (d, err) = load_deck(name.clone());

        if err {
            println!("Failed to load decks");
            exit(1);
        }
        let deck: Deck = match d {
            Some(d) => d,
            None => {
                println!("Failed to load decks");
                exit(1);
            }
        };

        let total = deck.cards.len();

        let mut due = 0;

        for c in deck.cards {
            if is_due(c) {
                due += 1;
            }
        }

        println!(
            "{}: {total} cards ({due} due)",
            name.trim_end_matches(".json")
        )
    }
}
