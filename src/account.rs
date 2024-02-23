
use std::fmt;
use termion::cursor;

pub struct Account {
    pub x: u16,
    pub y: u16,
    cash: f64
}

impl Account {
    pub fn new(x: u16, y: u16) -> Account {
        Account { x, y, cash: 0.0 }
    }

    pub fn earn(&mut self, amount: f64) {
        self.cash += amount;
    }

    pub fn spend(&mut self, amount: f64) {
        self.cash -= amount;
    }

    pub fn cash(&self) -> f64 {
        self.cash
    }
}

impl fmt::Display for Account {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Cash on hand: ${:.2}", cursor::Goto(self.x, self.y), self.cash)
    }
}