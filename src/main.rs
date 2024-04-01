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

    //crear pantalla
    let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
    write!(screen, "{}", termion::cursor::Hide).unwrap();

    //crear tablero
    let mut tablero = Tablero::new();
    //crear ficha
    let mut pool = TetrominoPool::new();  

    //pintar en pantalla
    write_alt_screen_msg(&mut screen, &tablero,  &pool.current());
    screen.flush().unwrap(); 
    let mut b = async_stdin().bytes();
    let mut lastsec = Instant::now();

    loop {
        //valorar input
        match b.next() {
            Some(Ok(b'q'))=> break,
            Some(Ok(b'a')) => {
                pool.current().sub_col();  
                if !se_puede_poner(&pool.current(), &tablero){
                    pool.current().add_col();
                }
                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
            },
            Some(Ok(b'w')) => { 
                pool.current().rotar();
                repeler(&mut pool.current(), &tablero);
                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
            },
            Some(Ok(b'd')) => {
                pool.current().x +=1; 
                if !se_puede_poner(&pool.current(), &tablero){
                    pool.current().x -=1;  
                }
                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
            },
            Some(Ok(b's')) => {
                pool.current().y +=1;
                if !se_puede_poner(&pool.current(), &tablero){
                    pool.current().y -=1;  
                }

                screen.flush().unwrap();
                write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
            },
            Some(Ok(b'e')) => {
                pool.current().y =19;

                //girar pool.current()
                write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
            }, 
            _ => write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
        }

        if Instant::now() - lastsec >= Duration::from_secs(1) { 
            //ver si la pool.current() puede descender: dentro de las dimensiones del tablero && posición libre
            //puede descender desciende y ya
            pool.current().y +=1;
            if se_puede_poner(&pool.current(), &tablero){ 
                // poner_tetromino_en_tablero(&pool.current(), &tablero); 
            }
            //no pude descender
            //colocar tetrominó en el tablero
            //generar nueva pool.current() 
            else{ 
                //si ya no puede poner más terminar juego
                if pool.current().y == 1{
                    break;
                }
                pool.current().y -=1;

                //si entras aqui quiere decir que has llegado al suelo
                poner_tetromino_en_tablero(&pool.current(), &mut tablero);
                validar_filas(pool.current().y, &mut tablero);
                
                //resetamos ficha
                pool.next();
            }

            lastsec = Instant::now();
            write_alt_screen_msg(&mut screen, &tablero,  &pool.current())
        }
 
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
