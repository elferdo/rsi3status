mod colorscale;
mod gauge;
mod status;
mod time;

use gauge::bar;
use sysctl::Sysctl;
use status::{Status, StatusItem};
use std::time::Duration;
use time::{date_status, time_status};

const BATTERY:char = '🔋';

fn preface() -> String {
    "{\"version\":1}\n[".to_string()
}


fn ferdo_status() -> StatusItem {
    let mut time_item = StatusItem::default();

    time_item.name = "Ferdo".to_string();
    time_item.full_text = "Ferdo".to_string();

    time_item
}

fn battery_status() -> StatusItem {
    let mut battery_item = StatusItem::default();
    battery_item.markup = "pango".to_string();

    let life_ctl = sysctl::Ctl::new("hw.acpi.battery.life").unwrap();
    let time_ctl = sysctl::Ctl::new("hw.acpi.battery.time").unwrap();

    let mut time = 0;

    if let sysctl::CtlValue::Int(val) = time_ctl.value().unwrap() {
	time = val;
    }
    
    battery_item.name = "Battery".to_string();
    let value_string = life_ctl.value_string().unwrap();
    let value = value_string.parse::<u8>().unwrap();

    battery_item.full_text = format!("{}{}% <span foreground=\\\"#00de55\\\" background=\\\"#555555\\\">{}</span> {}", BATTERY, value_string, bar(value, 25).unwrap(), minutes_to_human(time));

    battery_item
}

fn status() -> Status {
    let mut status = Status::default();

    status.push(ferdo_status());
    status.push(battery_status());
    status.push(date_status());
    status.push(time_status());

    status
}

fn minutes_to_human(min: i32) -> String {
    let hours = min / 60;
    let remainder = min - hours * 60;
    
    format!("{:02}h {:02}m", hours, remainder)
}

fn main() {

    println!("{}", preface());

    println!("{}", status());

    loop {
        std::thread::sleep(Duration::from_secs(1u64));
    
        println!(",{}", status());
    }
}

#[cfg(test)]
mod test {
    use super::{Status, StatusItem, minutes_to_human};
    

    #[test]

    fn xxx() {
	assert_eq!(minutes_to_human(60), "01h 00m");
    }
}
