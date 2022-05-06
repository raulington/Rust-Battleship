use dialoguer::Input;
use super::error::*;
use rand::Rng;

#[derive(Debug, Default)]
pub struct Battleship {
    // const kSize = 10;
    pub your_board: Vec<Vec<Node>>,
    pub enemy_board: Vec<Vec<Node>>,
    pub easy_or_not: bool,
}
pub const K_SIZE: usize = 10;
pub const K_NO5 : usize= 1; // Number of Carrier
pub const K_NO4: usize = 1; // Number of Battleship
pub const K_NO3: usize = 1; // Number of Cruiser
pub const K_NO2: usize = 1; // Number of Submarine

#[derive(Debug)]
pub struct ShipPiecesError {
    pub error_type : String,
    pub error_msg : String,
    pub user_input: String
}

/// Display implementation for BattleshipError for user output in the CLI.
impl std::fmt::Display for ShipPiecesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{} --- (user_input : {})\n{}",
            self.error_type, self.user_input, self.error_msg)
    } 
}

#[derive(Debug, PartialEq)]
pub enum ShipPieces {
    Carrier, // size 5
    Battleship, // size 4
    Cruiser, // size 3
    Submarine // size 2
}

#[derive(Debug,Clone)] // maybe include Copy in the list if needed
pub struct Node {
    guess: bool,
    empty: bool,
    ship: String,
}

impl Battleship {
    pub fn new() -> Result<Self, ShipPiecesError> {
        let mut your_board = vec![Vec::new(); K_SIZE];
        let mut enemy_board = vec![Vec::new(); K_SIZE];
        // Can spawn thread to do this faster if needed
        for vec in your_board.iter_mut() {
            for _i in 0..10 {            
                let n = Node {
                    guess: false,
                    empty : true,
                    ship : "none".to_string(),
                };
                vec.push(n);
            }
        }
        for vec in enemy_board.iter_mut() {
            for _i in 0..10 {            
                let n = Node {
                    guess: false,
                    empty : true,
                    ship : "none".to_string(),
                };
                vec.push(n);
            }
        }
        let battleship = Battleship{
            your_board: your_board.clone(), 
            enemy_board: enemy_board.clone(),
            easy_or_not: true};
        return Ok(battleship);
    }

    // Returns X is guess is true and empty is false, O is guess is false
    pub fn drawyourboard(&self, i: usize, j: usize) -> char {
        let n = self.your_board.get(i).unwrap().get(j).unwrap();
        if n.empty == false && n.guess == false {
            return '⬛';
        } else if n.empty == false && n.guess == true {
            return '❌';
        } else if n.empty == true && n.guess == true {
            return '🔘';
        }
        return '🌊';
        // Usign color to make it looks better
    } 

    pub fn drawenemyboard(&self, i: usize, j: usize) -> char {
        let n = self.enemy_board.get(i).unwrap().get(j).unwrap();
        if n.guess == true && n.empty == false {
            return '❌';
        } else if n.guess == true && n.empty == true {
            return '🔘';
        }
        return '🌊';
    }



    

