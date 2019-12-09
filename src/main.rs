use std::fmt::{Display, Formatter};
use std::time::{Duration};
use chrono::prelude::*;

struct Status<T: Display> {
    items: Vec<StatusItem<T>>
}

impl<T: Display> Status<T> {
    pub fn push(&mut self, item: StatusItem<T>) {
        self.items.push(item)
    }
}

impl<T: Display> Default for Status<T> {
    fn default() -> Status<T> {
        Status::<T>{items: vec![]}
    }
}

impl<T: Display> Display for Status<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[")?;

        for item in &self.items {
            write!(f, "{}", item)?;
        }

        write!(f, "]")
    }
}


struct StatusItem<T>{name: String,
                              instance: String,
                              markup: String,
                              full_text: T}

impl<T> StatusItem<T> {
}

impl<T: Default+Display> Default for StatusItem<T> {
    fn default() -> StatusItem<T> {
        StatusItem::<T> {
            name: "".to_string(),
            instance: "".to_string(),
            markup: "none".to_string(),
            full_text: T::default()
        }
    }
}

impl<T: Display> Display for StatusItem<T> {
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

fn status() -> Status<DateTime<Local>> {
    let mut time_item = StatusItem::default();

    time_item.name = "Time".to_string();
    time_item.full_text = Local::now();

    let mut status = Status::default();

    status.push(time_item);

    status
}

fn main() {
    println!("{}", preface());

    println!("[{}]", status());

    loop {
        std::thread::sleep(Duration::from_millis(500u64));
    
        println!(",[{}]", status());
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
