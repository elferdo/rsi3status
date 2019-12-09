use std::fmt::{Display, Formatter};
use std::time::{Duration};
use chrono::prelude::*;

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

        for item in &self.items {
            write!(f, "{}", item)?;
        }

        write!(f, "]")
    }
}


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

fn status() -> Status {
    let mut time_item = StatusItem::default();

    time_item.name = "Time".to_string();
    time_item.full_text = Local::now().to_string();

    let mut status = Status::default();

    status.push(time_item);

    status
}

fn main() {
    println!("{}", preface());

    println!("[{}]", status());

    loop {
        std::thread::sleep(Duration::from_secs(1u64));
    
        println!(",{}", status());
    }
}

#[cfg(test)]
mod test {
    use super::{Status, StatusItem};
    use std::fmt::{Display, Formatter};
    
    struct DateTimeFake {
    }

    impl Display for DateTimeFake {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "date_time_fake")
        }
    }
    
    #[test]
    fn default_status_item_when_to_string_then_all_fields_empty() {
        let status_item = StatusItem::<String>::default();
        
        assert_eq!(status_item.to_string(), "{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}");
    }

    #[test]
    fn default_status_xxx() {
        let status_item = StatusItem::<DateTimeFake>::new();
        
        assert_eq!(status_item.to_string(), "{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"date_time_fake\"}");
    }    

    #[test]
    fn default_status_when_default_then_empty_list() {
        let status = Status::default();

        assert_eq!(status.to_string(), "[]");
    }

    #[test]
    fn default_status_when_one_item_then_to_string_equals_list_of_one_item() {
        let mut status = Status::default();
        let status_item = StatusItem::<String>::default();

        status.push(status_item);
        
        assert_eq!(status.to_string(), "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]");
    }
}
