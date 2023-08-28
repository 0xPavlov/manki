use egui::{Image, Label};
use std::path::PathBuf;

pub struct ImageWrapper {
    pub path: PathBuf,
    pub image: Image,
}

// All the Widgets that are usable in Manki
pub enum WidgetWrapper {
    Label(Label),
    Image(ImageWrapper),
}
