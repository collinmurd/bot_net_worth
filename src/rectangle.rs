
use core::fmt;
use termion::cursor;

use crate::line::{Line, LineOrientation};

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
        write!(f, "{}{}{}{}{}{}{}{}{}{}",
            // top
            cursor::Goto(self.x, self.y),
            BOX_TOP_LEFT_CORNER,
            Line{orientation: LineOrientation::Horizontal, x: self.x + 1, y: self.y, length: self.width - 2},
            BOX_TOP_RIGHT_CORNER,
            // left
            Line{orientation: LineOrientation::Vertical, x: self.x, y: self.y + 1, length: self.height - 2},
            // right
            Line{orientation: LineOrientation::Vertical, x: self.x + self.width - 1, y: self.y + 1, length: self.height - 2},
            // bottom
            cursor::Goto(self.x, self.y + self.height - 1),
            BOX_BOTTOM_LEFT_CORNER,
            Line{orientation: LineOrientation::Horizontal, x: self.x + 1, y: self.y + self.height - 1, length: self.width - 2},
            BOX_BOTTOM_RIGHT_CORNER
        )
    }
}

#[cfg(test)]
mod test {
    use crate::rectangle as rect;

    #[test]
    fn test_display() {
        let r = rect::Rectangle {
            x: 2, y: 2, width: 3, height: 3
        };
        assert_eq!(
            format!("{}", r),
            format!("\u{1b}[2;2H┏\u{1b}[2;3H━┓\u{1b}[3;2H┃\u{1b}[3;4H┃\u{1b}[4;2H┗\u{1b}[4;3H━┛")
        )
    }
}