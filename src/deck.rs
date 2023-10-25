use crate::gui_util::WidgetWrapper;
use crate::io_manager::write_string_to_file;
use crate::serde_util::{deserialize_naive_datetime, serialize_naive_datetime};
use chrono::{Local, NaiveDateTime};
use egui::RichText;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Evaluation {
    #[serde(rename = "Easy")]
    Easy = 3,
    #[serde(rename = "Hard")]
    Hard = 2,
    #[serde(rename = "Again")]
    Again = 1,
}

impl ToString for Evaluation {
    fn to_string(&self) -> String {
        match &self {
            Self::Easy => String::from("Easy"),
            Self::Hard => String::from("Hard"),
            Self::Again => String::from("Again"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    // Headings are rendered at the top of the screen and are meant to be the field for the question
    front_heading: String,
    back_heading: String,
    front_body: Vec<WidgetWrapper>,
    back_body: Vec<WidgetWrapper>,
    flipped: bool,

    // Last Evaluation to determine the sorting for the next learning session
    last_eval: Evaluation,
}

#[allow(dead_code)]
impl Card {
    pub(crate) fn new(frt_head: String, bck_head: String) -> Self {
        Card {
            front_heading: frt_head,
            back_heading: bck_head,
            front_body: Vec::new(),
            back_body: Vec::new(),
            flipped: false,
            last_eval: Evaluation::Again,
        }
    }

    pub(crate) fn heading(&self) -> RichText {
        if self.flipped {
            return RichText::new(&self.back_heading).heading();
        }
        RichText::new(&self.front_heading).heading()
    }

    pub(crate) fn body(&self) -> &Vec<WidgetWrapper> {
        if self.flipped {
            return &self.back_body;
        }
        &self.front_body
    }

    pub(crate) fn flip(&mut self) {
        self.flipped = !self.flipped;
    }

    pub(crate) fn update_eval(&mut self, eval: Evaluation) {
        self.last_eval = eval;
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Deck {
    pub title: String,
    pub category: String,
    pub deserialize_failed: bool,

    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    last_studied: NaiveDateTime,

    pub cards: Vec<Card>,
}

#[allow(dead_code)]
impl Deck {
    pub(crate) fn empty(ttl: &str) -> Deck {
        return Deck {
            title: ttl.to_string(),
            category: "None".to_string(),
            last_studied: Local::now().naive_local(),
            cards: Vec::new(),
            deserialize_failed: false,
        };
    }

    pub(crate) fn as_unserializable(self) -> Deck {
        let mut deck = self.clone();
        deck.deserialize_failed = true;
        deck
    }

    pub(crate) fn save_to_json(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if self.deserialize_failed {
            // Ok is returned here instead of Error, to indicate the succesful capture of
            // unserealizable decks
            return Ok(());
        }

        let file_path = path.join(format!("{}.json", self.title));
        self.last_studied = Local::now().naive_local();
        write_string_to_file(file_path, serde_json::to_string(self)?)
    }

    pub(crate) fn read_from(path: &PathBuf) -> Result<Deck, Box<dyn Error>> {
        let file_contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&file_contents.as_str())?)
    }

    pub(crate) fn sort(&mut self) {
        self.cards.sort_by(|a, b| a.last_eval.cmp(&b.last_eval))
    }

    pub(crate) fn get(&mut self, index: usize) -> Option<&mut Card> {
        self.cards.get_mut(index)
    }

    pub(crate) fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}
