use super::StatusItem;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub trait StatusProvider {
    fn provide_status_item(&self) -> StatusItem;
}

pub struct Status {
    providers: Vec<Box<dyn StatusProvider>>,
}

impl Status {
    pub fn push(&mut self, provider: Box<dyn StatusProvider>) {
        self.providers.push(provider)
    }
}

impl Default for Status {
    fn default() -> Status {
        Status { providers: vec![] }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[")?;

        write!(
            f,
            "{}",
            self.providers
                .iter()
                .map(|p| p.provide_status_item())
                .join(",")
        )?;

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone)]
    struct MockProvider {}

    impl StatusProvider for MockProvider {
        fn provide_status_item(&self) -> StatusItem {
            StatusItem::default()
        }
    }

    #[test]
    fn default_status_when_default_then_empty_list() {
        let status = Status::default();

        assert_eq!(status.to_string(), "[]");
    }

    #[test]
    fn default_status_when_one_item_then_to_string_equals_list_of_one_item() {
        let mut status = Status::default();
        let status_item = MockProvider {};

        status.push(Box::new(status_item));

        assert_eq!(
            status.to_string(),
            "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]"
        );
    }

    #[test]
    fn default_status_when_two_items_then_to_string_equals_list_of_two_items() {
        let mut status = Status::default();
        let status_item = MockProvider {};

        status.push(Box::new(status_item.clone()));
        status.push(Box::new(status_item));

        assert_eq!(status.to_string(), "[{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"},{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}]");
    }
}
