
pub struct Account {
    cash: f64
}

impl Account {
    fn new() {
        Account { cash: 0 }
    }
}

impl fmt::Display for Account {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.cash)
    }
}