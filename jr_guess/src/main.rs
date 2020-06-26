use std::io;
use rand::Rng;

fn main() {

    let mut continue_playing = true;
    while continue_playing {
        let guess = gather_input();
        let target = either_zero_or_one();
        report_user_success(&guess, &target);
        allow_user_to_elect_to_continue_playing(&mut continue_playing);
    }
}

fn allow_user_to_elect_to_continue_playing(continue_playing: &mut bool) {
    let tokens_user_elects_to_quit = ["e", "ex", "exit", "q", "quit"]; //todo: How to define once and share as some kind of constant, or readonly?
    println!("type any of {:?} to exit, or anything else (including just pressing return) to continue", tokens_user_elects_to_quit);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read user input");

    *continue_playing = !tokens_user_elects_to_quit.iter().any(|v| v == &user_input.trim());;
}

fn gather_input() -> i32 {
    let mut guess:i32 = 0;
    let mut user_input_valid = false;

    println!("What's your guess, between 1 and 10?");
    while !&user_input_valid {

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("failed to read user input");
        let try_parse_guess = try_parse(&user_input);

        match try_parse_guess {
            Some(x) => user_input_valid = x > 0 && x <= 10,
            None => user_input_valid = false
        }
        
        if !user_input_valid {
            println!("Try again with a valid input");
        } else {
            guess = try_parse_guess.unwrap();
        }
    }
    return guess;
}

fn either_zero_or_one() -> i32 {
    let random_number = rand::thread_rng().gen_range(1, 11);
    return random_number;
}

fn try_parse(source: &String) -> Option<i32> {
    match source.trim().parse::<i32>() {
        Ok(n) => return Some(n),
        Err(_n) => return None,
    }
}

fn report_user_success(guess: &i32, target: &i32) {
    println!("you guessed {}. We generated {}. You {}!", guess, target, if guess == target {"win"} else {"lose"}  )
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// I'm following this
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

// Learned:
// & passes by ref. In this example it was similar to ref out
// mut makes something mutable. Things aren't mutable by default
// I think I need to provide "mut" when I intend to change the value
// I don't seem to need it for reading the value
// Rust prefers snake case - i_am_not_used_to_snake_case
// .expect seems to be some kind of catch{ Console.WriteLine(.); }
// Options are super useful as always. Note the need to .unwrap() to get the value out.
// When reading a line with read_line, it will include the trailing line termination. You'll want to .trim() it off