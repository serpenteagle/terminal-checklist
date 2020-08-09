use std::path::Path;
use terminal_checklist::list::List;
use terminal_checklist::ui::UI;

fn main() {
    let list = List::from_file(&Path::new("test.list"));
    // println!("{:?}", list);

    let mut ui = UI::new(list);
    ui.start();
}
