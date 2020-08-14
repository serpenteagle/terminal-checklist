use std::path::Path;
use terminalist::list::List;
use terminalist::ui::UI;

fn main() {
    let list = List::from_file(&Path::new("test.list"));
    // println!("{:?}", list);

    let mut ui = UI::new(list);
    ui.start();
}
