use dialoguer::Input;
use crate::battleship::game;


/// Runs a Battleship game on the command line.
#[derive(Debug, Default)]
pub struct BattleshipRunner {
    game: Option<Battleship>,
    verbose: bool
}

impl BattleshipRunner {
    /// Constructs a new BattleshipRunner with a Battleship game from the given word.
    pub fn new(verbose: bool) -> BattleshipRunner {
        let mut output = BattleshipRunner { game: None, verbose };
        match position {
            Some(p) => output.handle_new(),
            _ => {}
        };
        output
    }

    /// Handles I/O for guessing a position. (!attack)
    fn handle_guess(&mut self, p: i32) {
        match &mut self.game {
            Some(g) => {
                match g.attack(p) {
                    Ok(true) => println!("[{}] is a succesful hit!\n", p),
                    Ok(false) => println!("[{}] is a miss.\n", p),
                    Err(e) => println!("{}", e)
                }
            },
            _ => {
                println!("{}", BattleshipError::new(BattleshipErrorKind::GameNotStarted, c.to_string()))
            }
        };
    }

    /// Handles I/O for starting a new game with a new word. (!new)
    fn handle_new(&mut self) {
        self.game = match Battleship::new() {
            Ok(b) => {
                println!("Starting battleship game!\n";
                Some(h)
            },
            Err(e) => {
                println!("{}", e);
                None
            }
        };
    }

    /// Displays the game status including the board status, incorrect guesses,
    /// correct guesses, and a drawing of the boards. (!status)
    fn display_status(&self) {
        match &self.game {
            Some(g) => {
                let mut status : String = String::new();
                status = format!("Enemy Board: \n
                | {ai} | {bi} | {ci} | {di} | {ei} | {fi} | {gi} | {hi} | {ii} | {ji} |\n
                |---------------------------------------------------------------------|\n
                | {aj} | {bj} | {cj} | {dj} | {ej} | {fj} | {gj} | {hj} | {ij} | {jj} |\n
                |---------------------------------------------------------------------|\n
                | {ak} | {bk} | {ck} | {dk} | {ek} | {fk} | {gk} | {hk} | {ik} | {jk} |\n
                |---------------------------------------------------------------------|\n
                | {al} | {bl} | {cl} | {dl} | {el} | {fl} | {gl} | {hl} | {il} | {jl} |\n
                |---------------------------------------------------------------------|\n
                | {am} | {bm} | {cm} | {dm} | {em} | {fm} | {gm} | {hm} | {im} | {jm} |\n
                |---------------------------------------------------------------------|\n
                | {an} | {bn} | {cn} | {dn} | {en} | {fn} | {gn} | {hn} | {in} | {jn} |\n
                |---------------------------------------------------------------------|\n
                | {ao} | {bo} | {co} | {do} | {eo} | {fo} | {go} | {ho} | {io} | {jo} |\n",
                ai = g.drawenemyboard(0), bi = g.drawenemyboard(1), ci = g.drawenemyboard(2), di = g.drawenemyboard(3),
                ei = g.drawenemyboard(4), fi = g.drawenemyboard(5), gi = g.drawenemyboard(6), hi = g.drawenemyboard(7),
                ii = g.drawenemyboard(8), ji = g.drawenemyboard(9),
                aj = g.drawenemyboard(0), bj = g.drawenemyboard(1), cj = g.drawenemyboard(2), dj = g.drawenemyboard(3),
                ej = g.drawenemyboard(4), fj = g.drawenemyboard(5), gj = g.drawenemyboard(6), hj = g.drawenemyboard(7),
                ij = g.drawenemyboard(8), jj = g.drawenemyboard(9),
                ak = g.drawenemyboard(0), bk = g.drawenemyboard(1), ck = g.drawenemyboard(2), dk = g.drawenemyboard(3),
                ek = g.drawenemyboard(4), fk = g.drawenemyboard(5), gk = g.drawenemyboard(6), hk = g.drawenemyboard(7),
                ik = g.drawenemyboard(8), jk = g.drawenemyboard(9),
                al = g.drawenemyboard(0), bl = g.drawenemyboard(1), cl = g.drawenemyboard(2), dl = g.drawenemyboard(3),
                el = g.drawenemyboard(4), fl = g.drawenemyboard(5), gl = g.drawenemyboard(6), hl = g.drawenemyboard(7),
                il = g.drawenemyboard(8), jl = g.drawenemyboard(9),
                am = g.drawenemyboard(0), bm = g.drawenemyboard(1), cm = g.drawenemyboard(2), dm = g.drawenemyboard(3),
                em = g.drawenemyboard(4), fm = g.drawenemyboard(5), gm = g.drawenemyboard(6), hm = g.drawenemyboard(7),
                im = g.drawenemyboard(8), jm = g.drawenemyboard(9),
                an = g.drawenemyboard(0), bn = g.drawenemyboard(1), cn = g.drawenemyboard(2), dn = g.drawenemyboard(3),
                en = g.drawenemyboard(4), fn = g.drawenemyboard(5), gn = g.drawenemyboard(6), hn = g.drawenemyboard(7),
                in = g.drawenemyboard(8), jn = g.drawenemyboard(9),
                ao = g.drawenemyboard(0), bo = g.drawenemyboard(1), co = g.drawenemyboard(2), do = g.drawenemyboard(3),
                eo = g.drawenemyboard(4), fo = g.drawenemyboard(5), go = g.drawenemyboard(6), ho = g.drawenemyboard(7),
                io = g.drawenemyboard(8), jo = g.drawenemyboard(9)

                ) + "\n" + format!("Your Board: \n
                | {ai} | {bi} | {ci} | {di} | {ei} | {fi} | {gi} | {hi} | {ii} | {ji} |\n
                |---------------------------------------------------------------------|\n
                | {aj} | {bj} | {cj} | {dj} | {ej} | {fj} | {gj} | {hj} | {ij} | {jj} |\n
                |---------------------------------------------------------------------|\n
                | {ak} | {bk} | {ck} | {dk} | {ek} | {fk} | {gk} | {hk} | {ik} | {jk} |\n
                |---------------------------------------------------------------------|\n
                | {al} | {bl} | {cl} | {dl} | {el} | {fl} | {gl} | {hl} | {il} | {jl} |\n
                |---------------------------------------------------------------------|\n
                | {am} | {bm} | {cm} | {dm} | {em} | {fm} | {gm} | {hm} | {im} | {jm} |\n
                |---------------------------------------------------------------------|\n
                | {an} | {bn} | {cn} | {dn} | {en} | {fn} | {gn} | {hn} | {in} | {jn} |\n
                |---------------------------------------------------------------------|\n
                | {ao} | {bo} | {co} | {do} | {eo} | {fo} | {go} | {ho} | {io} | {jo} |\n",
                ai = g.drawyourboard(0), bi = g.drawyourboard(1), ci = g.drawyourboard(2), di = g.drawyourboard(3),
                ei = g.drawyourboard(4), fi = g.drawyourboard(5), gi = g.drawyourboard(6), hi = g.drawyourboard(7),
                ii = g.drawyourboard(8), ji = g.drawyourboard(9),
                aj = g.drawyourboard(0), bj = g.drawyourboard(1), cj = g.drawyourboard(2), dj = g.drawyourboard(3),
                ej = g.drawyourboard(4), fj = g.drawyourboard(5), gj = g.drawyourboard(6), hj = g.drawyourboard(7),
                ij = g.drawyourboard(8), jj = g.drawyourboard(9),
                ak = g.drawyourboard(0), bk = g.drawyourboard(1), ck = g.drawyourboard(2), dk = g.drawyourboard(3),
                ek = g.drawyourboard(4), fk = g.drawyourboard(5), gk = g.drawyourboard(6), hk = g.drawyourboard(7),
                ik = g.drawyourboard(8), jk = g.drawyourboard(9),
                al = g.drawyourboard(0), bl = g.drawyourboard(1), cl = g.drawyourboard(2), dl = g.drawyourboard(3),
                el = g.drawyourboard(4), fl = g.drawyourboard(5), gl = g.drawyourboard(6), hl = g.drawyourboard(7),
                il = g.drawyourboard(8), jl = g.drawyourboard(9),
                am = g.drawyourboard(0), bm = g.drawyourboard(1), cm = g.drawyourboard(2), dm = g.drawyourboard(3),
                em = g.drawyourboard(4), fm = g.drawyourboard(5), gm = g.drawyourboard(6), hm = g.drawyourboard(7),
                im = g.drawyourboard(8), jm = g.drawyourboard(9),
                an = g.drawyourboard(0), bn = g.drawyourboard(1), cn = g.drawyourboard(2), dn = g.drawyourboard(3),
                en = g.drawyourboard(4), fn = g.drawyourboard(5), gn = g.drawyourboard(6), hn = g.drawyourboard(7),
                in = g.drawyourboard(8), jn = g.drawyourboard(9),
                ao = g.drawyourboard(0), bo = g.drawyourboard(1), co = g.drawyourboard(2), do = g.drawyourboard(3),
                eo = g.drawyourboard(4), fo = g.drawyourboard(5), go = g.drawyourboard(6), ho = g.drawyourboard(7),
                io = g.drawyourboard(8), jo = g.drawyourboard(9)
                );
                println!("Board Status: {}", status);
            },
            _ => {}
        }

        fn display_help(&self) {
            println!("!new: starts a new game.\n!attack [i32]: \
                guesses the position of an enemy battleship in the current game.\n!status: \
                outputs the current game status.\n!help: displays this help message.\n!exit: exits the game.\n")
        }
    }

    /// Runs a game of Battleship on the command line
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // display the status once at the very start.
        if self.verbose {
            self.display_status();
        }
        loop {
            // Get input from the user
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
            // match the first word in the command line input (the command)
            match args[0] {
                "!attack" => {
                    if args.len() == 2 {
                        match args[1].chars().to_string()).parse::<i32>() {
                            Some(x) if args[1].len() == 1 => self.handle_guess(x),
                            _  => println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!{"{:?}", args}));
                        };
                    } else {
                        println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!("{:?}", args)));
                    }
                },
                "!status" => {
                    self.display_status();
                    continue;
                },
                "!exit" => {
                    return Ok(());
                },
                "!help" => {
                    self.display_help();
                    continue;
                },
                "!new" => {
                    if args.len() == 2 {
                        self.handle_new(args[1].to_string());
                        
                    } else {
                        println!("{}", BattleshipError::new(BattleshipErrorKind::ArgError, format!("{:?}", args)));
                    }
                }
                _ => {
                    println!("{}", BattleshipError::new(BattleshipErrorKind::CommandNotRecognized, format!("{:?}", args)));
                    continue;
                }
            };
            match &self.game {
                Some(g) => {
                    match g.get_game_result() {
                        Some(true) => {
                            println!("Game Over. The player wins!\n");
                        },
                        Some(false) => {
                            println!("Game Over. The player loses.\n");
                        }
                        _ => {
                            if self.verbose {
                                self.display_status();
                            }
                        }
                    }
                },
                _ => {}
            };
        }
    }
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

