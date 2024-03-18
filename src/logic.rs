use std::io::Write; 
  
use crate::view::{Tablero, TetrominoGame}; 

pub fn write_alt_screen_msg<W: Write>(screen: &mut W, _tablero: &Tablero, ficha: &TetrominoGame) {

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

fn pintar_ficha(x: usize, y:usize, ficha: &TetrominoGame)->bool{ 

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
        let arr = ficha.tipo.get();
 
        if arr[y][x]
        {
            print!("*"); 
            return true
        }  

    }
    return false

} 

//ver si la ficha puede descender: dentro de las dimensiones del tablero && posición libre 
//Comprueba que una ficha se pueda mover en esa dirección
pub fn se_puede_poner(ficha: &TetrominoGame,tablero: & Tablero) -> bool{

    let fx = ficha.x as i32;
    let fy = ficha.y as i32; 

    let arr = ficha.tipo.get();

    let mut en_rango: bool = false;
    let mut posiciones_libres: bool = true;

    if (fx-1 >= 0) && (fx < 10) && (fy < 20){
        en_rango = true;
    }

    'outer:for y in 0..3 {
        for x in 0..3{ 
            let row :isize = (fy as isize) -(y as isize) + 1;
            let col :isize = (fx as isize) - (x as isize ) + 1;
            if arr[y][x] && row >= 0 && row < 20 &&  col >=0 && col < 10{ 
                if tablero.get(row as usize, col as usize) > 0 {
                    posiciones_libres = false;
                    break 'outer;
                }
            } 
        }
    }

    return en_rango && posiciones_libres;
}

pub fn poner_tetromino_en_tablero(ficha: &TetrominoGame, tablero: &mut Tablero){
    // let forma_geo = ficha.tipo as [[bool:3]: 3];
    
    let fx = ficha.x as isize;
    let fy = ficha.y as isize;
    let arr = ficha.tipo.get();

    for y in 0..3 {
        for x in 0..3{ 
            if arr[y][x] {
                let row :isize = fy -(y as isize) + 1;
                let col :isize = fx - (x as isize ) + 1;

                if  row >= 0 && row < 20 &&  col >=0 && col < 10 {
                    tablero.set(row as usize, col as usize, 1);
                }
            } 
        }
    }
}

//ideas a futuro: tabla como matriz de bits
pub fn validar_filas(pos: usize, tablero: &mut Tablero){
    eprint!("validar_filas1\n");

    let rango_sup = pos-1 ;
    let rango_inf = if pos+1 > 20 {pos+1} else {pos};
    
    let mut eliminar : bool = true;
    
    for y in rango_sup ..= rango_inf{
        eprint!("y{}\n", y);

        for x in 0..10{
            if tablero.get(y, x) == 0{
                eliminar = false;
                break;
            }
        } 

        if eliminar {
            eprint!("validar_filas2 {}\n", eliminar);

            eliminar_fila(y, tablero);
        }
    }
}

fn eliminar_fila(fila: usize, tablero: &mut Tablero){
    eprint!("eliminar_fila {}\n", fila);

    if fila == 0 {
        for x in 0..10{ 
            tablero.set(fila, x, 0);
        }  
    }else{
        for x in 0..10{ 
            tablero.set(fila, x, tablero.get(fila-1, x));
        }  
        eliminar_fila(fila-1, tablero);

    }
}