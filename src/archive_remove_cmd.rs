use crate::storage::{archive_dir, deck_path};
use std::{fs::rename, process::exit};

pub fn remove(name: String) {
    let old_path = archive_dir().join(name.clone() + ".json");
    let new_path = deck_path(name.clone() + ".json");

    match rename(old_path, new_path) {
        Ok(_) => println!("Unarchived deck: {}", name.clone()),
        Err(_) => {
            println!("Failed to unarchive deck");
            exit(1);
        }
    }
}
