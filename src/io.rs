use std::{env, num::ParseIntError};

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
pub struct Args{
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
    fn validate_args(args: &Args) -> Option<&'static str> {
        match args.difficulty {
            None => (),
            Some(diff) => {
                if diff < 2 || diff > 10 {
                    return Some("Difficulty was out of bounds.");
                }
            }
        }
        return None;
    }
    pub fn parse_args() -> Result<Args, &'static str>{
        let input_args = env::args();
        let mut temp_rolls: Result<u32, ParseIntError> = Ok(0); 
        let mut temp_diff: Result<u32, ParseIntError> = Ok(0);
        let mut temp_spec: String = String::from(""); 
        let mut args: Args = Args::new();

        if input_args.len() < 2 {
            return Err("Too few arguments provided.\n");
        }
        
        for (i, arg) in input_args.into_iter().enumerate() {
            println!("{}", arg);
            match i {
                0 => (),
                1 => temp_rolls = arg.parse(),
                2 => temp_diff = arg.parse(),
                3 => temp_spec = arg,
                4.. => break,
            }
        }
        match temp_rolls {
            Ok(t) => args.remaining_rolls = t,
            Err(_) => return Err("Please enter a valid number for number of rolls."),
        }
        match temp_diff{
            Ok(t) => args.difficulty = Some(t),
            Err(_) => return Err("Please enter a valid number for difficulty."),
        }
        //FIXME: Hacky fix because I am tired, correct later.
        if args.difficulty == Some(0) {
            args.difficulty = None;
        }
        if temp_spec.to_lowercase() == String::from("y") {
            args.is_special = true;
        }
        match Self::validate_args(&args) {
            Some(e) => return Err(e), 
            None => (),
        }
        return Ok(args);
    }
}
