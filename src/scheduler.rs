use crate::models::Card;
use chrono::{DateTime, Days, FixedOffset, Utc};

fn interval(state: u16) -> u16 {
    match state {
        1 => 1,
        2 => 3,
        3 => 7,
        _ => 14,
    }
}

pub fn is_due(card: Card) -> bool {
    match card.last_reviewed {
        Some(last_reviewed) => {
            let next = last_reviewed.checked_add_days(Days::new(interval(card.state).into()));
            let now: DateTime<FixedOffset> = Utc::now().into();

            next <= Some(now)
        }
        None => true,
    }
}

pub fn promote(card: &mut Card) {
    if card.state < 4 {
        card.state += 1
    }
    let now: DateTime<FixedOffset> = Utc::now().into();
    card.last_reviewed = Some(now);
}

pub fn reset(card: &mut Card) {
    card.state = 1;
    let now: DateTime<FixedOffset> = Utc::now().into();
    card.last_reviewed = Some(now);
}
