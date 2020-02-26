use crate::status::StatusProvider;
use crate::status_item::StatusItem;
use chrono::prelude::*;

const CLOCK: char = '🕒';
const CALENDAR: char = '📅';

struct DateStatusProvider {}
struct TimeStatusProvider {}

impl StatusProvider for DateStatusProvider {
    fn provide_status_item(&self) -> StatusItem {
        let mut time_item = StatusItem::default();

        time_item.name = "Time".to_string();
        time_item.full_text = format!("{}{}", CALENDAR, Local::now().format("%d %h %Y"));

        time_item
    }
}

impl StatusProvider for TimeStatusProvider {
    fn provide_status_item(&self) -> StatusItem {
        let mut time_item = StatusItem::default();

        time_item.name = "Time".to_string();
        time_item.full_text = format!("{}{}", CLOCK, Local::now().format("%H:%M:%S"));

        time_item
    }
}

pub fn date_status_provider() -> impl StatusProvider {
    DateStatusProvider {}
}

pub fn time_status_provider() -> impl StatusProvider {
    TimeStatusProvider {}
}
