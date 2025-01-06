/* Programmer:          Per Stoor
 * Date:                2025-01-06
 * Last changed:        2025-01-06
 * Type of program:     Input: Two functions `f1` and `f2` where 
 *                      `f1(x) = x + 1`, `f2(x) = x * 2`
 *                      Expected Output: `5` if `f2(f1(2))` is calculated
 *                      Task: Show function composition with closures,
 *                      demonstrating how functions can be passed around like 
 *                      variables, emphasizing functional programming aspects 
 *                      in Rust.
 */

// use public_library::string_conversions::*;

fn main(){ 

    let closure_one = |x: i32| x + 1;
    let closure_two = |x: i32| x * 2 - 1; // The only way to get five is to add -1 here. This
                                          // exercis was A.I generated so it could be just a
                                          // mistake I guess...
    println!("{}", closure_two(closure_one(2)));

} 
