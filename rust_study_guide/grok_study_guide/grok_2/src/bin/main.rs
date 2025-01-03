/* Programmer:          Per Stoor
 * Date:                2024-12-25
 * Last changed:        2024-12-25
 * Type of program:     Create a function that accepts a string
 *                      slice (&str), which demonstrates borrowing.
 *                      Print the string inside the function,
 *                      then return it to show that it's still
 *                      usable after the function call.
 *                      This illustrates how references work without
 *                      taking ownership.
 */

fn main(){
    let original_string_slice = "This is my string slice.";
    println!("Printing the string slice immediately: {}", original_string_slice);
    accept_string_slice(original_string_slice);
    println!("Printing the string slice after it is returned from the function: {}", original_string_slice);
} 

/*
 * This is where the borrowing is happening. The fact that the
 * function takes a &str as a parameter and returns &str is why
 * this works.
 * */
fn accept_string_slice (string_slice: &str) -> &str{
    println!("Printing &str: {}", string_slice);

return string_slice;
}
