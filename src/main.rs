use diceroller::io::{self, Args};
use diceroller::dice_handler::DiceState;
use colored::Colorize;

fn run(args: Args) {
    if args.difficulty == None {
        println!("\n\nDice roll total: {} ", DiceState::roll_all(args));
    } else {
        println!("\n\nyou got {} successes", DiceState::roll_all(args));
    }
}
fn main() {
    match Args::parse_args() {
        Ok(args) => run(args),
        Err(e) => println!("{}\n\n\t{}", io::welcome_message(), format!("ERROR: {}", e).red()), 
    }
}
