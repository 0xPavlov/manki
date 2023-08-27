use std::any::Any;

use chrono::NaiveDateTime;
use egui::{Button, Image, Label, RichText, TextEdit};
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serializer};

use crate::gui_util::WidgetWrapper;

pub(crate) fn serialize_naive_datetime<S>(
    datetime: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    return serializer.serialize_str(&formatted_datetime);
}

pub(crate) fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_str = String::deserialize(deserializer)?;
    return NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom);
}

pub(crate) fn __serialize_widgets<S>(
    widgets: &Vec<Box<dyn Any>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(widgets.len()))?;

    for widget in widgets {
        if let Some(label) = widget.downcast_ref::<Label>() {
            map.serialize_entry("Label", label.text())?;
        } else if let Some(image) = widget.downcast_ref::<Image>() {
            unimplemented!()
        } else if let Some(code_block) = widget.downcast_ref::<RichText>() {
            map.serialize_entry("Code Block", code_block.text())?;
        } else {
            unimplemented!();
        }
    }
    map.end()
}

pub(crate) fn serialize_widgets<S>(
    widgets: &Vec<WidgetWrapper>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(widgets.len()))?;

    for widget in widgets {
        match widget {
            WidgetWrapper::Button(button) => {}
            WidgetWrapper::Label(label) => {}
            WidgetWrapper::Image(image) => {}
        }
    }

    map.end()
}
