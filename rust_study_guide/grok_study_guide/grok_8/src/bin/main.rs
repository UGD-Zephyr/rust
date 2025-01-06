/* Programmer:          Per Stoor
 * Date:                2025-01-05
 * Last changed:        2025-01-05
 * Type of program:     Input: A vector of strings `["hello", "world"]`
 *                      Expected Output: `["HELLO", "WORLD"]`
 *                      Task: Define a closure to transform strings to uppercase.
 *                      This uses Rust's closure syntax and introduces the 
 *                      concept of higher-order functions. 
 */

use public_library::string_conversions::*;

fn main(){ 

    let mut lowercase_strings: Vec<String> = Vec::new(); 

        for _ in 0..2 {
            print!("Enter a string: ");
            let user_input = input_to_string();
                lowercase_strings.push(user_input);
    }

    let uppercase_strings: Vec<String> = lowercase_strings
                                                    .iter()
                                                    .map(|x| x.to_uppercase())
                                                    .collect();

    println!("The finished lowercase vector: {:?}", lowercase_strings);
    println!("The finished uppercase vector: {:?}", uppercase_strings);
}
