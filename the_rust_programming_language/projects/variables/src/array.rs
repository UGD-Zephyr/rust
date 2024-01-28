/* Programmer:          Per Stoor
 * Date:                2024-01-20
 * Last changed:        2024-01-20
 * Type of program:     Making an array program from chapter 3.
 */

use std::io;
use std::io::Write;

fn main(){ 

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    /*
     * So I could look at the array.
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);
    println!("{}", array[3]);
    println!("{}", array[4]);
    */

    println!("Enter an array index: ");
    let mut array_index = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut array_index)
        .expect("Error: Cannot read string.");

    let array_index: usize = array_index
        .trim()
        .parse()
        .expect("Error: Cannot convert string into u32 integer type.");

    println!("Array index array[{}] = {}", array_index, array[array_index]);

} 
