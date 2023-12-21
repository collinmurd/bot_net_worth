
extern crate termion;

use std::io::{Read, Write, stdout};
use std::{thread, time};

use termion::raw::IntoRawMode;
use termion::{clear, cursor};

mod shapes;
mod account;

use crate::shapes::{rectangle, text};
use crate::account::Account;

const GAME_WIDTH: u16 = 100;
const GAME_HEIGHT: u16 = 25;

fn main() {

    // initialize screen
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let game_border = rectangle::Rectangle {
        x: 2, y: 2, width: GAME_WIDTH, height: GAME_HEIGHT
    };

    let title = text::Text { x: 3, y: 3, content: "Bot Net Worth".to_string()};
    let mut account = Account::new(3, 4);

    write!(stdout, "{}{}{}{}", clear::All, game_border, title, account).unwrap();
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

        thread::sleep(time::Duration::from_millis(1000 / 30));
    }

    // exit
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
}
