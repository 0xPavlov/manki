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
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

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
struct Card {
    index: usize,
    front: String,
    back: String,

    // Last Evaluation to determine the sorting for the next learning session
    last_eval: Evaluation,
}
         
#[derive(Serialize, Deserialize)]
pub(crate) struct Deck {
    title: String,
    category: String, 

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

    pub(crate) fn save_to_json(&mut self, path: PathBuf) -> Result<(), String> {
        let file_path = format!("{}/{}.json", path.to_str().unwrap(), self.title);
        let serialized_deck =  match serde_json::to_string(self) {
            Ok(string) => string,
            Err(err) => return Err(format!("Could not serialize Deck due to {}", err)), 
        };

        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(err) => return Err(format!("Could not create file due to {}", err)),
        };

        match file.write_all(serialized_deck.as_bytes()) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(format!("Could not write to file due to {}", err)),
        }
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
