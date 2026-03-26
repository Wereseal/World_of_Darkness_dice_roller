use std::env;

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
        let mut input_args = env::args();
        if input_args.len() < 2 {
            return Err("Too few arguments provided.\n");
        }
        input_args.next();
        let temp_rolls = input_args.next();
        let temp_diff = input_args.next();
        let temp_spec = input_args.next();

        let mut args: Args = Args::new();
        
        match temp_rolls.expect("Somehow rolls is None.").parse::<u32>() {
            Ok(t) => args.remaining_rolls = t,
            Err(_) => return Err("Please enter a valid number for number of rolls."),
        }
        if let Some(t) = temp_diff {
            match t.parse::<u32>() {
                Ok(t) => args.difficulty = Some(t),
                Err(_) => return Err("Please enter a valid number for difficulty."),
            }
        }
        if let Some(t) = temp_spec {
            args.is_special = t.to_lowercase() == String::from("y");
        }
        match Self::validate_args(&args) {
            Some(e) => return Err(e), 
            None => (),
        }
        return Ok(args);
    }
}
