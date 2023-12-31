
extern crate termion;

use std::io::{Read, Write, stdout};
use std::time::Duration;
use std::{thread, time};

use business::BusinessContainer;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

mod account;
mod business;
mod shapes;

use crate::account::Account;
use crate::business::Business;
use crate::shapes::{rectangle, text};

const GAME_WIDTH: u16 = 100;
const GAME_HEIGHT: u16 = 25;
const FPS: u16 = 30;

fn main() {

    // initialize screen
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let game_border = rectangle::Rectangle {x: 2, y: 2, width: GAME_WIDTH, height: GAME_HEIGHT};
    let title = text::Text { x: 3, y: 3, content: "Bot Net Worth".to_string()};
    let mut account = Account::new(3, 4);
    let mut businesses = init_bussiness(3, 6);


    write!(stdout, "{}{}{}{}", clear::All, game_border, title, account).unwrap();
    write!(stdout, "{}{}", cursor::Goto(3, 6), businesses).unwrap();
    stdout.flush().unwrap();

    // game loop
    let mut stdin = termion::async_stdin().bytes();
    loop {
        match stdin.next(){
            Some(b) => {

                match b.as_ref().unwrap() { // not sure why the byte queue needs Result for the possibility of an error...
                    b'q' => break,
                    _ => write!(stdout, "{}{}", cursor::Goto(1,1), b.unwrap()).unwrap()
                }
            },
            None => ()
        }

        stdout.flush().unwrap();

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
    BusinessContainer {
        x,
        y,
        businesses: vec! [
            Business::new("Crypto Mining".to_string(), Duration::from_secs(10), 0.05),
            Business::new("Selling RAM Online".to_string(), Duration::from_secs(30), 3.0),
            Business::new("Antivirus Software".to_string(), Duration::from_secs(60), 7.0),
            Business::new("Floppy Discs".to_string(), Duration::from_secs(60 * 3), 25.0),
            Business::new("Extra USB Ports".to_string(), Duration::from_secs(60 * 10), 60.0),
            Business::new("NFT Storage".to_string(), Duration::from_secs(60 * 22), 160.0),
        ]
    }
}