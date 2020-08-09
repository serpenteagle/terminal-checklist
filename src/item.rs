#[derive(Debug)]
pub struct Item {
    id: u64,
    pub name: String,
    pub checked: bool,
}

impl Item {
    pub fn new(name: &str, id: u64) -> Item {
        Item {
            id,
            name: name.to_string(),
            checked: false,
        }
    }

    pub fn check(&mut self) -> &mut Item {
        self.checked = true;
        self
    }

    pub fn rename(&mut self, new_name: &str) -> &mut Item {
        self.name = new_name.to_string();
        self
    }

    pub fn as_string(&self) -> String {
        format!("[{}] {}", if self.checked { "X" } else { " " }, self.name)
    }

    pub fn parse(item: &str, id: u64) -> Item {
        let parts: Vec<&str> = item.split(' ').collect();
        let name = if parts.len() == 3 {
            parts[2].to_string()
        } else {
            parts[1].to_string()
        };
        let check = if parts.len() == 3 {
            ' '
        } else {
            parts[0].chars().nth(1).unwrap()
        };

        let checked = match check {
            'X' => Ok(true),
            ' ' => Ok(false),
            _ => Err(()),
        }
        .expect("Error parsing");

        Item { id, name, checked }
    }
}

mod tests {
    use super::Item;

    #[test]
    fn general() {
        let mut item = Item::new("Apple", 2);
        assert_eq!(item.name, String::from("Apple"));
        assert_eq!(item.checked, false);
        assert_eq!(item.id, 2);

        item.check();
        assert_eq!(item.checked, true);

        item.rename("Orange");
        assert_eq!(item.name, String::from("Orange"));

        assert_eq!(item.as_string(), format!("[X] Orange"));
    }

    #[test]
    fn parse_item() {
        let string_rep = "[X] Banana";
        let item = Item::parse(&string_rep, 3);

        assert_eq!(item.id, 3);
        assert_eq!(item.name, String::from("Banana"));
        assert_eq!(item.checked, true);
    }
}
