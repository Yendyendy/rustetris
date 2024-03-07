// --------------------

// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   | 
// |                   |  
// --------------------

// [(-1,-1), (-1, 0), ( 1,-1)],
// [(-1, 0), ( 0, 0), ( 1, 0)],
// [(-1, 1), (0 ,-1), ( 1, 1)]

pub struct Ficha {
    pub x: usize,
    pub y: usize,
    pub tipo : Tficha,
}

impl Ficha{
    pub fn new () -> Ficha{
        Ficha{
            x: 5, 
            y: 0,
            tipo: Tficha::new_o()
        }
    }
}

pub enum Tficha  {
    O([[bool; 3]; 3]),
    I([[bool; 3]; 3]),
    S([[bool; 3]; 3]),
    Z([[bool; 3]; 3]),
    T([[bool; 3]; 3]),
    J([[bool; 3]; 3]),
    L([[bool; 3]; 3])
}

impl Tficha{
    pub fn new_o () -> Tficha{
        Self::O([[false, false, false],
                 [false, true, true],
                 [false, true, true]])
    }
    pub fn get (&self, row: usize, col: usize) -> i8{
        let Tficha::O(arr) self::Ficha;
    }
    
    pub fn set (&mut self, row: usize, col: usize, val: i8) {
        self.[row][col] = val;
    } 
}

impl Tficha {
    // pub fn get(&self, row: usize, col: usize) -> Option<i8> {
    //     Some(*self.rows.get(row)?.get(col)?)
    // }

} 

// // //rataui
// // //wgpu
// // //tetris rotatiosn: super rotation system - wall kicis

pub struct Tablero {
    pub rows: [[i8; 10]; 20],
}

impl Tablero {
    // pub fn get(&self, row: usize, col: usize) -> Option<i8> {
    //     Some(*self.rows.get(row)?.get(col)?)
    // }

    pub fn get (&self, row: usize, col: usize) -> i8{
        self.rows[row][col]
    }

    pub fn set (&mut self, row: usize, col: usize, val: i8) {
        self.rows[row][col] = val;
    }

    pub fn new () -> Tablero{
        Tablero{
            rows: [[0; 10]; 20]
        }
    }
}