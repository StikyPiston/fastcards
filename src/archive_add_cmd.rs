use crate::storage::{archive_dir, deck_path};
use std::{fs::rename, process::exit};

pub fn add(name: String) {
    let old_path = deck_path(name.clone() + ".json");
    let new_path = archive_dir().join(name.clone() + ".json");

    match rename(old_path, new_path) {
        Ok(_) => println!("Archived deck: {}", name.clone()),
        Err(_) => {
            println!("Failed to archive deck");
            exit(1);
        }
    }
}
