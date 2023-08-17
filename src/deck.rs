use chrono::{
    Local,
    NaiveDateTime,
};
use serde::{
    Serialize,
    Deserialize,
    Serializer,
    Deserializer,
};
use std::error::Error;
use std::io::{Write, Read};
use std::path::PathBuf;
use std::fs::File;

use crate::{
    file_manager,
    logger::Logger,
};

#[derive(Serialize, Deserialize)]
enum Evaluation {
    #[serde(rename = "VeryGood")]
    VeryGood,
    #[serde(rename = "Good")]
    Good,
    #[serde(rename = "Bad")]
    Bad,
    #[serde(rename = "VeryBad")]
    VeryBad,
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    //index: usize,
    front: String,
    back: String,

    // Last Evaluation to determine the sorting for the next learning session
    last_eval: Evaluation,
}

impl Card {
    pub(crate) fn from(frt: String, bck: String) -> Card {
        return Card {
            front: frt,
            back: bck,
            last_eval: Evaluation::VeryBad,
        }
    }
}
         
#[derive(Serialize, Deserialize)]
pub(crate) struct Deck {
    pub title: String,
    pub category: String, 

    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
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
        }
    }

    pub(crate) fn save_to_json(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: This file path is kinda ugly ngl
        let file_path = format!("{}/{}.json", file_manager::decks_directory().to_str().unwrap(), self.title);


        let serialized_deck =  serde_json::to_string(self)?; 
        let mut file = File::create(file_path)?;
        file.write_all(serialized_deck.as_bytes())?;
        Ok(())
    }

    pub(crate) fn read_from(path: &PathBuf, logger: &mut Logger) -> Result<Deck, Box<dyn Error>> {
        let mut file = File::open(&path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        Ok(serde_json::from_str(&file_contents.as_str())?)
    }
}

fn serialize_naive_datetime<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    return serializer.serialize_str(&formatted_datetime);
}

fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error> where D: Deserializer<'de> {
    let datetime_str = String::deserialize(deserializer)?;
    return NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom);
}
