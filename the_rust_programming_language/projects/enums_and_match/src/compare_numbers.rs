/* Programmer:          Per Stoor
 * Date:                2024-02-11
 * Last changed:        2024-02-11
 * Type of program:     Rust program two add two numbers
 *                      and using match statements.
 */

fn main(){ 

    println!("Adding numbers 1 and 2...");
    let addition_result = add_numbers(1, 2);
        match addition_result {
            Ok(number) => number,
            Err(error) => println!("Error: {}", error),
        };

    println!("Addition result: {}", addition_result);

} 

fn add_numbers(number1: i32, number2: i32) -> i32 {

    let sum = number1 + number2;

return sum;
}
