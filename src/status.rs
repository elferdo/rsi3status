use crate::status_item::StatusItem;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub struct Status {
    items: Vec<StatusItem>,
}

impl Status {
    pub fn push(&mut self, item: StatusItem) {
        self.items.push(item)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status { items: vec![] }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[")?;

        write!(f, "{}", self.items.iter().join(","))?;

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

        assert_eq!(
            status.to_string(),
            "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]"
        );
    }

    #[test]
    fn default_status_when_two_items_then_to_string_equals_list_of_two_items() {
        let mut status = Status::default();
        let status_item = StatusItem::default();

        status.push(status_item.clone());
        status.push(status_item);

        assert_eq!(status.to_string(), "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"},{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]");
    }
}
