
use core::fmt;
use termion::cursor;

pub struct Text {
    pub x: u16,
    pub y: u16,
    pub content: String
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", cursor::Goto(self.x, self.y), self.content)
    }
}


#[cfg(test)]
mod test {
    use crate::text::Text;

    const ESC: &str = "\u{001B}";

    #[test]
    fn test_display() {
        let text = Text {x: 3, y: 5, content: "Hello there".to_string()};

        assert_eq!(
            format!("{}", text),
            format!("{ESC}[5;3HHello there")
        )
    }
}