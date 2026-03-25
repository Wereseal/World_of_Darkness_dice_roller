mod io;
mod dice_handler;
use crate::io::Args;
use crate::dice_handler::DiceState;
use colored::Colorize;

fn main() {
    let args: Result<Args, &'static str> = Args::parse_args();
    match args {
        Ok(values) => println!("\n\nyou got {} successes", DiceState::roll_all(values)),
        Err(e) => println!("{}\n\n\t  {}", io::welcome_message(), (String::from("ERROR: ") + e).red()), 
    }
}
