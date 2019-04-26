#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::io::{self, stdout, Read};

use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::screen::*;

mod ui;

pub fn main() -> Result<(), io::Error> {
    env_logger::init();
    let mut stdin = async_stdin().bytes();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let mut cmd_buff: Vec<u8> = vec![];
    let mut body_buff: Vec<u8> = vec![];

    ui::redraw(&mut screen, body_buff.as_ref(), cmd_buff.as_ref())?;

    loop {
        let next_char = stdin.next();

        if let Some(c) = next_char {
            if let Ok(b) = c {
                match b {
                    13 => {
                        body_buff.push(b'\n');
                        body_buff.push(13);
                        body_buff.append(&mut cmd_buff);
                        cmd_buff.clear();
                    }
                    27 => break, // Escape
                    127 => {
                        let _ = cmd_buff.pop();
                    }

                    b => {
                        cmd_buff.push(b.into());
                    }
                }
            };

            ui::redraw(&mut screen, &body_buff, &cmd_buff)?;
        };
    }
    Ok(())
}
