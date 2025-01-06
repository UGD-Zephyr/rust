/* Programmer:          Per Stoor
 * Date:                2025-01-06
 * Last changed:        2025-01-06
 * Type of program:     Input: A vector `[1, 2, 3, 4, 5]`
 *                      Expected Output: `15` (sum of even numbers) (A.I generated...maybe a mistake here?)
 *                      Task: Use `fold` with a closure to sum only even numbers.
 *                      This exercise combines iteration, closures,
 *                      and conditionals for a more complex operation.
 */

use public_library::string_conversions::*;

fn main(){ 

    let mut sum: Vec<u8> = Vec::new();

    print!("Input size of vector: ");
    let vector_size = string_to_u8();

        for loop_counter in 0..vector_size {
            print!("Input element ({}) : ", loop_counter);
            let vector_element = string_to_u8();
                sum.push(vector_element);
        }
    let sum_of_vector_elements = sum.iter()
                                    .fold(0, |acc, &x| acc + x);

    println!("The finished vector:      {:?}", sum);
    println!("Sum of the vector:        {}", sum_of_vector_elements);
    //println!("Sum of only even numbers: {}", );
} 
