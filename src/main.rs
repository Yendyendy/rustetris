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
                if !se_puede_poner(&ficha, &tablero){
                    ficha.x +=1;  
                }

                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b'd')) => {
                ficha.x +=1; 
                if !se_puede_poner(&ficha, &tablero){
                    ficha.x -=1;  
                }
                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b's')) => {
                ficha.y +=1;
                if !se_puede_poner(&ficha, &tablero){
                    ficha.y -=1;  
                }

                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b'e')) => {
                ficha.y =19;

                //girar ficha
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            Some(Ok(b'w')) => {
                //girar ficha
                write_alt_screen_msg(&mut screen, &tablero,  &ficha)
            },
            _ => write_alt_screen_msg(&mut screen, &tablero,  &ficha)
        }

        if Instant::now() - lastsec >= Duration::from_secs(1) {

            //ver si la ficha puede descender: dentro de las dimensiones del tablero && posición libre
            //puede descender desciende y ya
            ficha.y +=1;
            if se_puede_poner(&ficha, &tablero){ 
                // poner_tetromino_en_tablero(&ficha, &tablero); 
            }
            //no pude descender
            //colocar tetrominó en el tablero
            //generar nueva ficha 
            else{
                ficha.y -=1;

                //si entras aqui quiere decir que has llegado al suelo
                poner_tetromino_en_tablero(&ficha, &mut tablero);
                validar_filas(ficha.y, &mut tablero);
                
                //resetamos ficha
                ficha = Ficha::new();
            }


            lastsec = Instant::now();
            write_alt_screen_msg(&mut screen, &tablero,  &ficha)
        }
 
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
