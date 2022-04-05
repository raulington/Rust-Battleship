#[derive(Debug, Default)]
pub struct Battleship {
    // const kSize = 10;
    pub your_board: Vec<Vec<Node>>,
    pub enemy_board: Vec<Vec<Node>>,
}
pub const kSize: usize = 10;
pub const kNo5 : usize= 1; // Number of Carrier
pub const kNo4: usize = 1; // Number of Battleship
pub const kNo3: usize = 1; // Number of Cruise
pub const kNo2: usize = 1; // Number of Submarine


#[derive(Debug)]
pub struct ShipPiecesError {
    pub error_type : HangmanErrorKind,
    pub error_msg : String,
    pub user_input: String
}

#[derive(Debug, PartialEq)]
pub enum ShipPieces {
    Carrier, // size 5
    Battleship, // size 4
    Cruise, // size 3
    Submarine // size 2
}

#[derive(Debug)]
pub struct Node {
    guess: bool,
    empty: bool,
}

impl Battleship {
    pub fn new() -> Result<Self, ShipPiecesError> {
        let mut your_board = vec![Vec::new(); kSize];
        let mut enemy_board = vec![Vec::new(); kSize];
        // Can spawn thread to do this faster if needed
        for vec in your_board {
            for i in 0..10 {            
                let mut n = Node {
                    guess: false,
                    empty : true,
                };
                vec.push(n);
            }
        }
        for vec in enemy_board {
            for i in 0..10 {            
                let mut n = Node {
                    guess: false,
                    empty : true,
                };
                vec.push(n);
            }
        }
        let battleship = Battleship{your_board, enemy_board};
        return Ok(battleship);
    }

    // Returns X is guess is true and empty is false, O is guess is false
    pub fn drawyourboard(i: i32) -> char {
        Node n = your_board.get(i);
        if (n.guess == true && empty == false) {
            return 'X';
        }
        return 'O';
    } 

    pub fn drawenemyboard(i: i32) -> char {
        Node n = enemy_board.get(i);
        if (n.guess == true && empty == false) {
            return 'X';
        }
        return 'O';
    }

    // Attack Function - Joseph
    //  1. Guessing right prints "Hit" and Node at your enemy's board will be updated
    // 2. Guessing wrong prints "Miss" and Node at your enemy's board will be updated

    // Place Function (battleship piece generic (ex: ShipPieces::Carrier) - Anh
    //    1. Make it so that you are forced to keep placing until you have used all of your pieces
    //    2. Make it so that you can't attack until you have placed all of your pieces
    //    3. Once one piece has been placed, the terminal will print what pieces are left to be made
}