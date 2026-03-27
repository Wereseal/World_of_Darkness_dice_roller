use std::{env, fmt};
use std::error::Error;

pub fn welcome_message() -> String {
    format!("\n\tWelcome to my dice roller!\n
    {}", get_instructions())
}
pub fn get_instructions() -> String {
    //FIXME: Correct instructions to account for printing only total.
    format!("Enter the arguements in the form:
    \"cargo run arg1 arg2 arg3\" or \"./diceroller arg1 arg2 arg3\"
    arg1 = Number of rolls.
    arg2 = Roll difficulty 2-10.
    arg3 = Enter y for special stat otherwise put nothing.")
}
#[derive(PartialEq, Debug)]
pub enum ArgsError {
    InvalidDifficulty,
    InsufficientArgs,
    NaNRolls,
    NaNDifficulty,
}
impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            ArgsError::InvalidDifficulty => "Difficulty was out of bounds.",
            ArgsError::InsufficientArgs => "Too few arguments provided.",
            ArgsError::NaNRolls => "Please enter a valid number for number of rolls.",
            ArgsError::NaNDifficulty => "Please enter a valid number for difficulty.",
        };
        f.write_str(description)
    }
}
impl Error for ArgsError {}

pub struct Args {
    pub remaining_rolls: u32,
    pub difficulty: Option<u32>,
    pub is_special: bool,
}
impl Args {
    fn new() -> Self {
        Args {
            remaining_rolls: 0,
            difficulty: None,
            is_special: false,
        }
    }
    fn validate_args(args: &Args) -> Result<(), ArgsError> {
        if let Some(diff) = args.difficulty {
            if diff < 2 || diff > 10 {
                return Err(ArgsError::InvalidDifficulty);
            }
        }
        Ok(())
    }
    pub fn parse_args() -> Result<Args, ArgsError>{
        let mut input_args = env::args();
        if input_args.len() < 2 {
            return Err(ArgsError::InsufficientArgs);
        }
        input_args.next();
        let temp_rolls = input_args.next();
        let temp_diff = input_args.next();
        let temp_spec = input_args.next();

        let mut args: Args = Args::new();
        
        match temp_rolls.expect("Somehow rolls is None.").parse::<u32>() {
            Ok(t) => args.remaining_rolls = t,
            Err(_) => return Err(ArgsError::NaNRolls),
        }
        if let Some(t) = temp_diff {
            match t.parse::<u32>() {
                Ok(t) => args.difficulty = Some(t),
                Err(_) => return Err(ArgsError::NaNDifficulty),
            }
        }
        if let Some(t) = temp_spec {
            args.is_special = t.to_lowercase() == String::from("y");
        }
        Self::validate_args(&args)?;
        return Ok(args);
    }
}
