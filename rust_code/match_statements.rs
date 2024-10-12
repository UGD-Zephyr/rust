/* Programmer:          Per Stoor
 * Date:                2024-02-12
 * Last changed:        2024-02-12
 * Type of program:     Practicing the match statement.
 */

use std::io;
use std::io::Write;

fn main(){ 

    println!("Enter a number: ");

    let user_input = string_to_integer();
    println!("The number is: {}", user_input);

} 

fn string_to_integer() -> i32{

    let mut string_to_integer_buffer = String::new();

    match io::stdout().flush() {
        Ok(_)   => (),
        Err(_)  => {
            println!("There was an error flushing stdout!");
            return -1;
        }
    };

    io::stdin()
        .read_line(&mut string_to_integer_buffer)
        .expect("Error: Failed to read string.");

    let string_to_integer_buffer: i32 = match string_to_integer_buffer
        .trim()
        .parse() {
            Ok (success)    => success,
            Err (_)         => {
                println!("Error: Could not parse string to integer.");
                return -1;
            }
        };

return string_to_integer_buffer;
}
