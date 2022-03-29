#[derive(Debug, Default)]
pub struct Battleship {
    const kSize = 10;
    let mut your_board = vec![Vec::new(); kSize];
    let mut enemy_board = vec![Vec::new(); kSize];
}

#[derive(Debug)]
pub struct ShipPieces {
    pub error_type : HangmanErrorKind,
    pub error_msg : String,
    pub user_input: String
}

#[derive(Debug, PartialEq)]
pub struct ShipPieces {
    CARRIER, // size 5
    BATTLESHIP, // size 4
    CRUISE, // size 3
    SUBMARINE // size 2
}

// Run Function - Raul
// 1. Starts The game

// Print Board Function - Raul
//  1. Just Prints the boards to the terminal

// Attack Function - Joseph
//  1. Guessing right prints "Hit" and Node at your enemy's board will be updated
// 2. Guessing wrong prints "Miss" and Node at your enemy's board will be updated

// Place Function (battleship piece generic (ex: ShipPieces::Carrier) - Anh
//    1. Make it so that you are forced to keep placing until you have used all of your pieces
//    2. Make it so that you can't attack until you have placed all of your pieces
//    3. Once one piece has been placed, the terminal will print what pieces are left to be made