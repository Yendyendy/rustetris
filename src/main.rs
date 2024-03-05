use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen, ToMainScreen};
use std::io::{Write,Read, stdout, stdin};
use std::{time, thread}; 
use termion::async_stdin;  
use std::time::{Instant, Duration};


use crate::logic::frame;
use crate::view::view::*; 
pub mod logic; 
pub mod view;





fn main() {
    let stdin = stdin();
    let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

    write!(screen, "{}", termion::cursor::Hide).unwrap();
    let mut tablero = Matrix::new();
    let mut ficha = Ficha {x :0, y: 5};
    let mut aux :i32 = 0;

    frame::write_alt_screen_msg(&mut screen, &tablero,  &ficha);

    screen.flush().unwrap(); 

    let mut b = async_stdin().bytes();
    let mut lastsec = Instant::now();
    loop { 
        

        
        match b.next() {
            Some(Ok(b'q'))=> break,
            Some(Ok(b'a')) => {
                ficha.y -=1; 

                screen.flush().unwrap();
                frame::write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b'd')) => {
                ficha.y +=1; 

                screen.flush().unwrap();
                frame::write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            _ => frame::write_alt_screen_msg(&mut screen, &tablero,  &ficha)
        }

        if Instant::now() - lastsec >= Duration::from_secs(1) {
            ficha.x +=1;
            lastsec = Instant::now();
            frame::write_alt_screen_msg(&mut screen, &tablero,  &ficha)
        }
 
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
