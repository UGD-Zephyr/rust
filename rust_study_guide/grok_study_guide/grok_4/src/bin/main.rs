/* Programmer:          Per Stoor
 * Date:                2024-12-31
 * Last changed:        2024-12-31
 * Type of program:     Write a simple function to add numbers, 
 *                      introducing function signatures and return 
 *                      values in Rust.
 */

use public_library::string_conversions::*;

fn main(){ 

    print!("Enter first number: ");
    let user_input_buffer = string_to_i32();
    let user_number1 = user_input_buffer;
    
    print!("Enter second number: ");
    let user_input_buffer = string_to_i32();
    let user_number2 = user_input_buffer;

    let sum = add_two_numbers(user_number1, user_number2);
    println!("The sum of {} + {} = {}",user_number1, user_number2, sum);
} 
fn add_two_numbers(first_number: i32, second_number: i32) -> i32{

    let sum_of_two_numbers: i32 = first_number + second_number;

return sum_of_two_numbers;
}
