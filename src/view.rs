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
use rand::Rng;

static O: [[bool; 3]; 3] = [[false, false, false],
                            [false, true, true],
                            [false, true, true]];

static Z: [[bool; 3]; 3] = [[false, false, false],
                            [true, true, false],
                            [false, true, true]];

static S: [[bool; 3]; 3] = [[false, false, false],
                            [false, true, true],
                            [true, true, false]];


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
            tipo: Tficha::new_rand()
        }
    } 
}

pub enum Tficha  {
    O,
    I,
    S,
    Z,
    T,
    J,
    L
}

impl Tficha{
    pub fn new_o () -> Tficha{
        Tficha::O
    } 

    pub fn get_forma (&self) -> &'static [[bool; 3];3]{
        match self {
            Self::O => &O,
            Self::S => &S,
            Self::Z => &Z,
            _ => &O
        }
    }

    pub fn new_rand() -> Tficha {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        let rnd = rand::thread_rng().gen_range(0..=2);
        match rnd { // rand 0.8
            0 => Tficha::O,
            1 => Tficha::S,
            _ => Tficha::Z,
        }
    }
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