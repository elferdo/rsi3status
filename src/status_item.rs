use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct StatusItem {
    pub name: String,
    instance: String,
    pub markup: String,
    pub full_text: String,
    urgent: bool,
}

impl Default for StatusItem {
    fn default() -> StatusItem {
        StatusItem {
            name: "".to_string(),
            instance: "".to_string(),
            markup: "none".to_string(),
            full_text: "".to_string(),
            urgent: false,
        }
    }
}

impl Display for StatusItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{{\"name\":\"{}\",\"instance\":\"{}\",\"markup\":\"{}\",\"full_text\":\"{}\"}}",
            self.name, self.instance, self.markup, self.full_text
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_status_item_when_to_string_then_all_fields_empty() {
        let status_item = StatusItem::default();

        assert_eq!(
            status_item.to_string(),
            "{\"name\":\"\",\"instance\":\"\",\"markup\":\"none\",\"full_text\":\"\"}"
        );
    }
}
