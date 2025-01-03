/* Programmer:          Per Stoor
 * Date:                2025-01-01
 * Last changed:        2025-01-01
 * Type of program:     Use if-else to check if a number is even or odd,
 *                      demonstrating conditional control flow.
 *                      This will also introduce you to string comparison 
 *                      in Rust.
 */

use public_library::string_conversions::*;

fn main(){ 

    print!("Enter a number: ");
    let user_input = string_to_i32();
        if is_number_even(user_input){
            println!("Number is even.");
        }
        else {
            println!("Number is odd.");
        }
} 
fn is_number_even(number_string: i32) -> bool{

    let even_number: bool = number_string % 2 == 0;
        if even_number{
            return true;
        }
        else {
            return false;
        }
}
