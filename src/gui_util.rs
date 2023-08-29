use egui::{Image, Label, Widget};
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

impl WidgetWrapper {
    pub(crate) fn unwrap(&self) -> &dyn Widget {
        match self {
            WidgetWrapper::Label(label) => label,
            WidgetWrapper::Image(imageWrapper) => &imageWrapper.image,
        }
    }
}
