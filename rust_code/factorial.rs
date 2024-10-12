/* Programmer:          Per Stoor
 * Date:                2024-02-16
 * Last changed:        2024-02-16
 * Type of program:     Calculating user input integer into it's factorial result
 */

use std::io;
use std::io::Write;

fn main(){ 

    print!("Please enter a number: ");
    let user_input = string_to_integer();

    factorial_print(user_input);

} 

fn string_to_integer() -> i32 {

    let mut string_to_integer_buffer = String::new();

    match io::stdout().flush() {
        Ok(_)   => (),
        Err(_)  => {
            println!("Error: Failed to flush standard out.");
            return -1;
        }
    };

    match io::stdin().read_line(&mut string_to_integer_buffer) {
        Ok(_)   => (),
        Err(_)  => {
            println!("Error: Failed to read string from standard in.");
            return -1;
        }
    };

    let string_to_integer_buffer: i32 = match string_to_integer_buffer
        .trim()
        .parse() {
            Ok(success) => success,
            Err(_)      => {
                println!("Error: Could not parse string to integer.");
                return -1;
            }
        };

return string_to_integer_buffer;
}

fn factorial_print(factorial_number: i32) {

    let mut factorial_sum = factorial_number;
    for loop_counter1 in (1..factorial_number).rev() {
        factorial_sum = factorial_sum * loop_counter1;
        print!("{} x ", loop_counter1 + 1);
    };

        print!("1 = {}\n", factorial_sum);

}
