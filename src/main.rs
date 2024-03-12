use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use std::io::{Write,Read, stdout}; 
use termion::async_stdin;  
use std::time::{Instant, Duration};

use crate::view::*; 
use crate::logic::*; 
pub mod logic; 
pub mod view;


fn main() { 
    let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

    write!(screen, "{}", termion::cursor::Hide).unwrap();
    let mut tablero = Tablero::new();
    let mut ficha = Ficha::new(); 

    write_alt_screen_msg(&mut screen, &tablero,  &ficha);

    screen.flush().unwrap(); 

    let mut b = async_stdin().bytes();
    let mut lastsec = Instant::now();
    loop {
        eprint!(">");
        match b.next() {
            Some(Ok(b'q'))=> break,
            Some(Ok(b'a')) => {
                ficha.x -=1; 
                if !check_limites_tablero(&ficha){
                    ficha.x +=1;  
                }

                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b'd')) => {
                ficha.x +=1; 
                if !check_limites_tablero(&ficha){
                    ficha.x -=1;  
                }
                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b's')) => {
                ficha.y +=1;
                if !check_limites_tablero(&ficha){
                    ficha.y -=1;  
                }

                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            _ => write_alt_screen_msg(&mut screen, &tablero,  &ficha)
        }

        if Instant::now() - lastsec >= Duration::from_secs(1) {
            ficha.y +=1;
            if !check_limites_tablero(&ficha){
                //si entras aqui quiere decir que has llegado al suelo
                ficha.y -=1;  
                poner_tetromino_en_tablero(&ficha, &mut tablero);
                
                //resetamos ficha
                ficha = Ficha::new();
            }

            lastsec = Instant::now();
            write_alt_screen_msg(&mut screen, &tablero,  &ficha)
        }
 
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
