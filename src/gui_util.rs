use serde::{Deserialize, Serialize};

// All the Widgets that are usable in Manki
#[derive(Serialize, Deserialize, Clone)]
pub enum WidgetWrapper {
    Label(String),
    Image(String),
    Latex(String),
}
