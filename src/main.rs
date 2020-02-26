mod battery;
mod colorscale;
mod cpu;
mod gauge;
mod status;
mod status_item;
mod time;

use cpu::cpu_status_provider;
use status::{Status, StatusProvider};
use status_item::StatusItem;
use std::time::Duration;
use time::{date_status_provider, time_status_provider};

fn preface() -> String {
    "{\"version\":1}\n[".to_string()
}

struct StaticStringProvider {
    string: String,
}

impl StatusProvider for StaticStringProvider {
    fn provide_status_item(&self) -> StatusItem {
        let mut item = StatusItem::default();

        item.full_text = self.string.clone();

        item
    }
}

fn ferdo_status_provider() -> impl StatusProvider {
    StaticStringProvider {
        string: "Ferdo".to_owned(),
    }
}

fn build_status() -> Status {
    let mut status = Status::default();

    status.push(Box::new(ferdo_status_provider()));
    // status.push(battery_status());
    status.push(Box::new(cpu_status_provider()));
    status.push(Box::new(date_status_provider()));
    status.push(Box::new(time_status_provider()));

    status
}

fn main() {
    println!("{}", preface());

    let status = build_status();

    println!("{}", status);

    loop {
        std::thread::sleep(Duration::from_secs(1));

        println!(",{}", status);
    }
}
