use crate::storage::list_archived_deck_names;

pub fn list() {
    let names = list_archived_deck_names();

    if names.is_empty() {
        println!("No decks found.")
    }

    for name in names {
        println!("{}", name.trim_end_matches(".json"))
    }
}
