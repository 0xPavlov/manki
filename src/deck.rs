use chrono::prelude::*;
enum Evaluation {
    VeryGood,
    Good,
    Bad,
    VeryBad,
}

struct Card {
    index: usize,
    front: String,
    back: String,

    // Last Evaluation to determine the sorting for the next learning session
    last_eval: Evaluation,
}

pub(crate) struct Deck {
    last_studied: DateTime<Local>,
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        return Deck {
            last_studied: Local::now(),
            cards: Vec::new(),
        }
    }

    fn from(crds: Vec<Card>, timestamp: DateTime<Local>) -> Deck {
        return Deck {
            last_studied: timestamp,
            cards: crds,
        }
    }
}
