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


pub struct TetrominoGame {
    pub x: usize,
    pub y: usize,
    pub tipo : Tetromino,
}

impl TetrominoGame{
    pub fn new () -> TetrominoGame{
        TetrominoGame{
            x: 5, 
            y: 0,
            tipo: Tetromino::new_o()
        }
    } 
}

struct Tetromino{
    rotacion: usize,
    tipo : TetrominoShape
}

//TODO: crear pool

impl Tetromino{
    pub fn new() -> Tetromino{
        Tetromino{
            rotacion: 0,
            tipo: TetrominoShape::O
        }
    }

    //sentido del reloj
    pub fn rotar(&self) -> &'static [[bool;3];3]{
        let rotacion = (self.rotacion +1)%4;
        self.get()
    }

    //devuelve tetromino
    pub fn get(&self) -> &'static [[bool;3];3]{
        match self.tipo {
            TetrominoShape::O => {&O}
            _ => {&S}
        }
    }
}

pub enum TetrominoShape  {
    O,
    I,
    S,
    Z,
    T,
    J,
    L
}

// impl TetrominoShape{
//     pub fn new_o () -> TetrominoShape{
//         TetrominoShape::O
//     } 

//     pub fn new_s () -> TetrominoShape{
//         TetrominoShape::S
//     } 

//     pub fn get_forma (&self) -> &'static [[bool; 3];3]{
//         match self {
//             Self::O => &O,
//             Self::S => &S,
//             Self::Z => &Z,
//             _ => &O
//         }
//     }

//     pub fn new_rand() -> TetrominoShape {
//         // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
//         let rnd = rand::thread_rng().gen_range(0..=2);
//         match rnd { // rand 0.8
//             0 => TetrominoShape::O,
//             1 => TetrominoShape::S,
//             _ => TetrominoShape::Z,
//         }
//     }
// }




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