use std::io::Write; 
  
use crate::view::{Tablero, Ficha};
use crate::Tficha;   

pub fn write_alt_screen_msg<W: Write>(screen: &mut W, _tablero: &Tablero, ficha: &Ficha) {

    // write!(screen, "{}{}\n",  termion::clear::All,termion::cursor::Goto(1, 1)).unwrap();
    write!(screen, "{}{}\n",  termion::clear::All,termion::cursor::Goto(1, 1)).unwrap();

    // let mut ficha = Ficha::new();
    let techo = String::from("------------\n\r");
    // let filaTablero = String::from("|                     | \n");

    write!(screen, "rusTetris\n\r").unwrap();

    write!(screen, "{}\n\r", &techo).unwrap(); 

    for y in 0..20 {
        write!(screen, "|").unwrap();  
        for x in 0..10 {  
            let pintado = pintar_ficha(x, y, ficha);
            if !pintado{
                if _tablero.get(y,x) == 1 {
                    print!("*");  
                }else{
                    print!(" ");  
    
                }
            }
        }
        write!(screen, "|\n\r").unwrap(); 

    }

    write!(screen, "{}\n\r", &techo).unwrap();

    //acceleración en un futuro
    std::thread::sleep(std::time::Duration::from_millis(100)); 
    
}

fn pintar_ficha(x: usize, y:usize, ficha: &Ficha)->bool{ 

    let fx:usize = ficha.x;
    let fy:usize = ficha.y;

    let xmin = if fx <= 0 {0} else {fx-1};
    let xmax = if (fx+1) > 10 {fx} else {fx+1};

    let ymin = if fy <= 0 {0} else {fy-1};
    let ymax = if (fy+1) > 20 {fy} else {fy+1};

    //ficha en rango
    if  xmin <= x && xmax >= x && ymin <= y && ymax >= y
    { 
        let x:usize = fx+1-x;
        let y:usize = fy+1-y;

        if let Tficha::O(arr) = ficha.tipo
        {
            if arr[y][x]
            {
                print!("*"); 
                return true
            } 
        }
    }
    return false

} 

//Comprueba que una ficha se pueda mover en esa dirección
pub fn check_limites_tablero(ficha: &Ficha) -> bool{

    let fx = ficha.x as i32;
    let fy = ficha.y as i32;

    if (fx-1 >= 0) && (fx < 10) && (fy < 20){
        return true;
    }else{
        return false;
    } 
}

pub fn poner_tetromino_en_tablero(ficha: &Ficha, tablero: &mut Tablero){
    // let forma_geo = ficha.tipo as [[bool:3]: 3];
    
    let fx = ficha.x;
    let fy = ficha.y;

    for y in 0..3 {
        for x in 0..3{
            if let Tficha::O(arr) = ficha.tipo {
                if arr[y][x] {
                    if (fy-y+1) as isize > 0 &&  (fx-x+1) as isize >0 {
                        tablero.set((fy-y+1) as usize, (fx-x+1) as usize, 1);
                        
                    }

                }
            }
        }
    }
}

/*
Notas:
Hay que diferenciar entre limites del tablero y colisiones con otras fichas

*/