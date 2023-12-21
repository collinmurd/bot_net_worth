
use core::fmt;
use termion::cursor;

const BOX_HORIZONTAL: &str = "\u{2501}";
const BOX_VERTICAL: &str = "\u{2503}";

pub enum LineOrientation {
    Horizontal,
    Vertical
}

pub struct Line {
    pub orientation: LineOrientation,
    pub x: u16,
    pub y: u16,
    pub length: u16
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.orientation {
            LineOrientation::Horizontal => {
                write!(f, "{}{}",
                    cursor::Goto(self.x, self.y),
                    BOX_HORIZONTAL.repeat(self.length.into())
                )
            },
            LineOrientation::Vertical => {
                write!(f, "{}",
                    (self.y..self.length + self.y)
                        .map(|i| format!("{}{}", cursor::Goto(self.x, i), BOX_VERTICAL))
                        .collect::<Vec<String>>()
                        .join("")
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::shapes::line::{Line, LineOrientation};
    use crate::shapes::line::BOX_HORIZONTAL;
    use crate::shapes::line::BOX_VERTICAL;

    const ESC: &str = "\u{001B}";

    #[test]
    fn test_display_horizontal() {
        let line = Line {orientation: LineOrientation::Horizontal, x: 2, y: 4, length: 3};
        assert_eq!(
            format!("{}", line),
            format!("{ESC}[4;2H{}", BOX_HORIZONTAL.repeat(3))
        )
    }

    #[test]
    fn test_display_vertical() {
        let line = Line {orientation: LineOrientation::Vertical, x: 2, y: 4, length: 3};
        assert_eq!(
            format!("{}", line),
            format!("{ESC}[4;2H{BOX_VERTICAL}{ESC}[5;2H{BOX_VERTICAL}{ESC}[6;2H{BOX_VERTICAL}")
        )
    }
}