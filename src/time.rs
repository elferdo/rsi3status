use chrono::prelude::*;
use crate::status_item::StatusItem;

const CLOCK:char = '🕒';
const CALENDAR:char = '📅';

pub fn time_status() -> StatusItem {
    let mut time_item = StatusItem::default();

    time_item.name = "Time".to_string();
    time_item.full_text = format!("{}{}", CLOCK, Local::now().format("%H:%M:%S"));

    time_item
}

pub fn date_status() -> StatusItem {
    let mut time_item = StatusItem::default();

    time_item.name = "Time".to_string();
    time_item.full_text = format!("{}{}", CALENDAR, Local::now().format("%d %h %Y"));

    time_item
}
