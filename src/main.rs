pub mod runner;
pub mod opt;
pub mod Battleship;
use structopt::StructOpt;

/// main function for the hangman program
/// reads command line input and starts the game.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Battleship!\nInstructions: enter a command (e.g., !new, !attack, !exit, !status), \
                    or use !help for usage instructions.\n");
    // read command line args
    let cmd = opt::BattleshipOpt::from_args();
    // start the hangman runner
    let mut battleship_runner = runner::BattleshipRunner::new(cmd.verbose);
    // run the game
    battleship_runner.run()?;
    Ok(())
}