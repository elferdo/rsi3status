use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub struct Status {
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
pub struct StatusItem{
    pub name: String,
    instance: String,
    pub markup: String,
    pub full_text: String,
    urgent: bool}

impl Default for StatusItem {
    fn default() -> StatusItem {
        StatusItem {
            name: "".to_string(),
            instance: "".to_string(),
            markup: "none".to_string(),
            full_text: "".to_string(),
	    urgent: false
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
