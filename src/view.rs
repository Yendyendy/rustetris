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

pub struct TetrominoPool{
    pool: [TetrominoGame; 7],
    actual: usize
}

impl TetrominoPool{

    pub fn next (&mut self){
        self.current().x = 5;
        self.current().y = 0;
        self.actual = (self.actual +1 )%7;
    } 
    pub fn current (&mut self) -> &mut TetrominoGame{ 
        &mut self.pool[self.actual]
    } 

    //TODO: crear pool
    //como crear el pool,
    pub fn new () -> TetrominoPool{
        TetrominoPool{
            actual : 0,
            pool : TetrominoPool::pool()
        }
    }

    pub fn pool () -> [TetrominoGame; 7] {
        [TetrominoGame::new_tetromino(TetrominoShape::O),
        TetrominoGame::new_tetromino(TetrominoShape::I),
        TetrominoGame::new_tetromino(TetrominoShape::S),
        TetrominoGame::new_tetromino(TetrominoShape::Z),
        TetrominoGame::new_tetromino(TetrominoShape::T),
        TetrominoGame::new_tetromino(TetrominoShape::J),
        TetrominoGame::new_tetromino(TetrominoShape::L)]
    }
}
pub struct TetrominoGame {
    pub x: usize,
    pub y: usize,
    pub actual: usize,
    pub tipo : Tetromino,
}

impl TetrominoGame{
    fn new_tetromino(shape: TetrominoShape) -> TetrominoGame{
        TetrominoGame{
            x: 5, 
            y: 0,
            actual: 0,
            tipo: Tetromino::new(shape)
        }
    }

    pub fn add_col(&mut self){
        self.x +=1;
    }

    pub fn sub_col(&mut self){
        self.x -=1; 
    }

    pub fn add_row(&mut self){
        self.y +=1;
    }

    pub fn sub_row(&mut self){
        self.y -=1;
    }   

    pub fn rotar(&mut self){
        self.tipo.rotar()
    }  


}

pub struct Tetromino{
    rotacion: usize,
    tipo : TetrominoShape
}


impl Tetromino{
    pub fn new(shape: TetrominoShape) -> Tetromino{
        match shape{
            TetrominoShape::O => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::O
                }
            }
            TetrominoShape::I => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::I
                }
            }
            TetrominoShape::S => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::S
                }
            }
            TetrominoShape::Z => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::Z
                }
            }
            TetrominoShape::T => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::T
                }
            }
            TetrominoShape::J => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::J
                }
            }
            TetrominoShape::L => {
                Tetromino{
                    rotacion: 0,
                    tipo : TetrominoShape::L
                }
            } 
        }
    }

    //sentido del reloj
    pub fn rotar(&mut self){
        self.rotacion = (self.rotacion +1)%4; 
    }

    //devuelve tetromino
    pub fn get(&self) -> &'static [[bool;3];3]{
        match self.tipo {
            TetrominoShape::O => {
                &O_ARR[self.rotacion]
            }
            TetrominoShape::I => {
                &I_ARR[self.rotacion]
            }
            TetrominoShape::S => {
                &S_ARR[self.rotacion]
            }
            TetrominoShape::Z => {
                &Z_ARR[self.rotacion]
            }
            TetrominoShape::T => {
                &T_ARR[self.rotacion]
            }
            TetrominoShape::J => {
                &J_ARR[self.rotacion]
            }
            TetrominoShape::L => {
                &L_ARR[self.rotacion]
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