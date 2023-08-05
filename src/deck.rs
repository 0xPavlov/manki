
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

struct Deck {
    cards: Vec<Card>,
}
