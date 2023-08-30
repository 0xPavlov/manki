use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// All the Widgets that are usable in Manki
#[derive(Serialize, Deserialize)]
pub enum WidgetWrapper {
    Label(String),
    Image(PathBuf),
}
