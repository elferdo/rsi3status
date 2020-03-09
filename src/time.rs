use crate::status::{StatusItem, StatusProvider};
use chrono::prelude::*;

const CLOCK: char = '🕒';
const CALENDAR: char = '📅';

struct DateStatusProvider {
    date_item: StatusItem,
}
struct TimeStatusProvider {
    time_item: StatusItem,
}

impl StatusProvider for DateStatusProvider {
    fn update(&mut self) {
        let mut date_item = StatusItem::default();

        date_item.name = "Time".to_string();
        date_item.full_text = format!("{}{}", CALENDAR, Local::now().format("%d %h %Y"));

        self.date_item = date_item;
    }

    fn provide_status_item(&self) -> &StatusItem {
        &self.date_item
    }
}

impl StatusProvider for TimeStatusProvider {
    fn update(&mut self) {
        let mut time_item = StatusItem::default();

        time_item.name = "Time".to_string();
        time_item.full_text = format!("{}{}", CLOCK, Local::now().format("%H:%M:%S"));

        self.time_item = time_item;
    }

    fn provide_status_item(&self) -> &StatusItem {
        &self.time_item
    }
}

pub fn date_status_provider() -> impl StatusProvider {
    DateStatusProvider {
        date_item: StatusItem::default(),
    }
}

pub fn time_status_provider() -> impl StatusProvider {
    TimeStatusProvider {
        time_item: StatusItem::default(),
    }
}
