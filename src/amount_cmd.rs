use crate::{
    models::Deck,
    scheduler::is_due,
    storage::{list_deck_names, load_deck},
};
use std::process::exit;

pub fn amount(deck: Option<String>) {
    let mut decks: Vec<Deck> = Vec::new();

    match deck {
        Some(dx) => {
            let (d, err) = load_deck(dx.clone());
            if err {
                println!("Deck {dx} not found");
                exit(1);
            }
            match d {
                Some(d) => decks.push(d),
                None => {
                    println!("Deck {dx} not found");
                    exit(1);
                }
            }
        }
        None => {
            let names = list_deck_names();
            for name in names {
                let (d, _) = load_deck(name.clone());
                if let Some(d) = d { decks.push(d) }
            }
        }
    }

    let mut total = 0;

    for d in decks {
        for c in d.cards {
            if is_due(c) {
                total += 1;
            }
        }
    }

    println!("󰘸 Flashcards due: {total}")
}
