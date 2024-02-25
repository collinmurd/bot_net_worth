

use std::fmt::{self};
use termion::cursor;

pub struct Menu {
    pub x: u16,
    pub y: u16,

    options: Vec<String>
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for (i, option) in self.options.iter().enumerate() {
            display += format!("{}{}", cursor::Goto(self.x, self.y + i as u16), option).as_str();
        }

        write!(f, "{display}")
    }
}

impl Menu {
    pub fn new(x: u16, y: u16, options: Vec<String>) -> Menu {
        Menu {x, y, options}
    }

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn add_option(&mut self, option: String) {
        self.options.push(
            format!("{}. {}", self.options.len() + 1, option)
        )
    }
}