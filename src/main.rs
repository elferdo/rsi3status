mod gauge;

use std::fmt::{Display, Formatter};
use std::time::{Duration};
use chrono::prelude::*;
use gauge::bar;
use sysctl::Sysctl;
use itertools::Itertools;

const BATTERY:char = '🔋';
const CLOCK:char = '🕒';

struct Status {
    items: Vec<StatusItem>
}

impl Status {
    pub fn push(&mut self, item: StatusItem) {
        self.items.push(item)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status{items: vec![]}
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[")?;

	write!(f, "{}", self.items.iter().join(","))?;
	       
        write!(f, "]")
    }
}

#[derive(Clone)]
struct StatusItem{name: String,
                              instance: String,
                              markup: String,
                              full_text: String}

impl Default for StatusItem {
    fn default() -> StatusItem {
        StatusItem {
            name: "".to_string(),
            instance: "".to_string(),
            markup: "none".to_string(),
            full_text: "".to_string()
        }
    }
}

impl Display for StatusItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{{\"name\":\"{}\",\"instance\":\"{}\",\"markup\":\"{}\",\"full_text\":\"{}\"}}",
               self.name,
               self.instance,
               self.markup,
               self.full_text)
    }
}

fn preface() -> String {
    "{\"version\":1}\n[".to_string()
}

fn time_status() -> StatusItem {
    let mut time_item = StatusItem::default();

    time_item.name = "Time".to_string();
    time_item.full_text = format!("{}{}", CLOCK, Local::now());

    time_item
}

fn ferdo_status() -> StatusItem {
    let mut time_item = StatusItem::default();

    time_item.name = "Ferdo".to_string();
    time_item.full_text = "Ferdo".to_string();

    time_item
}

fn battery_status() -> StatusItem {
    let mut battery_item = StatusItem::default();

    let life_ctl = sysctl::Ctl::new("hw.acpi.battery.life").unwrap();
    let time_ctl = sysctl::Ctl::new("hw.acpi.battery.time").unwrap();

    let mut time = 0;

    if let sysctl::CtlValue::Int(val) = time_ctl.value().unwrap() {
	time = val;
    }
    
    battery_item.name = "Battery".to_string();
    let value_string = life_ctl.value_string().unwrap();
    let value = value_string.parse::<u8>().unwrap();
    
    battery_item.full_text = format!("{}{}% {} {}", BATTERY, value_string, bar(value, 5).unwrap(), minutes_to_human(time));

    battery_item
}

fn status() -> Status {
    let mut status = Status::default();

    status.push(ferdo_status());
    status.push(battery_status());
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
    fn default_status_item_when_to_string_then_all_fields_empty() {
        let status_item = StatusItem::default();
        
        assert_eq!(status_item.to_string(), "{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}");
    }

    #[test]
    fn default_status_when_default_then_empty_list() {
        let status = Status::default();

        assert_eq!(status.to_string(), "[]");
    }

    #[test]
    fn default_status_when_one_item_then_to_string_equals_list_of_one_item() {
        let mut status = Status::default();
        let status_item = StatusItem::default();

        status.push(status_item);
        
        assert_eq!(status.to_string(), "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]");
    }

    #[test]
    fn default_status_when_two_items_then_to_string_equals_list_of_two_items() {
        let mut status = Status::default();
        let status_item = StatusItem::default();

        status.push(status_item.clone());
        status.push(status_item);
        
        assert_eq!(status.to_string(), "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"},{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]");
    }

    #[test]

    fn xxx() {
	assert_eq!(minutes_to_human(60), "01h 00m");
    }
}
