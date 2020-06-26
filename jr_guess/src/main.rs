use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut guess = None;
    
    while guess == None {
        let input = gather_input();
        guess = try_parse(&input);
    }

    let target = either_zero_or_one();
    report_user_success(&guess.unwrap(), &target);
}

fn gather_input() -> String {
    
    println!("What's your guess?");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read user input");
    return guess.trim().to_string();
}

fn either_zero_or_one() -> i32 {
    let random_number = rand::thread_rng().gen_range(0, 2);
    return random_number;
}

fn try_parse(source: &String) -> Option<i32> {
    match source.parse::<i32>() {
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