
extern crate termion;

use core::fmt;
use std::io::{Read, Write, stdout};

use termion::raw::IntoRawMode;
use termion::{clear, cursor};

mod rectangle;

fn main() {

    // initialize screen
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    let header_box = rectangle::Rectangle {
        x: 10, y: 10, width: 20, height: 10
    };

    write!(stdout, "{}{}", clear::All, header_box).unwrap();
    stdout.flush().unwrap();

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
    }

    // exit
    write!(stdout, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
}


pub struct Account {
    cash: f64
}

impl fmt::Display for Account {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.cash)
    }
}