/* Programmer:          Per Stoor
 * Date:                2025-01-02
 * Last changed:        2025-01-02
 * Type of program:     Input: A char 'A'
                        Expected Output: "It's an uppercase letter."
                        Task: Utilize match to categorize a character,
                        introducing pattern matching for control flow.
                        This teaches how to handle various types of data 
                        with a single construct.
 */

// use public_library::string_conversions::*;
use std::io::{self, Write};

fn main(){ 

    print!("Enter alphabet character: ");
    let user_input = string_to_char();

    match user_input {
        'a'..'z' => println!("It's a lowercase letter!"),
        'A'..'Z' => println!("It's an uppercase letter!"),
        _ => println!("Not an alphabet character!"),
    };
} 

fn string_to_char() -> char {
        let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<char>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

return string_buffer;
}
