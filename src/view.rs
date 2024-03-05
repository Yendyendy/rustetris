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

pub mod view {
    pub struct Ficha {
        pub x: u8,
        pub y: u8
    }
    
    impl Ficha{
        pub fn new () -> Ficha{
            Ficha{
                x: 10, 
                y: 0
            }
        }
    }
    
    // // //rataui
    // // //wgpu
    // // //tetris rotatiosn: super rotation system - wall kicis
    
    pub struct Matrix {
        pub rows: [[u8; 10]; 20],
    }
    
    impl Matrix {
        // pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        //     Some(*self.rows.get(row)?.get(col)?)
        // }
    
        pub fn get (&self, row: usize, col: usize) -> u8{
            self.rows[row][col]
        }
    
        pub fn set (&mut self, row: usize, col: usize, val: u8) {
            self.rows[row][col] = val;
        }
    
        pub fn new () -> Matrix{
            Matrix{
                rows: [[0; 10]; 20]
            }
        }
    }

}
