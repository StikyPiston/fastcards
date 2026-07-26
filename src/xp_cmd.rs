use crate::storage::load_xp;

pub fn xp() {
    let xp = load_xp();

    println!(" XP: {xp}")
}
