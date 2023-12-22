
use std::fmt;
use std::slice::IterMut;
use std::time::Duration;

use termion::cursor;

const BAR_BLOCKS: [&str; 8] = [
    "\u{258F}", // ▏
    "\u{258E}", // ▎
    "\u{258D}", // ▍
    "\u{258C}", // ▌
    "\u{258B}", // ▋
    "\u{258A}", // ▊
    "\u{2589}", // ▉
    "\u{2588}", // █
];

pub struct Business {
    pub name: String,
    sale_time: Duration,
    pub sale_progress: Duration,
    sale_amount: f64
}

impl Business {
    pub fn new(name: String,
               init_sale_time: Duration,
               init_sale_amount: f64
    ) -> Business {
        Business {
            name,
            sale_time: init_sale_time,
            sale_progress: Duration::ZERO,
            sale_amount: init_sale_amount
        }
    }

    pub fn progress(&mut self, time: Duration) -> Option<f64> {
        self.sale_progress += time;
        if self.sale_progress > self.sale_time {
            self.sale_progress = Duration::ZERO;
            return Some(self.sale_amount);
        }

        return None;
    }

    fn progress_bar(&self) -> String {
        if self.sale_progress.is_zero() {
            return String::new();
        }
        let value = self.sale_progress.as_secs_f32() / self.sale_time.as_secs_f32();
        let full_blocks = (value * 20.0) as usize;
        let partial_block = BAR_BLOCKS[((value * 20.0 - full_blocks as f32) * 8.0) as usize];
        (BAR_BLOCKS[7].repeat(full_blocks) + partial_block).to_string()
    }
}

impl fmt::Display for Business {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let remaining = (self.sale_time - self.sale_progress).as_secs();
        let timer = format!("{:0>2}:{:0>2}:{:0>2}",
            remaining / 3600,
            (remaining - (remaining / 3600 * 3600)) / 60,
            remaining % 60
        );

        write!(f, "{:<30}\n{}[{:<20}]{}",
            format!("{}  ${:.2}", self.name, self.sale_amount),
            cursor::Left(30),
            self.progress_bar(),
            timer
        )
    }
}

pub struct BusinessContainer {
    pub x: u16,
    pub y: u16,
    pub businesses: Vec<Business>
}


impl fmt::Display for BusinessContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for (i, b) in self.businesses.iter().enumerate() {
            if i % 2 == 0 {
                display += format!("{}{}", cursor::Goto(self.x + 1, self.y + (i / 2 * 4) as u16 + 1), b).as_str();
            } else {
                display += format!("{}{}", cursor::Goto(self.x + 45, self.y + (i / 2 * 4) as u16 + 1), b).as_str();
            }
        }

        write!(f, "{display}")
    }
}

impl BusinessContainer {
    pub fn iter_mut(&mut self) -> IterMut<'_, Business> {
        self.businesses.iter_mut()
    }
}


#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::Business;


    #[test]
    fn test_progress() {
        let mut business = Business::new("asdf".to_string(), Duration::from_millis(1500), 1.0);

        let mut result = business.progress(Duration::from_millis(1000));
        assert!(result.is_none());
        assert!(business.sale_progress > Duration::ZERO);
        
        result = business.progress(Duration::from_millis(1000));
        assert!(result.is_some_and(|x| x == 1.0));
        assert_eq!(business.sale_progress, Duration::ZERO);
    }
}