    // Attack Function - Joseph
    //  1. Guessing right prints "Hit" and Node at your enemy's board will be updated
    // 2. Guessing wrong prints "Miss" and Node at your enemy's board will be updated
    pub fn attack(&mut self, player_board : bool, coordinates : (i32, i32)) -> i32 {
        let row = coordinates.0 as usize;
        let col = coordinates.1 as usize;

        // println!("{}", self.enemy_board[row][col].ship.clone());

        if player_board == true {
            
            if self.your_board[row][col].guess == true { //if spot was already guessed
                return 0;
            } 
            let currently_sunk = self.is_sunk(player_board, self.your_board[row][col].ship.clone());
            self.your_board[row][col].guess = true;
            if self.your_board[row][col].empty == true { //no ship was on that spot
                println!("The CPU missed");
                return 1;
            } // if a ship was on that spot...
            
            println!("The CPU hit your {}!", self.your_board[row][col].ship);
            if self.is_sunk(player_board, self.your_board[row][col].ship.clone()) && currently_sunk == false {
                println!("Your {} got sunk!", self.your_board[row][col].ship.clone());
            }
        } else if player_board == false {
            if self.enemy_board[row][col].guess == true { //if spot was already guessed
                println!("That spot was already guessed, '!attack ___' again");
                return 0;
            } 
            let currently_sunk = self.is_sunk(player_board, self.enemy_board[row][col].ship.clone());
            self.enemy_board[row][col].guess = true;
            if self.enemy_board[row][col].empty == true { //no ship was on that spot
                println!("Your hit missed");
                return 1;
            }

            println!("Your hit was successful!");
            if self.is_sunk(player_board, self.enemy_board[row][col].ship.clone()) && currently_sunk == false {
                println!("You sunk your opponent's {}!!!", self.enemy_board[row][col].ship.clone());
            }
        }
        
        return 2;
    }

    pub fn is_sunk(&mut self, player_board : bool, ship: String) -> bool {
        let mut count_left = 0;
        if (ship == "Carrier".to_string()) {
            count_left = 5;
        } else if (ship == "Battleship".to_string()) {
            count_left = 4;
        } else if (ship == "Cruiser".to_string()) {
            count_left = 3;
        } else if (ship == "Submarine".to_string()) {
            count_left = 2;
        } else {
            return false;
            println!("NOT FOUND in is_sunk");
        }

        for row in 0..9 {
            for col in 0..9 {
                if player_board {
                    if self.your_board[row][col].ship == ship && self.your_board[row][col].guess == true {
                        count_left = count_left - 1;
                    }
                } else {
                    if self.enemy_board[row][col].ship == ship && self.enemy_board[row][col].guess == true {
                        count_left = count_left - 1;
                    }
                }
            }
        }

        if count_left == 0 {
            return true;
        }
        return false;
    }

