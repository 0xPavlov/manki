use std::any::Any;

use chrono::NaiveDateTime;
use egui::{Image, Label, TextEdit};
use serde::{Deserialize, Deserializer, Serializer};

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

pub(crate) fn serialize_widgets<S>(
    widgets: &Vec<Box<dyn Any>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer<Ok = ()>,
{
    for widget in widgets {
        if let Some(label) = widget.downcast_ref::<Label>() {
        } else if let Some(image) = widget.downcast_ref::<Image>() {
        } else if let Some(code_block) = widget.downcast_ref::<TextEdit>() {
        } else {
            unimplemented!();
        }
    }
    Ok(())
}
