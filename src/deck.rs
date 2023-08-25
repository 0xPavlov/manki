use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::file_manager;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Evaluation {
    #[serde(rename = "VeryGood")]
    VeryGood = 4,
    #[serde(rename = "Good")]
    Good = 3,
    #[serde(rename = "Bad")]
    Bad = 2,
    #[serde(rename = "VeryBad")]
    VeryBad = 1,
}

#[derive(Serialize, Deserialize, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct Card {
    front: String,
    back: String,
    flipped: bool,
    // Last Evaluation to determine the sorting for the next learning session
    last_eval: Evaluation,
}

impl Card {
    pub(crate) fn _new(frt: String, bck: String) -> Card {
        return Card {
            front: frt,
            back: bck,
            flipped: false,
            last_eval: Evaluation::VeryBad,
        };
    }

    pub(crate) fn display_text(&self) -> &String {
        if self.flipped {
            return &self.back;
        }
        return &self.front;
    }

    pub(crate) fn flip(&mut self) {
        self.flipped = !self.flipped;
    }

    pub(crate) fn update_eval(&mut self, eval: Evaluation) {
        self.last_eval = eval;
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Deck {
    pub title: String,
    pub category: String,

    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    last_studied: NaiveDateTime,

    cards: Vec<Card>,
}

impl Deck {
    pub(crate) fn empty(ttl: &str) -> Deck {
        return Deck {
            title: ttl.to_string(),
            category: "None".to_string(),
            last_studied: Local::now().naive_local(),
            cards: Vec::new(),
        };
    }

    pub(crate) fn save_to_json(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: This file path is kinda ugly ngl
        let file_path = format!(
            "{}/{}.json",
            file_manager::decks_directory().to_str().unwrap(),
            self.title
        );
        self.last_studied = Local::now().naive_local();
        let serialized_deck = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized_deck.as_bytes())?;
        Ok(())
    }

    pub(crate) fn read_from(path: &PathBuf) -> Result<Deck, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        Ok(serde_json::from_str(&file_contents.as_str())?)
    }

    pub(crate) fn _add_card(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub(crate) fn sort(&mut self) {
        self.cards.sort_by(|a, b| a.last_eval.cmp(&b.last_eval))
    }

    pub(crate) fn _len(&self) -> usize {
        self.cards.len()
    }

    pub(crate) fn get(&mut self, index: usize) -> Option<&mut Card> {
        self.cards.get_mut(index)
    }
}

fn serialize_naive_datetime<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    return serializer.serialize_str(&formatted_datetime);
}

fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_str = String::deserialize(deserializer)?;
    return NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom);
}
