/// Error message for when a game is not in progress.
const GAME_NOT_STARTED : &str = "No game in progress. Start a new game with !new.";

/// Error message for when an incorrect number of arguments is provided.
const ARG_ERROR : &str = "Error in the number of arguments provided.";

/// Error message for an unrecognized command
const COMMAND_NOT_RECOGNIZED : &str = "Command Not Recognized.";

/// Error message for when a game is not in progress.
const GAME_ALREADY_OVER : &str = "Game already over. Start a new game with !new.";

#[derive(Debug)]
pub struct BattleshipError {
    pub error_type : BattleshipErrorKind,
    pub error_msg : String,
    pub user_input: String
}

#[derive(Debug, PartialEq)]
pub enum BattleshipErrorKind {
    CommandNotRecognized,
    ArgError,
    GameNotStarted,
    GameAlreadyOver
}

/// Display implementation for BattleshipErrorKind for user output in the CLI.
impl std::fmt::Display for BattleshipErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BattleshipError::{:?}", self)
    } 
}

/// BattleshipError impl block
impl BattleshipError {
    /// Instantiates a new BattleshipError object with the given error_type attached
    /// to the given user_input.
    pub fn new(error_type: BattleshipErrorKind, user_input: String) -> BattleshipError {
        let error_msg = match error_type {
            BattleshipErrorKind::CommandNotRecognized => COMMAND_NOT_RECOGNIZED.to_string(),
            BattleshipErrorKind::GameNotStarted => GAME_NOT_STARTED.to_string(),
            BattleshipErrorKind::GameAlreadyOver => GAME_ALREADY_OVER.to_string(),
            BattleshipErrorKind::ArgError => ARG_ERROR.to_string()
        };
        BattleshipError {
            error_type,
            error_msg,
            user_input
        }
    }
}

/// Display implementation for BattleshipError for user output in the CLI.
impl std::fmt::Display for BattleshipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{} --- (user_input : {})\n{}",
            self.error_type, self.user_input, self.error_msg)
    } 
}


/// Error impl for BattleshipError.
impl std::error::Error for BattleshipError {}