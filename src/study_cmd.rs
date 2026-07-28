use std::process::exit;

use crate::{
    models::{Card, Deck},
    scheduler::{is_due, promote, reset},
    storage::{list_deck_names, load_deck, load_xp, save_deck, save_xp},
    tui,
};

pub fn study(deck: Option<String>) {
    let mut decks: Vec<Deck> = Vec::new();
    let mut xp = load_xp();

    match deck {
        Some(dx) => {
            let (d, _) = load_deck(dx + ".json");
            match d {
                Some(d) => decks.push(d),
                None => {
                    println!("No such deck");
                    exit(1);
                }
            }
        }
        None => {
            for dx in list_deck_names() {
                let (d, _) = load_deck(dx.clone());
                match d {
                    Some(d) => decks.push(d),
                    None => {
                        println!("Failed to load deck {}", dx.clone());
                        continue;
                    }
                }
            }
        }
    }

    let mut msg = "".to_string();
    let mut answered = 0;
    let mut incorrect: Vec<String> = Vec::new();

    let mut due: Vec<(usize, Card)> = Vec::new();
    for mut d in decks.clone() {
        due = d
            .cards
            .iter()
            .enumerate()
            .filter(|(_, c)| is_due((**c).clone()))
            .map(|(i, c)| (i, c.clone()))
            .collect();

        for (i, c) in due.iter().enumerate() {
            match tui::run(
                d.name.clone(),
                c.1.front.clone(),
                c.1.back.clone(),
                (i as u32) + 1,
                d.cards.len() as u32,
                msg.clone(),
            ) {
                true => {
                    promote(&mut d.cards[c.0]);
                    save_deck(&d);

                    msg = "".to_string();
                    answered += 1;

                    if answered % 5 == 0 {
                        xp += 20;
                        save_xp(xp);
                        msg = format!(" +20 XP! Studied {answered} cards");
                    }
                }
                false => {
                    reset(&mut d.cards[c.0]);
                    save_deck(&d);
                    if answered % 5 == 0 {
                        xp -= 5;
                        save_xp(xp);
                    }
                    msg = "󱕤 -5 XP for incorrect answer".to_string();
                    incorrect.push(c.1.front.clone() + " -> " + &c.1.back.clone());
                }
            }
        }
    }

    if !due.is_empty() {
        xp += 100;
        save_xp(xp);
        println!(" +100 XP! Finished all due cards");
        if !incorrect.is_empty() {
            println!("Incorrectly-answered cards:");
            for x in incorrect {
                println!("- {x}");
            }
        }
        exit(0);
    } else {
        println!("No due cards.");
        exit(0);
    }
}
