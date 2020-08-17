use std::env;
use std::path::Path;
use std::process;
use terminalist::list::List;
use terminalist::ui::UI;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).unwrap_or_else(|| {
        println!("Error: no file provided");
        process::exit(1);
    });

    let list = List::from_file(&Path::new(&file));

    let mut ui = UI::new(list);
    ui.start();
}
