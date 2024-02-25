
use std::fmt;
use std::slice::IterMut;
use std::time::Duration;

use termion::cursor;

use crate::shapes::rectangle;

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

pub enum BusinessSelectDirection {
    Up,
    Right,
    Left,
    Down
}

pub struct Business {
    pub name: String,

    sale_time: Duration,
    sale_progress: Duration,
    sale_amount: f64,

    level: u16,
    pub level_up_cost: f64
}

impl Business {
    pub fn new(name: String,
               init_sale_time: Duration,
               init_sale_amount: f64,
               level_up_cost: f64
    ) -> Business {
        Business {
            name,
            sale_time: init_sale_time,
            sale_progress: Duration::ZERO,
            sale_amount: init_sale_amount,
            level: 1,
            level_up_cost
        }
    }

    pub fn progress(&mut self, time: Duration) -> Option<f64> {
        if self.level < 1 {
            return None;
        }
        self.sale_progress += time;
        if self.sale_progress > self.sale_time {
            self.sale_progress = Duration::ZERO;
            return Some(self.sale_amount * self.level as f64);
        }

        return None;
    }

    pub fn upgrade(&mut self) {
        self.level += 1;
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

        let level_line = format!("Level: {} Revenue: ${:.2}", self.level, self.sale_amount * self.level as f64);
        write!(f, "{}\n{}{}\n{}[{:<20}]{}",
            self.name,
            cursor::Left(self.name.len() as u16),
            level_line,
            cursor::Left(level_line.len() as u16),
            self.progress_bar(),
            timer
        )
    }
}

pub struct BusinessContainer {
    pub x: u16,
    pub y: u16,
    pub businesses: Vec<Business>,

    selected: Option<usize>
}


impl fmt::Display for BusinessContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        for (i, b) in self.businesses.iter().enumerate() {
            let x = if i % 2 == 0 {self.x + 1} else {self.x + 45};
            let y = self.y + (i / 2 * 5) as u16 + 1;

            display += format!("{}{}", cursor::Goto(x, y), b).as_str();

            if self.selected.is_some_and(|x| x == i) {
                display += format!("{}", rectangle::Rectangle{
                    x: x - 1,
                    y: y - 1,
                    width: 33,
                    height: 6
                }).as_str();
            }
        }

        write!(f, "{display}")
    }
}

impl BusinessContainer {

    pub fn new(x: u16, y: u16, businesses: Vec<Business>) -> BusinessContainer {
        if businesses.len() < 1 {
            panic!("At least one business must be supplied");
        }
        BusinessContainer {
            x, y, businesses,
            selected: Some(0)
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Business> {
        self.businesses.iter_mut()
    }

    pub fn select_business(&mut self, direction: BusinessSelectDirection) {
        match direction {
            BusinessSelectDirection::Up => {
                if self.selected.is_some_and(|x| x > 1) {
                    self.selected = Some(self.selected.unwrap() - 2);
                }
            },
            BusinessSelectDirection::Right => {
                if self.selected.is_some_and(|x| x % 2 == 0) {
                    self.selected = Some(self.selected.unwrap() + 1)
                }
            },
            BusinessSelectDirection::Down => {
                if self.selected.is_some_and(|x| x < self.businesses.len() - 2) {
                    self.selected = Some(self.selected.unwrap() + 2)
                }
            }
            BusinessSelectDirection::Left => {
                if self.selected.is_some_and(|x| x % 2 == 1) {
                    self.selected = Some(self.selected.unwrap() - 1)
                }
            }
        }
    }

    pub fn get_mut_selected_business(&mut self) -> Option<&mut Business> {
        if self.selected.is_some() {
            return Some(self.businesses.get_mut(self.selected.unwrap()).unwrap());
        }
        return None;
    }
}


#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::business::BusinessSelectDirection;

    use super::{Business, BusinessContainer};


    #[test]
    fn test_business_progress() {
        let mut business = Business::new("asdf".to_string(), Duration::from_millis(1500), 1.0, 1.0);

        let mut result = business.progress(Duration::ZERO);
        assert!(result.is_none());
        assert_eq!(business.sale_progress, Duration::ZERO);

        business.level = 1;
        result = business.progress(Duration::from_millis(1000));
        assert!(result.is_none());
        assert!(business.sale_progress > Duration::ZERO);

        result = business.progress(Duration::from_millis(1000));
        assert!(result.is_some_and(|x| x == 1.0));
        assert_eq!(business.sale_progress, Duration::ZERO);
    }

    #[test]
    fn test_business_container_select_business() {
        let mut cont = BusinessContainer::new(
            1,
            1,
            vec![
                Business::new("Antivirus Software".to_string(), Duration::from_secs(60), 7.0, 1.0),
                Business::new("Antivirus Software".to_string(), Duration::from_secs(60), 7.0, 1.0),
                Business::new("Antivirus Software".to_string(), Duration::from_secs(60), 7.0, 1.0),
                Business::new("Antivirus Software".to_string(), Duration::from_secs(60), 7.0, 1.0)
            ]
        );

        assert!(cont.selected.is_some_and(|x| x == 0));

        // Down
        cont.select_business(BusinessSelectDirection::Down);
        assert!(cont.selected.is_some_and(|x| x == 2));
        cont.select_business(BusinessSelectDirection::Down);
        assert!(cont.selected.is_some_and(|x| x == 2));

        // Up
        cont.select_business(BusinessSelectDirection::Up);
        assert!(cont.selected.is_some_and(|x| x == 0));
        cont.select_business(BusinessSelectDirection::Up);
        assert!(cont.selected.is_some_and(|x| x == 0));

        // Right
        cont.select_business(BusinessSelectDirection::Right);
        assert!(cont.selected.is_some_and(|x| x == 1));
        cont.select_business(BusinessSelectDirection::Right);
        assert!(cont.selected.is_some_and(|x| x == 1));

        // Left
        cont.select_business(BusinessSelectDirection::Left);
        assert!(cont.selected.is_some_and(|x| x == 0));
        cont.select_business(BusinessSelectDirection::Left);
        assert!(cont.selected.is_some_and(|x| x == 0));
    }
}