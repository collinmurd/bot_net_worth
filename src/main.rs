
extern crate termion;

use std::io::{Read, Write, stdout};
use std::time::Duration;
use std::{thread, time};

use business::{BusinessContainer, BusinessSelectDirection};
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

mod account;
mod business;
mod shapes;

use crate::account::Account;
use crate::business::Business;
use crate::shapes::{rectangle, text};

const GAME_WIDTH: u16 = 100;
const GAME_HEIGHT: u16 = 30;
const FPS: u16 = 30;

fn main() {

    // initialize screen
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let game_border = rectangle::Rectangle {x: 2, y: 2, width: GAME_WIDTH, height: GAME_HEIGHT};
    let title = text::Text { x: 3, y: 3, content: "Bot Net Worth".to_string()};
    let mut account = Account::new(3, 4);
    let mut businesses = init_bussiness(4, 6);

    // game loop
    let mut stdin = termion::async_stdin();
    loop {
        // reset screen 
        write!(stdout, "{}{}{}{}", clear::All, game_border, title, account).unwrap();
        write!(stdout, "{}{}", cursor::Goto(3, 6), businesses).unwrap();

        let mut buf: Vec<u8> = Vec::new();
        stdin.read_to_end(&mut buf).unwrap();
        match buf.pop() {
            Some(b) => {

                match b {
                    // exit
                    b'q' => break,

                    // menu
                    49 => match businesses.get_mut_selected_business() {
                        Some(business) => upgrade_business(&mut account, business),
                        None => ()
                    },

                    // select businesses
                    65 => businesses.select_business(BusinessSelectDirection::Up),
                    67 => businesses.select_business(BusinessSelectDirection::Right),
                    66 => businesses.select_business(BusinessSelectDirection::Down),
                    68 => businesses.select_business(BusinessSelectDirection::Left),
                    _ => write!(stdout, "{}{}", cursor::Goto(1,1), b).unwrap()
                }
            },
            None => ()
        }

        for business in businesses.iter_mut() {
            if let Some(amount) = business.progress(Duration::from_secs_f32(1.0 / FPS as f32)) {
                account.earn(amount);
                write!(stdout, "{}", account).unwrap();
            }
        }
        write!(stdout, "{}{}", cursor::Goto(3, 6), businesses).unwrap();
        stdout.flush().unwrap();

        thread::sleep(time::Duration::from_secs_f32(1.0 / FPS as f32));
    }

    // exit
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}

fn init_bussiness(x: u16, y: u16) -> BusinessContainer {
    BusinessContainer::new(
        x,
        y,
        vec! [
            Business::new("Crypto Mining".to_string(), Duration::from_secs(10), 0.05),
            Business::new("Selling RAM Online".to_string(), Duration::from_secs(30), 3.0),
            Business::new("Antivirus Software".to_string(), Duration::from_secs(60), 7.0),
            Business::new("Floppy Discs".to_string(), Duration::from_secs(60 * 3), 25.0),
            Business::new("Extra USB Ports".to_string(), Duration::from_secs(60 * 10), 60.0),
            Business::new("NFT Storage".to_string(), Duration::from_secs(60 * 22), 160.0),
        ]
    )
}

fn upgrade_business(account: &mut Account, business: &mut Business) {
    if business.level_up_cost < account.cash() {
        account.spend(business.level_up_cost);
        business.upgrade();
    }
}