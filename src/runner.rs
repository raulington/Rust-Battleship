use dialoguer::Input;
use crate::Battleship::game::*;
use crate::Battleship::error::*;
use rand::Rng;

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
        output
    }

    /// Handles I/O for guessing a position. (!attack)
    fn handle_guess(&mut self, player_board : bool, coordinates : (i32, i32)) -> bool {
        match &mut self.game {
            Some(g) => {
                match g.attack(player_board, coordinates) {
                    2 => println!("Succesful hit! \n"),
                    0 => println!("bad hit \n"),
                    1 => {
                        println!("miss.\n");
                        return false;
                    },
                    _ => println!("num not generated")
                }
            },
            _ => {
                println!("{}", BattleshipError::new(BattleshipErrorKind::GameNotStarted, "game not started".to_string()))
            }
        };
        return true;
    }

    fn handle_cpu_guess(&mut self) -> (i32, i32) {
        match &mut self.game {
            Some(g) => {
                return g.cpu_attack();
            },
            _ => {
                println!("{}", BattleshipError::new(BattleshipErrorKind::GameNotStarted, "game not started".to_string()))
            }
        };
        return (0,0);
    }

    /// Handles I/O for starting a new game with a new word. (!new)
    fn handle_new(&mut self) {
        self.game = match Battleship::new() {
            Ok(b) => {
                println!("Starting battleship game!\n");
                Some(b)
            },
            Err(e) => {
                println!("{}", e);
                None
            }
        };
        self.game.as_mut().unwrap().cpu_place();
        self.game.as_mut().unwrap().player_place();
    }

    fn display_status(&self) {
        match &self.game {
            Some(g) => {
                let mut status : String = String::new();
                status = format!("{a}{b}{c}", a = format!("Enemy Board: \n
                {ai} {bi} {ci} {di} {ei} {fi} {gi} {hi} {ii} {ji}\n
                {aj} {bj} {cj} {dj} {ej} {fj} {gj} {hj} {ij} {jj}\n
                {ak} {bk} {ck} {dk} {ek} {fk} {gk} {hk} {ik} {jk}\n
                {al} {bl} {cl} {dl} {el} {fl} {gl} {hl} {il} {jl}\n
                {am} {bm} {cm} {dm} {em} {fm} {gm} {hm} {im} {jm}\n
                {an} {bn} {cn} {dn} {en} {ffn} {gn} {hn} {iin} {jn}\n
                {ao} {bo} {co} {doo} {eo} {fo} {go} {ho} {io} {jo}\n
                {ap} {bp} {cp} {dp} {ep} {fp} {gp} {hp} {ip} {jp}\n
                {aq} {bq} {cq} {dq} {eq} {fq} {gq} {hq} {iq} {jq}\n
                {ar} {br} {cr} {dr} {er} {fr} {gr} {hr} {ir} {jr}\n",
                ai = g.drawenemyboard(0, 0), bi = g.drawenemyboard(0, 1), ci = g.drawenemyboard(0, 2), di = g.drawenemyboard(0, 3),
                ei = g.drawenemyboard(0, 4), fi = g.drawenemyboard(0, 5), gi = g.drawenemyboard(0, 6), hi = g.drawenemyboard(0, 7),
                ii = g.drawenemyboard(0, 8), ji = g.drawenemyboard(0, 9),
                aj = g.drawenemyboard(1, 0), bj = g.drawenemyboard(1, 1), cj = g.drawenemyboard(1, 2), dj = g.drawenemyboard(1, 3),
                ej = g.drawenemyboard(1, 4), fj = g.drawenemyboard(1, 5), gj = g.drawenemyboard(1, 6), hj = g.drawenemyboard(1, 7),
                ij = g.drawenemyboard(1, 8), jj = g.drawenemyboard(1, 9),
                ak = g.drawenemyboard(2, 0), bk = g.drawenemyboard(2, 1), ck = g.drawenemyboard(2, 2), dk = g.drawenemyboard(2, 3),
                ek = g.drawenemyboard(2, 4), fk = g.drawenemyboard(2, 5), gk = g.drawenemyboard(2, 6), hk = g.drawenemyboard(2, 7),
                ik = g.drawenemyboard(2, 8), jk = g.drawenemyboard(2, 9),
                al = g.drawenemyboard(3, 0), bl = g.drawenemyboard(3, 1), cl = g.drawenemyboard(3, 2), dl = g.drawenemyboard(3, 3),
                el = g.drawenemyboard(3, 4), fl = g.drawenemyboard(3, 5), gl = g.drawenemyboard(3, 6), hl = g.drawenemyboard(3, 7),
                il = g.drawenemyboard(3, 8), jl = g.drawenemyboard(3, 9),
                am = g.drawenemyboard(4, 0), bm = g.drawenemyboard(4, 1), cm = g.drawenemyboard(4, 2), dm = g.drawenemyboard(4, 3),
                em = g.drawenemyboard(4, 4), fm = g.drawenemyboard(4, 5), gm = g.drawenemyboard(4, 6), hm = g.drawenemyboard(4, 7),
                im = g.drawenemyboard(4, 8), jm = g.drawenemyboard(4, 9),
                an = g.drawenemyboard(5, 0), bn = g.drawenemyboard(5, 1), cn = g.drawenemyboard(5, 2), dn = g.drawenemyboard(5, 3),
                en = g.drawenemyboard(5, 4), ffn = g.drawenemyboard(5, 5), gn = g.drawenemyboard(5, 6), hn = g.drawenemyboard(5, 7),
                iin = g.drawenemyboard(5, 8), jn = g.drawenemyboard(5, 9),
                ao = g.drawenemyboard(6, 0), bo = g.drawenemyboard(6, 1), co = g.drawenemyboard(6, 2), doo = g.drawenemyboard(6, 3),
                eo = g.drawenemyboard(6, 4), fo = g.drawenemyboard(6, 5), go = g.drawenemyboard(6, 6), ho = g.drawenemyboard(6, 7),
                io = g.drawenemyboard(6, 8), jo = g.drawenemyboard(6, 9),
                ap = g.drawenemyboard(7, 0), bp = g.drawenemyboard(7, 1), cp = g.drawenemyboard(7, 2), dp = g.drawenemyboard(7, 3),
                ep = g.drawenemyboard(7, 4), fp = g.drawenemyboard(7, 5), gp = g.drawenemyboard(7, 6), hp = g.drawenemyboard(7, 7),
                ip = g.drawenemyboard(7, 8), jp = g.drawenemyboard(7, 9),
                aq = g.drawenemyboard(8, 0), bq = g.drawenemyboard(8, 1), cq = g.drawenemyboard(8, 2), dq = g.drawenemyboard(8, 3),
                eq = g.drawenemyboard(8, 4), fq = g.drawenemyboard(8, 5), gq = g.drawenemyboard(8, 6), hq = g.drawenemyboard(8, 7),
                iq = g.drawenemyboard(8, 8), jq = g.drawenemyboard(8, 9),
                ar = g.drawenemyboard(9, 0), br = g.drawenemyboard(9, 1), cr = g.drawenemyboard(9, 2), dr = g.drawenemyboard(9, 3),
                er = g.drawenemyboard(9, 4), fr = g.drawenemyboard(9, 5), gr = g.drawenemyboard(9, 6), hr = g.drawenemyboard(9, 7),
                ir = g.drawenemyboard(9, 8), jr = g.drawenemyboard(9, 9)
                ), b = "\n", c = format!("Your Board: \n
                {ai} {bi} {ci} {di} {ei} {fi} {gi} {hi} {ii} {ji}\n
                {aj} {bj} {cj} {dj} {ej} {fj} {gj} {hj} {ij} {jj}\n
                {ak} {bk} {ck} {dk} {ek} {fk} {gk} {hk} {ik} {jk}\n
                {al} {bl} {cl} {dl} {el} {fl} {gl} {hl} {il} {jl}\n
                {am} {bm} {cm} {dm} {em} {fm} {gm} {hm} {im} {jm}\n
                {an} {bn} {cn} {dn} {en} {ffn} {gn} {hn} {iin} {jn}\n
                {ao} {bo} {co} {doo} {eo} {fo} {go} {ho} {io} {jo}\n
                {ap} {bp} {cp} {dp} {ep} {fp} {gp} {hp} {ip} {jp}\n
                {aq} {bq} {cq} {dq} {eq} {fq} {gq} {hq} {iq} {jq}\n
                {ar} {br} {cr} {dr} {er} {fr} {gr} {hr} {ir} {jr}\n",
                ai = g.drawyourboard(0, 0), bi = g.drawyourboard(0, 1), ci = g.drawyourboard(0, 2), di = g.drawyourboard(0, 3),
                ei = g.drawyourboard(0, 4), fi = g.drawyourboard(0, 5), gi = g.drawyourboard(0, 6), hi = g.drawyourboard(0, 7),
                ii = g.drawyourboard(0, 8), ji = g.drawyourboard(0, 9),
                aj = g.drawyourboard(1, 0), bj = g.drawyourboard(1, 1), cj = g.drawyourboard(1, 2), dj = g.drawyourboard(1, 3),
                ej = g.drawyourboard(1, 4), fj = g.drawyourboard(1, 5), gj = g.drawyourboard(1, 6), hj = g.drawyourboard(1, 7),
                ij = g.drawyourboard(1, 8), jj = g.drawyourboard(1, 9),
                ak = g.drawyourboard(2, 0), bk = g.drawyourboard(2, 1), ck = g.drawyourboard(2, 2), dk = g.drawyourboard(2, 3),
                ek = g.drawyourboard(2, 4), fk = g.drawyourboard(2, 5), gk = g.drawyourboard(2, 6), hk = g.drawyourboard(2, 7),
                ik = g.drawyourboard(2, 8), jk = g.drawyourboard(2, 9),
                al = g.drawyourboard(3, 0), bl = g.drawyourboard(3, 1), cl = g.drawyourboard(3, 2), dl = g.drawyourboard(3, 3),
                el = g.drawyourboard(3, 4), fl = g.drawyourboard(3, 5), gl = g.drawyourboard(3, 6), hl = g.drawyourboard(3, 7),
                il = g.drawyourboard(3, 8), jl = g.drawyourboard(3, 9),
                am = g.drawyourboard(4, 0), bm = g.drawyourboard(4, 1), cm = g.drawyourboard(4, 2), dm = g.drawyourboard(4, 3),
                em = g.drawyourboard(4, 4), fm = g.drawyourboard(4, 5), gm = g.drawyourboard(4, 6), hm = g.drawyourboard(4, 7),
                im = g.drawyourboard(4, 8), jm = g.drawyourboard(4, 9),
                an = g.drawyourboard(5, 0), bn = g.drawyourboard(5, 1), cn = g.drawyourboard(5, 2), dn = g.drawyourboard(5, 3),
                en = g.drawyourboard(5, 4), ffn = g.drawyourboard(5, 5), gn = g.drawyourboard(5, 6), hn = g.drawyourboard(5, 7),
                iin = g.drawyourboard(5, 8), jn = g.drawyourboard(5, 9),
                ao = g.drawyourboard(6, 0), bo = g.drawyourboard(6, 1), co = g.drawyourboard(6, 2), doo = g.drawyourboard(6, 3),
                eo = g.drawyourboard(6, 4), fo = g.drawyourboard(6, 5), go = g.drawyourboard(6, 6), ho = g.drawyourboard(6, 7),
                io = g.drawyourboard(6, 8), jo = g.drawyourboard(6, 9),
                ap = g.drawyourboard(7, 0), bp = g.drawyourboard(7, 1), cp = g.drawyourboard(7, 2), dp = g.drawyourboard(7, 3),
                ep = g.drawyourboard(7, 4), fp = g.drawyourboard(7, 5), gp = g.drawyourboard(7, 6), hp = g.drawyourboard(7, 7),
                ip = g.drawyourboard(7, 8), jp = g.drawyourboard(7, 9),
                aq = g.drawyourboard(8, 0), bq = g.drawyourboard(8, 1), cq = g.drawyourboard(8, 2), dq = g.drawyourboard(8, 3),
                eq = g.drawyourboard(8, 4), fq = g.drawyourboard(8, 5), gq = g.drawyourboard(8, 6), hq = g.drawyourboard(8, 7),
                iq = g.drawyourboard(8, 8), jq = g.drawyourboard(8, 9),
                ar = g.drawyourboard(9, 0), br = g.drawyourboard(9, 1), cr = g.drawyourboard(9, 2), dr = g.drawyourboard(9, 3),
                er = g.drawyourboard(9, 4), fr = g.drawyourboard(9, 5), gr = g.drawyourboard(9, 6), hr = g.drawyourboard(9, 7),
                ir = g.drawyourboard(9, 8), jr = g.drawyourboard(9, 9)
                ));
                println!("Board Status: {}", status);
            },
            _ => {}
        }
    }


    fn display_self_status(&self) {
        match &self.game {
            Some(g) => {
                let mut status : String = String::new();
                status = format!("{a}", a = format!("Your Board: \n
                {ai} {bi} {ci} {di} {ei} {fi} {gi} {hi} {ii} {ji}\n
                {aj} {bj} {cj} {dj} {ej} {fj} {gj} {hj} {ij} {jj}\n
                {ak} {bk} {ck} {dk} {ek} {fk} {gk} {hk} {ik} {jk}\n
                {al} {bl} {cl} {dl} {el} {fl} {gl} {hl} {il} {jl}\n
                {am} {bm} {cm} {dm} {em} {fm} {gm} {hm} {im} {jm}\n
                {an} {bn} {cn} {dn} {en} {ffn} {gn} {hn} {iin} {jn}\n
                {ao} {bo} {co} {doo} {eo} {fo} {go} {ho} {io} {jo}\n
                {ap} {bp} {cp} {dp} {ep} {fp} {gp} {hp} {ip} {jp}\n
                {aq} {bq} {cq} {dq} {eq} {fq} {gq} {hq} {iq} {jq}\n
                {ar} {br} {cr} {dr} {er} {fr} {gr} {hr} {ir} {jr}\n",
                ai = g.drawyourboard(0, 0), bi = g.drawyourboard(0, 1), ci = g.drawyourboard(0, 2), di = g.drawyourboard(0, 3),
                ei = g.drawyourboard(0, 4), fi = g.drawyourboard(0, 5), gi = g.drawyourboard(0, 6), hi = g.drawyourboard(0, 7),
                ii = g.drawyourboard(0, 8), ji = g.drawyourboard(0, 9),
                aj = g.drawyourboard(1, 0), bj = g.drawyourboard(1, 1), cj = g.drawyourboard(1, 2), dj = g.drawyourboard(1, 3),
                ej = g.drawyourboard(1, 4), fj = g.drawyourboard(1, 5), gj = g.drawyourboard(1, 6), hj = g.drawyourboard(1, 7),
                ij = g.drawyourboard(1, 8), jj = g.drawyourboard(1, 9),
                ak = g.drawyourboard(2, 0), bk = g.drawyourboard(2, 1), ck = g.drawyourboard(2, 2), dk = g.drawyourboard(2, 3),
                ek = g.drawyourboard(2, 4), fk = g.drawyourboard(2, 5), gk = g.drawyourboard(2, 6), hk = g.drawyourboard(2, 7),
                ik = g.drawyourboard(2, 8), jk = g.drawyourboard(2, 9),
                al = g.drawyourboard(3, 0), bl = g.drawyourboard(3, 1), cl = g.drawyourboard(3, 2), dl = g.drawyourboard(3, 3),
                el = g.drawyourboard(3, 4), fl = g.drawyourboard(3, 5), gl = g.drawyourboard(3, 6), hl = g.drawyourboard(3, 7),
                il = g.drawyourboard(3, 8), jl = g.drawyourboard(3, 9),
                am = g.drawyourboard(4, 0), bm = g.drawyourboard(4, 1), cm = g.drawyourboard(4, 2), dm = g.drawyourboard(4, 3),
                em = g.drawyourboard(4, 4), fm = g.drawyourboard(4, 5), gm = g.drawyourboard(4, 6), hm = g.drawyourboard(4, 7),
                im = g.drawyourboard(4, 8), jm = g.drawyourboard(4, 9),
                an = g.drawyourboard(5, 0), bn = g.drawyourboard(5, 1), cn = g.drawyourboard(5, 2), dn = g.drawyourboard(5, 3),
                en = g.drawyourboard(5, 4), ffn = g.drawyourboard(5, 5), gn = g.drawyourboard(5, 6), hn = g.drawyourboard(5, 7),
                iin = g.drawyourboard(5, 8), jn = g.drawyourboard(5, 9),
                ao = g.drawyourboard(6, 0), bo = g.drawyourboard(6, 1), co = g.drawyourboard(6, 2), doo = g.drawyourboard(6, 3),
                eo = g.drawyourboard(6, 4), fo = g.drawyourboard(6, 5), go = g.drawyourboard(6, 6), ho = g.drawyourboard(6, 7),
                io = g.drawyourboard(6, 8), jo = g.drawyourboard(6, 9),
                ap = g.drawyourboard(7, 0), bp = g.drawyourboard(7, 1), cp = g.drawyourboard(7, 2), dp = g.drawyourboard(7, 3),
                ep = g.drawyourboard(7, 4), fp = g.drawyourboard(7, 5), gp = g.drawyourboard(7, 6), hp = g.drawyourboard(7, 7),
                ip = g.drawyourboard(7, 8), jp = g.drawyourboard(7, 9),
                aq = g.drawyourboard(8, 0), bq = g.drawyourboard(8, 1), cq = g.drawyourboard(8, 2), dq = g.drawyourboard(8, 3),
                eq = g.drawyourboard(8, 4), fq = g.drawyourboard(8, 5), gq = g.drawyourboard(8, 6), hq = g.drawyourboard(8, 7),
                iq = g.drawyourboard(8, 8), jq = g.drawyourboard(8, 9),
                ar = g.drawyourboard(9, 0), br = g.drawyourboard(9, 1), cr = g.drawyourboard(9, 2), dr = g.drawyourboard(9, 3),
                er = g.drawyourboard(9, 4), fr = g.drawyourboard(9, 5), gr = g.drawyourboard(9, 6), hr = g.drawyourboard(9, 7),
                ir = g.drawyourboard(9, 8), jr = g.drawyourboard(9, 9)
                ));
                println!("{}", status);
            },
            _ => {}
        }
    }


    fn display_help(&self) {
        println!("!new: starts a new game.\n!attack [i32]: \
            guesses the position of an enemy battleship in the current game.\n!status: \
            outputs the current game status.\n!help: displays this help message.\n!exit: exits the game.\n")
    }

    pub fn coordinate_converter(&self, coord: &str) -> Result<(i32, i32), ()> {
        let mut all : Vec<char> = Vec::new();
        for i in 0..coord.len() {
            all.push(coord.chars().nth(i).unwrap());
        }
    
        let mut variables : Vec<char> = Vec::new();
        for i in 0..all.len() {
            if all[i] == 'A' || all[i] == 'B' || all[i] == 'C' || all[i] == 'D' || all[i] == 'E' 
            || all[i] == 'F' || all[i] == 'G' || all[i] == 'H' || all[i] == 'I' || all[i] == 'J'
            ||   all[i] == 'a' || all[i] == 'b' || all[i] == 'c' || all[i] == 'd' || all[i] == 'e' 
            || all[i] == 'f' || all[i] == 'g' || all[i] == 'h' || all[i] == 'i' || all[i] == 'j'
            || all[i] == '1' || all[i] == '2' || all[i] == '3' || all[i] == '4' || all[i] == '5'
            || all[i] == '6' || all[i] == '7' || all[i] == '8' || all[i] == '9' || all[i] == '0' {
                variables.push(all[i]);
            }
        }
    
        let mut num: i32 = -1;
        if variables.len() != 2 && variables.len() != 3 {
            return Err(());
        }
        if variables.len() == 3 {
            if variables[1] == '1' && variables[2] == '0' {
                num = 10 -1;
            } else if variables[1] == '0' {
                match variables[2] {
                    '1' => num = 1-1,
                    '2' => num = 2-1,
                    '3' => num = 3-1,
                    '4' => num = 4-1,
                    '5' => num = 5-1,
                    '6' => num = 6-1,
                    '7' => num = 7-1,
                    '8' => num = 8-1,
                    '9' => num = 9-1,
                    _ => return Err(())
                }
            } else {
                return Err(());
            }
        } else {
            match variables[1] {
                '1' => num = 1-1,
                '2' => num = 2-1,
                '3' => num = 3-1,
                '4' => num = 4-1,
                '5' => num = 5-1,
                '6' => num = 6-1,
                '7' => num = 7-1,
                '8' => num = 8-1,
                '9' => num = 9-1,
                _ => return Err(())
            }
        }
    
        let mut letter: i32;
    
        match variables[0] {
            'a' | 'A' => letter = 0,
            'b' | 'B' => letter = 1,
            'c' | 'C' => letter = 2,
            'd' | 'D' => letter = 3,
            'e' | 'E' => letter = 4,
            'f' | 'F' => letter = 5,
            'g' | 'G' => letter = 6,
            'h' | 'H' => letter = 7,
            'i' | 'I' => letter = 8,
            'j' | 'J' => letter = 9,
            _ => return Err(())
        }
    
        return Ok((letter, num));
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
                    let mut spot = args[1];
                    let coords = self.coordinate_converter(spot).unwrap();
                    let outcome = self.handle_guess(false, coords);
                    let cpucoords = self.handle_cpu_guess();
                    let cpuoutcome = self.handle_guess(true, cpucoords);
                    continue;
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
                    self.handle_new();
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
}
///////