    pub fn random_attack(&mut self) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let mut a = rng.gen_range(0..10);
        let mut b = rng.gen_range(0..10);
        while (self.your_board[a][b].guess == true) {
            a = rng.gen_range(0..10);
            b = rng.gen_range(0..10);
        }
        return (a as i32,b as i32);
    }

    pub fn unsunk_ships(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.your_board[row][col].guess == true && self.your_board[row][col].ship != "none".to_string() {
                    if self.is_sunk(true, self.your_board[row][col].ship.clone()) == false {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn hit_is_alone(&mut self, coordinates: (i32, i32)) -> bool {
        let row = coordinates.0 as usize;
        let col = coordinates.1 as usize;
        if (row > 0) {
            if self.your_board[row - 1][col].guess && self.your_board[row-1][col].ship != "none".to_string() {
                return false;
            }
        }
        if (row < 9) {
            if self.your_board[row + 1][col].guess && self.your_board[row+1][col].ship != "none".to_string() {
                return false;
            }
        }
        if (col > 0) {
            if self.your_board[row][col - 1].guess && self.your_board[row][col-1].ship != "none".to_string() {
                return false;
            }
        }
        if (col < 9) {
            if self.your_board[row][col - 1].guess && self.your_board[row][col+1].ship != "none".to_string() {
                return false;
            }
        }
        return true;
    }

    pub fn surround_options(&mut self, coordinates: (i32, i32), option: i32) -> (i32, i32) {
        let row = coordinates.0 as usize;
        let col = coordinates.1 as usize;
        let row_ = coordinates.0;
        let col_ = coordinates.1;

        if option == 1 {
            if (row > 0) {
                if self.your_board[row - 1][col].guess == false {
                    println!("option 1 was chosen");
                    return (row_-1, col_);
                }
            }
            return ((-1, -1));
        } else if option == 2 {
            if (row < 9) {
                if self.your_board[row + 1][col].guess == false {
                    println!("option 2 was chosen");
                    return (row_+1, col_);
                }
            }
            return ((-1, -1));
        } else if option == 3 {
            if (col > 0) {
                if self.your_board[row][col - 1].guess == false {
                    println!("option 3 was chosen");
                    return (row_, col_-1);
                }
            }
            return ((-1, -1));
        } else {
            if (col < 9) {
                if self.your_board[row][col + 1].guess == false {
                    println!("option 4 was chosen");
                    return (row_, col_+1);
                }
            }
        }
        return ((-1, -1));
    }


    pub fn surround_alone(&mut self, coordinates: (i32, i32)) -> (i32, i32) {
        let row = coordinates.0 as usize;
        let col = coordinates.1 as usize;
        let row_ = coordinates.0;
        let col_ = coordinates.1;

        let mut rng = rand::thread_rng();
        let mut a = rng.gen_range(0..5);

        let mut coords = self.surround_options(coordinates, a as i32);

        while coords == (-1, -1) {
            a = rng.gen_range(0..5);
            coords = self.surround_options(coordinates, a as i32);
        }



        return coords;
    }

    pub fn coordinate_of_interest(&mut self) -> (i32, i32) {
        for row in 0..9 {
            for col in 0..9 {
                if self.your_board[row][col].guess == true && self.your_board[row][col].ship != "none".to_string() {
                    if self.is_sunk(true, self.your_board[row][col].ship.clone()) == false {
                        return ((row as i32, col as i32));
                    }
                }
            }
        }
        println!("Error in Coord of Interest");
        return ((-1, -1));
    }

    pub fn ship_is_vertical(&mut self, coordinates: (i32, i32)) -> bool {
        let row = coordinates.0 as usize;
        let col = coordinates.1 as usize;
        if self.your_board[row+1][col].ship.clone() == self.your_board[row][col].ship.clone() {
            return true;
        }
        return false;
    }

    pub fn next_ship_coords(&mut self, ship: String) -> (i32, i32) {
        for row in 0..9 {
            for col in 0..9 {
                if self.your_board[row][col].ship == ship && self.your_board[row][col].guess == false {
                    return ((row as i32, col as i32));
                }
            }
        }
        println!("Error in next ship coords()");
        return (-1, -1);
    }

    pub fn continue_attack(&mut self, coords: (i32, i32)) -> (i32, i32) {
        let row = coords.0 as usize;
        let col = coords.1 as usize;
        if self.ship_is_vertical(coords) {
            if row > 0 {
                if self.your_board[row-1][col].guess == false {
                    return ((coords.0 - 1, coords.1));
                } 
            }
            return self.next_ship_coords(self.your_board[row][col].ship.clone());
        } // if ship is horizontal...
        if col > 0 {
            if self.your_board[row][col-1].guess == false {
                return ((coords.0, coords.1 - 1));
            } 
        }
        return self.next_ship_coords(self.your_board[row][col].ship.clone());
    }

    pub fn difficult_attack(&mut self) -> (i32, i32) {
        if self.unsunk_ships() == true { // if a ship was hit but not sunk yet
            let coords = self.coordinate_of_interest();
            if self.hit_is_alone(coords) { // if the hit is alone
                return self.surround_alone(coords); // scope around it to find next piece
            }
            return self.continue_attack(coords);
        }
        return self.random_attack();
    }

    pub fn cpu_attack(&mut self) -> (i32, i32) {
        if self.easy_or_not == true {
            return self.random_attack();
        }
        return self.difficult_attack(); // change later when other ai is added. 
    }

    

    // Place Function (battleship piece generic (ex: ShipPieces::Carrier)
    //    1. Make it so that you are forced to keep placing until you have used all of your pieces
    //    2. Make it so that you can't attack until you have placed all of your pieces
    //    3. Once one piece has been placed, the terminal will print what pieces are left to be made
    // Player Place function, arguments: ship_type
    pub fn player_place_1_ship(&mut self,ship_type : ShipPieces) ->() {

        let tuple_len_name : (usize,String) = match  ship_type {
            ShipPieces::Carrier => (5, "Carrier".to_string()),
            ShipPieces::Battleship => (4,"Battleship".to_string()),
            ShipPieces::Cruiser =>(3,"Cruiser".to_string()),
            ShipPieces::Submarine => (2,"Submarine".to_string()),
        };
        let mut start_pos: (usize, usize) = (0,0);
        let mut end_pos :(usize,usize) = (0,0);
        let mut done: bool = false;
        while !done { // loop until a success start position
        //asking for input
            println!("Which row do you want your {name} of size {len} to start?" ,name = tuple_len_name.1,len = tuple_len_name.0);
            let input : String = Input::new()
                .with_prompt(">")
                .interact_text().unwrap();

            // Split the command line input by spaces
            let args : Vec<&str> = input.trim().split(' ').collect();

            // If there are 0 arguments, return an error
            if args.len() == 0 {
                println!("Bad args");
                continue;
            }
            let row_start = match args[0] {
                "0" => 0,
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {  println!("Bad args");
                        continue; }
            };
            //asking for col
            println!("Which col do you want your {name} of size {len} to start?" ,name = tuple_len_name.1,len = tuple_len_name.0);
            let input : String = Input::new()
                .with_prompt(">")
                .interact_text().unwrap();

            // Split the command line input by spaces
            let args : Vec<&str> = input.trim().split(' ').collect();

            // If there are 0 arguments, return an error
            if args.len() == 0 {
                println!("Bad args");
                continue;
            }
            let col_start = match args[0] {
                "0" => 0,
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {  println!("Bad args");
                continue; }
            };
            if self.your_board[row_start][col_start].empty == true {
                start_pos.0 = row_start;
                start_pos.1 = col_start;
            } else {
                println!("Bad args");
                continue;
            }
            // Check for possible ways to put endpoint
            let mut all_possible_end : Vec<(usize,usize)> = Vec::new();
            let len : usize = tuple_len_name.0;
            let row_start = start_pos.0;
            let col_start = start_pos.1;
            // Handle up first
            if start_pos.0 >= len - 1 {
                let mut a = true;
                for i in 0..=len-1 {
                    if self.your_board[row_start - i][col_start].empty == false {
                        a = false;
                    }
                }
                if a {
                    all_possible_end.push((row_start +1 - len ,col_start));
                }

            }
            // Handle down
            if row_start + len <= K_SIZE  {
                let mut a:  bool = true;
                for i in 0..=len-1 {
                    if self.your_board[row_start + i][col_start].empty == false {
                        a = false;
                    }
                }
                if a {
                    all_possible_end.push((row_start + len-1 ,col_start));
                }
            }
            // Handle left 
            if col_start >= len - 1 {
                let mut a = true;
                for i in 0..=len-1 {
                    if self.your_board[row_start][col_start - i].empty == false {
                        a = false;
                    }
                }
                if a {
                    all_possible_end.push((row_start,col_start+1 - len));
                }
            }
            // Handle right
            if col_start + len <= K_SIZE   {
                let mut a = true;
                for i in 0..=len-1 {
                    if self.your_board[row_start ][col_start+i].empty == false {
                        a = false;
                    }
                }
                if a {
                    all_possible_end.push((row_start , len+ col_start-1));
                }
                
            }
            for i in 0..all_possible_end.len() {
                println!("{no} - ({row},{col})", 
                    no = i + 1,
                    row = all_possible_end[i].0 ,
                    col = all_possible_end[i].1 ,)
            }
            while !done { // loop until get endpoint
                //asking for input
                println!("Which position do you want your {name} of size {len} to end?" ,name = tuple_len_name.1,len = tuple_len_name.0);
                let input : String = Input::new()
                    .with_prompt(">")
                    .interact_text().unwrap();
        
                // Split the command line input by spaces
                let args : Vec<&str> = input.trim().split(' ').collect();
    
                // If there are 0 arguments, return an error
                if args.len() == 0 {
                    println!("Bad args");
                    continue;
                }
                let end_choice:usize = match args[0] {
                    "1" => 0,
                    "2" => 1,
                    "3" => 2,
                    "4" => 3,
                    _ =>panic!("Bad args")
                };
                end_pos = all_possible_end[end_choice];
                done = true;
            } 
            // Finished getting start_pos and end_pos
        }
        if start_pos.0 == end_pos.0 {
            if start_pos.1 < end_pos.1 {
                for _i in start_pos.1..end_pos.1+1 {
                    self.your_board[start_pos.0][_i].ship = tuple_len_name.1.clone();// tuple_len_name.1.clone();
                    self.your_board[start_pos.0][_i].empty = false;
                    
                }
            } else {
                for _i in end_pos.1..start_pos.1+1 {
                    self.your_board[start_pos.0][_i].ship = tuple_len_name.1.clone();// tuple_len_name.1.clone();
                    self.your_board[start_pos.0][_i].empty = false;
                    
                }
            }
        } else {
            if start_pos.0 < end_pos.0 {
                for _i in start_pos.0..end_pos.0+1 {
                    self.your_board[_i][start_pos.1].ship = tuple_len_name.1.clone();// tuple_len_name.1.clone();
                    self.your_board[_i][start_pos.1].empty = false;
                    
                }
            } else {
                for _i in end_pos.0..start_pos.0+1 {
                    self.your_board[_i][start_pos.1].ship = tuple_len_name.1.clone(); //tuple_len_name.1.clone();
                    self.your_board[_i][start_pos.1].empty = false;
                    
                }
            }
        }
        // Finished placing
    } 
    pub fn player_place(&mut self) {
        let mut finished : bool = false;
        while !finished {
            println!("
            Which difficulty do you want ?
            1. Easy
            2. Hard");
            let input : String = Input::new()
                .with_prompt(">")
                .interact_text().unwrap();
        
                // Split the command line input by spaces
            let args : Vec<&str> = input.trim().split(' ').collect();
    
                // If there are 0 arguments, return an error
            if args.len() != 1 || (args[0] != "1" && args[0] != "2") {
                println!("Bad args");
            } else {
                if args[0] == "2" {
                    self.easy_or_not = false;
                }
                finished = true;
            }   
        }
        for row in 0..=K_SIZE-1 {
            for col in 0..=K_SIZE-1 {
                if self.your_board[row][col].empty == false || self.your_board[row][col].guess == true {
                    panic!("Place in an already worked on board");
                }
            }
        }
        for _i in 0..K_NO5 {
            Battleship::player_place_1_ship(self,ShipPieces::Carrier);
        }
        for _i in 0..K_NO4{
            Battleship::player_place_1_ship(self,ShipPieces::Battleship);
        }
        for _i in 0..K_NO3{
            Battleship::player_place_1_ship(self,ShipPieces::Cruiser);
        }
        for _i in 0..K_NO2{
            Battleship::player_place_1_ship(self,ShipPieces::Submarine);
        }
        println!("All Player ships being placed, ready to game!")
    }  
    pub fn cpu_place(&mut self) {
        for row in 0..=K_SIZE-1 {
            for col in 0..=K_SIZE-1 {
                if self.enemy_board[row][col].empty == false || self.enemy_board[row][col].guess == true {
                    panic!("Place in an already worked on board");
                }
            }
        }
        for _i in 0..K_NO5 {
            Battleship::cpu_place_1_ship(self,ShipPieces::Carrier);
        }
        for _i in 0..K_NO4{
            Battleship::cpu_place_1_ship(self,ShipPieces::Battleship);
        }
        for _i in 0..K_NO3{
            Battleship::cpu_place_1_ship(self,ShipPieces::Cruiser);
        }
        for _i in 0..K_NO2{
            Battleship::cpu_place_1_ship(self,ShipPieces::Submarine);
        }
        println!("All CPU ships being placed, ready to game!");
    }   


    pub fn cpu_place_1_ship(&mut self,ship_type : ShipPieces) ->() {
        let tuple_len_name : (usize,String) = match  ship_type {
            ShipPieces::Carrier => (5, "Carrier".to_string()),
            ShipPieces::Battleship => (4,"Battleship".to_string()),
            ShipPieces::Cruiser =>(3,"Cruiser".to_string()),
            ShipPieces::Submarine => (2,"Submarine".to_string()),
        };
        let len = tuple_len_name.0;
        let mut rng = rand::thread_rng();
        let mut finished: bool = false;
        while !finished {
            let row_start:usize = rng.gen_range(0..10);
            let col_start:usize = rng.gen_range(0..10);
            if self.enemy_board[row_start][col_start].empty == true {
                let direction = rng.gen_range(0..4);// 0 - up, 1 - down, 2- left, 3- right
                if direction == 0 {
                    if row_start >= len - 1 {
                        let mut a = true;
                        for i in 0..=len-1 {
                            if self.enemy_board[row_start - i][col_start].empty == false {
                                a = false;
                            }
                        }
                        if a {
                            for i in 0..=len-1 {
                                self.enemy_board[row_start -i][col_start].empty = false;
                                self.enemy_board[row_start -i][col_start].ship = tuple_len_name.1.clone();
                            }
                            finished = true;
                        }
                    }
                }
                if direction == 1 {
                    if row_start + len <= K_SIZE  {
                        let mut a:  bool = true;
                        for i in 0..=len-1 {
                            if self.enemy_board[row_start + i][col_start].empty == false {
                                a = false;
                            }
                        }
                        if a {
                            for i in 0..=len-1 {
                                self.enemy_board[row_start + i][col_start].empty = false;
                                self.enemy_board[row_start + i][col_start].ship = tuple_len_name.1.clone();
                            }
                            finished = true;
                        }
                    }
                }
                if direction == 2 {
                    if col_start >= len - 1 {
                        let mut a = true;
                        for i in 0..=len-1 {
                            if self.enemy_board[row_start][col_start - i].empty == false {
                                a = false;
                            }
                        }
                        if a {
                            for i in 0..=len-1 {
                                self.enemy_board[row_start][col_start - i].empty = false;
                                self.enemy_board[row_start][col_start - i].ship = tuple_len_name.1.clone();
                            }
                            finished = true;
                        }
                    }
                }
                if direction == 3 {
                    if col_start + len <= K_SIZE   {
                        let mut a = true;
                        for i in 0..=len-1 {
                            if self.enemy_board[row_start ][col_start+i].empty == false {
                                a = false;
                            }
                        }
                        if a {
                            for i in 0..=len-1 {
                                self.enemy_board[row_start ][col_start+i].empty = false;
                                self.enemy_board[row_start ][col_start+i].ship = tuple_len_name.1.clone();
                            }
                            finished = true;
                        }
                        
                    }
                }
            }
        }
    }
    pub fn get_game_result(&self) -> Option<bool> {
        let mut y_count = 0;
        for vec in &self.your_board {
            for i in vec {
                if i.guess == true && i.empty == false {
                    y_count = y_count + 1;
                }
            }
        }
        if y_count == 5 * K_NO5 + 4 * K_NO4 + 3 * K_NO3 + 2 * K_NO2 {
            return Some(false);
        }
        let mut e_count = 0;
        for vec in &self.enemy_board {
            for i in vec {
                if i.guess == true && i.empty == false {
                    e_count = e_count + 1;
                }
            }
        }
        if e_count == 5 * K_NO5 + 4 * K_NO4 + 3 * K_NO3 + 2 * K_NO2 {
            return Some(true);
        }
        return None;
    }
}