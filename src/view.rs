use rand::Rng;

static O_0: [[bool; 3]; 3] = [[false, false, false],
                            [false, true, true],
                            [false, true, true]];
static O_1: [[bool; 3]; 3] = [[false, false, false],
                            [true, true, false],
                            [true, true, false]];
static O_2: [[bool; 3]; 3] = [[true, true, false],
                            [true, true, false],
                            [false, false, false]];
static O_3: [[bool; 3]; 3] = [[false, true, true],
                            [false, true, true],
                            [false, false, false]];
                            
static O_ARR: [[[bool; 3]; 3]; 4] = [O_0, O_1, O_2, O_3];

static S_0: [[bool; 3]; 3] = [[false, true, true],
                              [true, true, false],
                              [false, false, false]];

static S_1: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, true],
                              [false, false, true]];

static S_2: [[bool; 3]; 3] = [[false, false, false],
                              [false, true, true],
                              [true, true, false]];

static S_3: [[bool; 3]; 3] = [[true, false, false],
                            [true, true, false],
                            [false, true, false]];
                            
static S_ARR: [[[bool; 3]; 3]; 4] = [S_0, S_1, S_2, S_3];

static Z_0: [[bool; 3]; 3] = [[true, true, false],
                              [false, true, true],
                              [false, false, false]];

static Z_1: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, true],
                              [false, false, true]];

static Z_2: [[bool; 3]; 3] = [[false, false, false],
                              [false, true, true],
                              [true, true, false]];

static Z_3: [[bool; 3]; 3] = [[true, false, false],
                            [true, true, false],
                            [false, true, false]];
                            
static Z_ARR: [[[bool; 3]; 3]; 4] = [Z_0, Z_1, Z_2, Z_3];

static J_0: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, false],
                              [true, true, false]];

static J_1: [[bool; 3]; 3] = [[true, false, false],
                              [true, true, true],
                              [false, false, false]];

static J_2: [[bool; 3]; 3] = [[false, true, true],
                              [false, true, false],
                              [false, true, false]];

static J_3: [[bool; 3]; 3] = [[false, false, false],
                            [true, true, true],
                            [false, false, true]];
                            
static J_ARR: [[[bool; 3]; 3]; 4] = [J_0, J_1, J_2, J_3];

static L_0: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, false],
                              [false, true, true]];

static L_1: [[bool; 3]; 3] = [[false, false, false],
                              [true, true, true],
                              [true, false, false]];

static L_2: [[bool; 3]; 3] = [[true, true, false],
                              [false, true, false],
                              [false, true, false]];

static L_3: [[bool; 3]; 3] = [[false, false, true],
                            [true, true, true],
                            [false, false, false]];
                            
static L_ARR: [[[bool; 3]; 3]; 4] = [L_0, L_1, L_2, L_3];


static T_0: [[bool; 3]; 3] = [[true, true, true],
                              [false, true, false],
                              [false, true, false]];

static T_1: [[bool; 3]; 3] = [[false, false, true],
                              [true, true, true],
                              [false, false, true]];

static T_2: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, false],
                              [true, true, true]];

static T_3: [[bool; 3]; 3] = [[true, false, false],
                            [true, true, true],
                            [true, false, false]];
                            
static T_ARR: [[[bool; 3]; 3]; 4] = [T_0, T_1, T_2, T_3];


static I_0: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, false],
                              [false, true, false]];

static I_1: [[bool; 3]; 3] = [[false, false, false],
                              [true, true, true],
                              [false, false, false]];

static I_2: [[bool; 3]; 3] = [[false, true, false],
                              [false, true, false],
                              [false, true, false]];

static I_3: [[bool; 3]; 3] = [[false, false, false],
                            [true, true, true],
                            [false, false, false]];
                            
static I_ARR: [[[bool; 3]; 3]; 4] = [I_0, I_1, I_2, I_3];


pub struct TetrominoGame {
    pub x: usize,
    pub y: usize,
    actual: usize,
    pub tipo : [TetrominoShape; 7],
}

impl TetrominoGame{
    pub fn new () -> TetrominoGame{
        TetrominoGame{
            x: 5, 
            y: 0,
            actual: 0,
            tipo: Tetromino::new()
        }
    } 

    pub fn next (&mut self) {
        self.actual = (self.actual +1 )%7;
    }
}

pub struct Tetromino{
    rotacion: usize,
    tipo : [TetrominoShape; 7]
}

//TODO: crear pool

impl Tetromino{
    pub fn new() -> Tetromino{
        Tetromino{
            rotacion: 0,
            tipo: [TetrominoShape::O,
            TetrominoShape::I,
            TetrominoShape::S,
            TetrominoShape::Z,
            TetrominoShape::T,
            TetrominoShape::J,
            TetrominoShape::L]
        }
    }

    //sentido del reloj
    pub fn rotar(&mut self){
        self.rotacion = (self.rotacion +1)%4; 
    }

    //devuelve tetromino
    pub fn get(&self) -> &'static [[bool;3];3]{
        match self.tipo[] {
            TetrominoShape::O => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::I => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::S => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::Z => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::T => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::J => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::L => {
                &O_ARR[self.rotacion]
            } 
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