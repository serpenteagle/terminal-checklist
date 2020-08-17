use crate::item::Item;
use std::fs;
use std::path::Path;
// use uuid::Uuid;

#[derive(Debug)]
pub struct List {
    // id: u128,
    pub name: String,
    pub items: Vec<Item>,
    pub file: Option<String>,
}

impl List {
    pub fn new(name: &str) -> List {
        List {
            // id: Uuid::new_v4().as_u128(),
            name: name.to_string(),
            items: Vec::new(),
            file: None,
        }
    }

    pub fn rename(&mut self, new_name: &str) -> &mut List {
        self.name = new_name.to_string();
        self
    }

    pub fn add_one(&mut self, item: Item) -> &mut List {
        self.items.push(item);
        self
    }

    pub fn add_many(&mut self, items: Vec<Item>) -> &mut List {
        for item in items.into_iter() {
            self.items.push(item);
        }
        self
    }

    pub fn from_file(file: &Path) -> List {
        let name = file.file_name().unwrap().to_str().unwrap().to_string();
        let contents = fs::read_to_string(file).expect("Error reading file");
        let mut items = Vec::new();

        for (num, line) in contents.lines().enumerate() {
            items.push(Item::parse(line, num as u64));
            // println!("{}", line);
        }

        List {
            name,
            items,
            file: Some(String::from(file.to_str().unwrap())),
        }
    }

    pub fn as_string(&self) -> String {
        let mut result = String::new();
        for item in self.items.iter() {
            let as_string = format!("{}\n", &item.as_string());
            result.push_str(&as_string);
        }
        result
    }

    pub fn write_file(&self) {
        match &self.file {
            Some(path) => {
                fs::write(&path, &self.as_string());
            }
            None => {
                panic!("No associated file");
            }
        }
    }
}

mod tests {
    use super::{Item, List};
    use std::fs;
    use std::path::Path;

    #[test]
    fn general() {
        let mut list = List::new("Shopping");
        list.rename("Grocery");
        assert_eq!(list.name, String::from("Grocery"));

        let item_1 = Item::new("Apples", 0);
        list.add_one(item_1);
        assert_eq!(list.items[0].name, String::from("Apples"));

        let item_2 = Item::new("Oranges", 1);
        let item_3 = Item::new("Bananas", 2);
        let item_4 = Item::new("Watermelons", 3);
        let items = vec![item_2, item_3, item_4];

        list.add_many(items);
        assert_eq!(list.items[1].name, String::from("Oranges"));
        assert_eq!(list.items[2].name, String::from("Bananas"));
        assert_eq!(list.items[3].name, String::from("Watermelons"));
    }

    #[test]
    fn parse_from_file() {
        let list = List::from_file(&Path::new("test.list"));
        println!("{:?}", list)
    }

    #[test]
    fn list_as_string() {
        let raw = fs::read_to_string("test.list").expect("Error reading file");
        let list = List::from_file(Path::new("test.list"));
        let as_string = list.as_string();

        assert_eq!(raw, as_string);
    }
}
