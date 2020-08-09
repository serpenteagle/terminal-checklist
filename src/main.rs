use std::path::Path;
use terminal_checklist::list::List;

fn main() {
    let list = List::from_file(&Path::new("test.list"));
    println!("{:?}", list);
}
