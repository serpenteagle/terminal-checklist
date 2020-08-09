extern crate termion;
use crate::list::List;
use std::io::{self, Stdin, Stdout, Write};
use std::{thread, time};
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
        // thread::sleep(time::Duration::from_millis(1000));
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1));
        self.stdout.flush();
        // thread::sleep(time::Duration::from_millis(1000));
        self.selected = 0;

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
                "{}{}",
                termion::cursor::Goto(2, (line + 1) as u16),
                item.as_string()
            );
        }
        write!(self.stdout, "{}", termion::cursor::Goto(1, (self.selected + 1) as u16));

        self.stdout.flush().unwrap();
    }
}
