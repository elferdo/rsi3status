#[cfg(feature = "battery")]
mod battery;
mod colorscale;
mod cpu;
mod gauge;
mod status;
mod time;

use crate::status::{Status, StatusItem, StatusProvider};
use cpu::cpu_status_provider;
use std::time::Duration;
use time::{date_status_provider, time_status_provider};

fn preface() -> String {
    "{\"version\":1}\n[".to_string()
}

struct StaticStringProvider {
    static_string_status_item: StatusItem,
}

impl StatusProvider for StaticStringProvider {
    fn update(&mut self) {}

    fn provide_status_item(&self) -> &StatusItem {
        &self.static_string_status_item
    }
}

fn ferdo_status_provider() -> impl StatusProvider {
    let mut status_item = StatusItem::default();

    status_item.full_text = "Ferdo".to_owned();

    StaticStringProvider {
        static_string_status_item: status_item,
    }
}

fn build_status() -> Status {
    let mut status = Status::default();

    status.push(Box::new(ferdo_status_provider()));
    #[cfg(feature = "battery")]
    status.push(battery_status());
    status.push(Box::new(cpu_status_provider()));
    status.push(Box::new(date_status_provider()));
    status.push(Box::new(time_status_provider()));

    status
}

fn main() {
    println!("{}", preface());

    let mut status = build_status();

    println!("{}", status);

    loop {
        std::thread::sleep(Duration::from_secs(1));

        status.update();
        println!(",{}", status);
    }
}
