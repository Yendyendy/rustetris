use std::io::{Write,Read, stdout, stdin};
use termion::input::TermRead;

use std::time::{Instant, Duration};
use std::{time, thread}; 

use crate::view::view::{Matrix, Ficha};   

pub mod func_fichas{
    //rotar
    // derecha/ izquierda
    // bajar/ teletransportar
    // colocar
    // eliminar
    // proyectar
}

pub mod func_tablero{
    //eliminar fila
}

pub mod puntuacion{
    //calcular puntuación
}

pub mod frame{
    pub fn write_alt_screen_msg<W: std::io::Write>(screen: &mut W, tablero: &crate::view::view::Matrix, ficha: &crate::view::view::Ficha) {

        // write!(screen, "{}{}\n",  termion::clear::All,termion::cursor::Goto(1, 1)).unwrap();
        write!(screen, "{}{}\n",  termion::clear::All,termion::cursor::Goto(1, 1)).unwrap();
    
        // let mut ficha = Ficha::new();
        let techo = String::from("------------\n\r");
        // let filaTablero = String::from("|                     | \n");
    
        write!(screen, "rusTetris\n\r").unwrap();
    
        write!(screen, "{}\n\r", &techo).unwrap(); 
    
        for x in 0..20 {
            write!(screen, "|").unwrap();  
            for y in 0..10 { 
                //se para solo 
                if ficha.x == x && ficha.y == y{
                    write!(screen,"*").unwrap(); 
                }else{
                    write!(screen," ").unwrap();
                }  
    
            }
            write!(screen, "|\n\r").unwrap(); 
    
        }
    
        write!(screen, "{}\n\r", &techo).unwrap();
    
        //acceleración en un futuro
        std::thread::sleep(std::time::Duration::from_millis(250)); 
        
    }
}