
use core::fmt;
use termion::cursor;

const BOX_HORIZONTAL: &str = "\u{2501}";
const BOX_VERTICAL: &str = "\u{2503}";
const BOX_TOP_LEFT_CORNER: &str = "\u{250F}";
const BOX_TOP_RIGHT_CORNER: &str = "\u{2513}";
const BOX_BOTTOM_LEFT_CORNER: &str = "\u{2517}";
const BOX_BOTTOM_RIGHT_CORNER: &str = "\u{251B}";

pub struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rect = format!("{}{}{}{}",
            cursor::Goto(self.x.into(), self.y.into()),
            BOX_TOP_LEFT_CORNER,
            BOX_HORIZONTAL.repeat((self.width - 2).into()),
            BOX_TOP_RIGHT_CORNER,
        );

        for i in 1..self.height - 1 {
            rect += format!("{}{}{}{}",
                cursor::Goto(self.x, self.y + i),
                BOX_VERTICAL,
                cursor::Goto(self.x + self.width - 1, self.y + i),
                BOX_VERTICAL
            ).as_str();
        }

        rect += format!("{}{}{}{}",
            cursor::Goto(self.x.into(), (self.y + self.height - 1).into()),
            BOX_BOTTOM_LEFT_CORNER,
            BOX_HORIZONTAL.repeat((self.width - 2).into()),
            BOX_BOTTOM_RIGHT_CORNER,
        ).as_str();

        write!(f, "{}", rect)
    }
}
