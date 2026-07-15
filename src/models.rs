use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Card {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub state: u16,
    pub last_reviewed: Option<DateTime<FixedOffset>>,
}

#[derive(Deserialize, Serialize)]
pub struct Deck {
    pub id: Uuid,
    pub name: String,
    pub cards: Vec<Card>,
}

#[derive(Deserialize, Serialize)]
pub struct Xp {
    pub xp: u32,
}
