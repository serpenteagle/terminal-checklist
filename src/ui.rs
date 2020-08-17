extern crate termion;
use crate::list::List;
use crate::item::Item;
use std::io::{self, Stdin, Stdout, Write};
use std::{thread, time};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct UI {
    stdout: termion::raw::RawTerminal<Stdout>,
    list: List,
    selected: usize,
}

impl UI {
    pub fn new(list: List) -> UI {
        UI {
            stdout: io::stdout().into_raw_mode().unwrap(),
            list,
            selected: 0,
        }
    }

    pub fn start(&mut self) -> &mut UI {
        self.write_list();
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1));
        self.stdout.flush();

        let stdin = io::stdin();

        for k in stdin.keys() {
            match k.unwrap() {
                Key::Char('q') => break,
                Key::Char('j') => {
                    if self.selected < self.list.items.len() - 1 {
                        self.move_selection(1);
                    }
                }
                Key::Char('k') => {
                    if self.selected > 0 {
                        self.move_selection(-1);
                    }
                }
                Key::Char(' ') => {
                    self.list.items[self.selected as usize].toggle();
                    self.write_list();
                }
                Key::Char('w') => {
                    self.list.write_file();
                }
                Key::Char('a') => {
                    self.list.add_one(Item::new("New_item", self.list.len()));
                    self.write_list();
                }
                _ => continue,
            };
        }

        self
    }

    fn move_selection(&mut self, delta: isize) -> &mut UI {
        self.selected = (self.selected as isize + delta) as usize;
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(1, (self.selected + 1) as u16)
        );
        self.stdout.flush();
        self
    }

    fn write_list(&mut self) {
        writeln!(self.stdout, "{}", termion::clear::All);

        for (line, item) in self.list.items.iter().enumerate() {
            writeln!(
                self.stdout,
                "{goto}{color}{item}{reset}",
                goto = termion::cursor::Goto(2, (line + 1) as u16),
                color = if item.checked {
                    color::Fg(color::AnsiValue::grayscale(15))
                } else {
                    color::Fg(color::AnsiValue::grayscale(23))
                },
                item = item.as_string(),
                reset = color::Fg(color::Reset)
            );
        }
        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(1, (self.selected + 1) as u16)
        );

        self.stdout.flush().unwrap();
    }
}
