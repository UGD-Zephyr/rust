/* Programmer:          Per Stoor
 * Date:                2024-12-31
 * Last changed:        2024-12-31
 * Type of program:     Input a vector of integers [1, 2, 3, 4, 5]
 *                      Use iter() to sum elements without moving them,
 *                      demonstrating how to work with references to 
 *                      avoid ownership issues. 
 *                      This exercise helps understand iteration over 
 *                      collections in Rust.
 */

fn main(){ 

    let mut vector_of_integers: Vec<i32> = Vec::new();
        for integers in 1..=5 {
            vector_of_integers.push(integers);
        }

    let sum_of_integers: i32 = vector_of_integers.iter().sum();
    println!("Original vector: {:?}", vector_of_integers);
    println!("Sum: {}", sum_of_integers);

/*
 * The point here is that we can still use the vector after summing
 * it up.
 * */

} 
