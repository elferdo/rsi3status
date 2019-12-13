mod battery;
mod colorscale;
mod gauge;
mod status;
mod time;

use battery::battery_status;
use status::{Status, StatusItem};
use std::time::Duration;
use time::{date_status, time_status};


fn preface() -> String {
    "{\"version\":1}\n[".to_string()
}


fn ferdo_status() -> StatusItem {
    let mut time_item = StatusItem::default();

    time_item.name = "Ferdo".to_string();
    time_item.full_text = "Ferdo".to_string();

    time_item
}


fn status() -> Status {
    let mut status = Status::default();

    status.push(ferdo_status());
    status.push(battery_status());
    status.push(date_status());
    status.push(time_status());

    status
}


fn main() {

    println!("{}", preface());

    println!("{}", status());

    loop {
        std::thread::sleep(Duration::from_secs(1u64));
    
        println!(",{}", status());
    }
}

