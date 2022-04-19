use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="Battleship", about="Battleship Game", author="Raul, Anh, Joseph")]
/// Handles command line arguments for a Hangman game.
pub struct BattleshipOpt {
    /// Displays the game status after every guess when verbose is True.
    /// Displays the game status only after the !status command is used when False.
    #[structopt(short, long)]
    pub verbose: bool,
}