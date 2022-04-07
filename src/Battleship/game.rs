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
    // Player Place function, arguments: ship_type
    fn Player_Place_1_ship(ship_type : ShipPieces) ->() {
        let tuple_len_name : (i32,String) = match  ship_type {
            ShipPieces::Carrier => (5, "Carrier"),
            ShipPieces::Battleship => (4,"Battleship"),
            ShipPieces::Cruise =>(3,"Cruise"),
            ShipPieces::Submarine => (2,"Submarine"),
        };
        let mut start_pos: (usize, usize) = (0,0);
        let mut end_pos :(usize,usize) = (0,0);
        loop { // loop until a success start position
        //asking for input
            println!("Which row do you want your {name} of size {len} to start?" ,name = tuple_len_name[1],len = tuple_len_name[0]);
            let input : String = Input::new()
                .with_prompt(">")
                .interact_text()?;

            // Split the command line input by spaces
            let args : Vec<&str> = input.trim().split(' ').collect();

            // If there are 0 arguments, return an error
            if args.len() == 0 {
                println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                continue;
            }
            let row_start = match args[0] {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                "4" => 3,
                "5" => 4,
                "6" => 5,
                "7" => 6,
                "8" => 7,
                "9" => 8,
                _ => {  println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                        continue; }
            };
            //asking for col
            println!("Which col do you want your {name} of size {len} to start?" ,name = tuple_len_name[1],len = tuple_len_name[0]);
            let input : String = Input::new()
                .with_prompt(">")
                .interact_text()?;

            // Split the command line input by spaces
            let args : Vec<&str> = input.trim().split(' ').collect();

            // If there are 0 arguments, return an error
            if args.len() == 0 {
                println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                continue;
            }
            let col_start = match args[0] {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                "4" => 3,
                "5" => 4,
                "6" => 5,
                "7" => 6,
                "8" => 7,
                "9" => 8,
                _ => {  println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                continue; }
            };
            if (Battleship::your_board[row_start][col_start].empty == true ) {
                start_pos.0 = row_start;
                start_pos.1 = col_start;
                break;
            } else {
                println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                continue;
            }
            // Check for possible ways to put endpoint
            let mut all_posible_end : Vec<(usize,usize)> = Vec::new();
            let len : usize = tuple_len_name.0;
            let row_start = start_pos.0;
            let col_start = start_pos.1;
            // Handle up first
            if (start_pos.0 >= len - 1) {
                for i in 0..len-1 {
                    if (Battleship::your_board[row_start - i][col_start].empty == false) {
                        continue;
                    }
                }
                all_possible_end.push((row_start - len + 1,col_start));
            }
            // Handle down
            if (row_start + len <= kSize  - 1) {
                for i in 0..len-1 {
                    if (Battleship::your_board[row_start + i][col_start].empty == false) {
                        continue;
                    }
                }
                all_possible_end.push((row_start + len ,col_start));
            }
            // Handle left 
            if (col_start >= len - 1) {
                for i in 0..len-1 {
                    if (Battleship::your_board[row_start][col_start - i].empty == false) {
                        continue;
                    }
                }
                all_possible_end.push((row_start,col_start-len+1));
            }
            // Handle right
            if (col_start + len <= kSize  - 1) {
                for i in 0..len-1 {
                    if (Battleship::your_board[row_start ][col_start+i].empty == false) {
                        continue;
                    }
                }
                all_possible_end.push((row_start , len+ col_start));
            }
            for i in 0..all_possible_end.size() {
                println!("{no} - ({row},{col})", 
                    no = i + 1,
                    row = all_possible_end[i].0,
                    col = all_possible_end[i].1,)
            }
            loop { // loop until get endpoint
                //asking for input
                println!("Which position do you want your {name} of size {len} to end?" ,name = tuple_len_name[1],len = tuple_len_name[0]);
                let input : String = Input::new()
                    .with_prompt(">")
                    .interact_text()?;
        
                // Split the command line input by spaces
                let args : Vec<&str> = input.trim().split(' ').collect();
    
                // If there are 0 arguments, return an error
                if args.len() == 0 {
                    println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                    continue;
                }
                if (args[0] >= all_possible_end.size() || args[0] < 0) {
                    println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                    continue;
                } 
                let end_choice:usize = match args[0] {
                    "1" => 0,
                    "2" => 1,
                    "3" => 2,
                    "4" => 3,
                };
                end_pos = all_possible_end[end_choice];
                break;
            } /// Finished getting start_pos and end_pos
        }
        if (start_pos.0 == end_pos.0) {
            if (start_pos.1 < end_pos.1) {
                for i in start_pos.1..end_pos.1 {
                    Battleship::your_board[start_pos.0][i].empty = false;
                }
            } else {
                for i in end_pos.1..start_pos.1 {
                    Battleship::your_board[start_pos.0][i].empty = false;
                }
            }
        } else {
            if (start_pos.0 < end_pos.0) {
                for i in start_pos.0..end_pos.0 {
                    Battleship::your_board[i][start_pos.1].empty = false;
                }
            } else {
                for i in end_pos.0..start_pos.0 {
                    Battleship::your_board[i][start_pos.1].empty = false;
                }
            }
        }
        // Finished placing
    } 
    fn Player_Place() {
        for row in 0..kSize-1 {
            for col in 0..kSize-1 {
                if (Battleship::your_board[row][col].empty == false || Battleship::your_board[row][col].guess == true) {
                    panic!("Place in a already worked on board");
                }
            }
        }
        for i in 0..kNo5 - 1 {
            Player_Place_1_ship(Battleship::Carrier);
        }
        for i in 0..kNo4 -1 {
            Player_Place_1_ship(Battleship::Battleship);
        }
        for i in 0..kNo3-1 {
            Player_Place_1_ship(Battleship::Cruise);
        }
        for i in 0..kNo2-1 {
            Player_Place_1_ship(Battleship::Submarine);
        }
       print!("All ships being placed, ready to game!")
    }  
}