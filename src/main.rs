use rand::Rng;
use std::env;

fn welcome_message(){
    println!("Welcome to my dice roller!");
    println!("enter the arguements in the form:");
    println!("cargo run arg1 arg2 arg3 or ./diceroller arg1 arg2 arg3");
    println!("arg1 = number of rolls");
    println!("arg2 = roll difficulty 2-10");
    println!("arg3 = enter y for special stat otherwise put nothing");
}
struct Args {
    num_rolls: i32,
    difficulty: i32,
    is_special: bool,
}
impl Args {
    fn parse_input(arguments: Vec<String>) -> Option<Args>{
        let num = arguments.get(1)?.trim().parse().ok()?;
        let diff = arguments.get(2)?.trim().parse().ok()?;
        let spec: bool;
        if diff < 2 || diff > 10 {
            return None;
        }
        if arguments.len() >= 4 && arguments[3] == String::from("y"){
            spec = true;       
        }
        else{
            spec = false;
        }
        return Some(Args{num_rolls: num,difficulty: diff,is_special: spec});
    }
}
fn dice_rolls(mut remaining_rolls: i32, difficulty: i32, is_special: bool) -> i32{
    let mut num_successes = 0;
    let mut rand = rand::rng();
    let mut roll: i32;
    println!("rolling...\n");
    if is_special {
        while remaining_rolls > 0 {
            roll = rand.random_range(1..11);
            print!("{} ", roll);
            if roll == 1 {
                num_successes -= 1;
            }
            else if roll == 10 {
                num_successes += 2;
            }
            else if roll >= difficulty {
                num_successes += 1;
            }
            remaining_rolls -=1;
        }
    }
    else {
        while remaining_rolls > 0 {
            roll = rand.random_range(1..11);
            print!("{} ", roll);
            if roll == 1 {
                num_successes -= 1;
            }
            else if roll == 10 {
                remaining_rolls += 2;
            }
            else if roll >= difficulty {
                num_successes += 1;
            }
            remaining_rolls -= 1;
        }
    }
    num_successes
}
fn main() {
    let inputs = Args::parse_input(env::args().collect());
    match inputs{
        Some(inner) => println!("\n\nyou got {} successes", dice_rolls(inner.num_rolls, inner.difficulty, inner.is_special)),
        None => welcome_message()
    }
}
