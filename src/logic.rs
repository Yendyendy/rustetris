use std::io::Write; 
  
use crate::view::{Tablero, TetrominoGame}; 

pub fn write_alt_screen_msg<W: Write>(screen: &mut W, _tablero: &Tablero, ficha: &TetrominoGame) {

    let mut tablero_aux = _tablero.clone();
    // write!(screen, "{}{}\n",  termion::clear::All,termion::cursor::Goto(1, 1)).unwrap();
    write!(screen, "{}{}\n",  termion::clear::All,termion::cursor::Goto(1, 1)).unwrap();

    // let mut ficha = Ficha::new();
    let techo = String::from("------------\n\r");
    // let filaTablero = String::from("|                     | \n");

    write!(screen, "rusTetris\n\r").unwrap();

    write!(screen, "{}\n\r", &techo).unwrap();  

    poner_tetromino_en_tablero(ficha, &mut tablero_aux);

    //     write!(screen, "|").unwrap();  
    //     for x in 0..10 {  
              // let pintado = pintar_ficha(fila, columna, ficha);
            // if !pintado{
            //     if _tablero.get(fila,columna) == 1 {
            //         print!("*");  
            //     }else{
            //         print!(" ");  
    
            //     }
            // }
    //     }
    //     write!(screen, "|\n\r").unwrap(); 

    for fila in tablero_aux.rows {
        write!(screen, "|").unwrap();  

        for casilla in fila{
            if casilla == 1 {
                print!("*");
            }else{
                print!(" ");
            }
        }
        write!(screen, "|\n\r").unwrap(); 

    }

    write!(screen, "{}\n\r", &techo).unwrap();

    //acceleración en un futuro
    std::thread::sleep(std::time::Duration::from_millis(100)); 
    
}

fn pintar_ficha(ficha: &TetrominoGame, tablero: &Tablero){ 

    // let fx = ficha.x as isize;
    // let fy = ficha.y as isize;  
    // let x = fx+1-x;
    // let y = fy+1-y;
    
    // if x >= 0 && x <= 2 && y >= 0 && y <= 2 {
    //     let arr = ficha.tipo.get();
    //     if arr[y as usize][x as usize]
    //     {
    //         print!("*"); 
    //         return true
    //     }   
    // }
 
    // return false

} 

fn usize_add(u: usize, i: isize) -> usize {

    let res : Option<usize> = if i.is_negative() { 
        u.checked_sub(isize::abs(i) as usize)
    } else {
        u.checked_add(i as usize)
    };

    match res{
        Some(n) => n,
        None=> {
            eprint!("ERROR :usize_add({u},{i})\n");
            u
        }
    }
}



//ver si la ficha puede descender: dentro de las dimensiones del tablero && posición libre 
//Comprueba que una ficha se pueda mover en esa dirección
pub fn se_puede_poner(ficha: &TetrominoGame, tablero: & Tablero) -> bool{

    let fx = ficha.x;
    let fy = ficha.y;  

    let mut en_rango: bool = true;
    let mut posiciones_libres: bool = true;

    let mut y:isize = -1;
    'outer1: for filas in ficha.tipo.get(){
        let mut x:isize = -1;
        for columna in filas {
            
            
            if *columna {
                
                if let Some(u) = (fx as isize).checked_add(x){
                    let aux = u.is_negative();
                    
                    if aux{
                        en_rango = false;
                        break 'outer1;
                    }
                }
                else{
                    eprint!("ERROR: fn se_puede_poner() Error al intentar sumar\n"); 
                    en_rango = false;
                    break 'outer1;
                }
                let val_x = usize_add(fx, x);
                let val_y = usize_add(fy, y);
                let rang_x = val_x < 10;
                let rang_y = val_y < 20; 
                
                //comprobar rango de la ficha
                if !(rang_x && rang_y){ 
                    en_rango = false;
                    break 'outer1;
                }
                //comprobar si nueva posición libre
                else if tablero.get(val_y as usize, val_x as usize) == 1{
                    posiciones_libres = false;
                    break 'outer1;
                }
            }
            x+=1;
        }
        y+=1;
    } 
    // eprint!("{en_rango} && {posiciones_libres}\n\r\n\r\n\r");
    return en_rango && posiciones_libres;
}

pub fn poner_tetromino_en_tablero(ficha: &TetrominoGame, tablero: &mut Tablero){
    // let forma_geo = ficha.tipo as [[bool:3]: 3];
    
    let fx = ficha.x as isize;
    let fy = ficha.y as isize;
    // let arr = ficha.tipo.get();

    // for y in 0..3 {
    //     for x in 0..3{ 
    //         if arr[y][x] == true {
    //             let row :isize = fy + (y as isize) -1;
    //             let col :isize = fx + (x as isize) -1;

    //             if  row >= 0 && row < 20 &&  col >=0 && col < 10 {
    //                 tablero.set(row as usize, col as usize, 1);
    //             }
    //         } 
    //     }
    // }

    let mut y:isize = -1;
    for filas in ficha.tipo.get(){
        let mut x:isize = -1;
        for casilla in filas{  
            //si parte ficha en rango
            let row :isize = fy + (y as isize);
            let col :isize = fx + (x as isize);
            if *casilla && row >= 0 && row < 20 &&  col >=0 && col < 10{
                tablero.set(row as usize, col as usize, 1);
            }
            x +=1 ;
        }
        y += 1;
    }
}

//ideas a futuro: tabla como matriz de bits
pub fn validar_filas(pos: usize, tablero: &mut Tablero){ 

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

pub fn repeler (ficha: &mut TetrominoGame, tablero: & Tablero){
    
    let mut poner = false;
    // eprint!("poner {poner}\n");
    let mut i = -1;
    while !poner {

        let mut aux_ficha = ficha.clone() ;
        i+=1;
        match i {
            0 => (),
            1 => aux_ficha.sub_col(),
            2 => aux_ficha.add_col(),
            3 => aux_ficha.sub_row(),
            4 => {
                aux_ficha.add_col();
                aux_ficha.sub_row();
            },
            _ => {
                aux_ficha.sub_col();
                aux_ficha.sub_row();
            }, 
        };
        poner = se_puede_poner(&aux_ficha, tablero);
        
    }

    match i {
        0 => (),
        1 => ficha.x -=1,
        2 => ficha.x +=1,
        3 => ficha.y -=1,
        4 => {
            ficha.x +=1;
            ficha.y -=1;
        },
        _ => {
            ficha.x -=1;
            ficha.y -=1;
        },  
    };
    eprint!("{i}");

}


// let fy = ficha.y as isize;
// let fx = ficha.y as isize;

// // let mover
// let mut ocupado = false;

// //comenzamos en la ultima fila
// let mut y:isize = 1;
// 'outer :for filas in ficha.tipo.get(){
//     let mut x:isize = 1;
//     for casilla in filas{ 
//         let val_rango_y = (fy - y) >= 0 && (fy - y)< 20;
//         let val_rango_x = (fx - x) < 10 && (fx - x) >= 0;
//         //si parte ficha en rango
//         if *casilla && val_rango_x && val_rango_y{
//             //
//             if tablero.get(y as usize, x as usize) == 1 {
//                 ocupado = true;
//                 break 'outer;
//             }
//         }
//         x -=1 ;
//     }
//     y -= 1;
// }