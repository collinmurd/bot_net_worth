
use std::fmt;
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
        BAR_BLOCKS[7].repeat(((self.sale_progress.as_secs_f32() / self.sale_time.as_secs_f32()) * 20.0).round() as usize)
    }
}

impl fmt::Display for Business {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}[{:<20}]{}",
            self.name,
            cursor::Left(self.name.len().try_into().unwrap()),
            self.progress_bar(),
            self.sale_progress.as_secs()
        )
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