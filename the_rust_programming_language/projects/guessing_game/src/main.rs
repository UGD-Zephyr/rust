use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    loop{
        print!("Enter your guess: ");

        let mut user_inputted_guess = String::new();

        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut user_inputted_guess)
            .expect("Error: Cannot read string.");

        let user_inputted_guess: i32 = match user_inputted_guess
            .trim()
            .parse(){
                Ok(user_inputted_guess) => user_inputted_guess,
                Err(_) => continue,
            };

        println!("Your guess was: {}", user_inputted_guess);

        match user_inputted_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
