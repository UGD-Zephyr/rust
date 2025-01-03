/* Programmer:          Per Stoor
 * Date:                2025-01-01
 * Last changed:        2025-01-01
 * Type of program:     Input: An array of integers [1, 2, 3, 4, 5]
                        Expected Output: [1, 4, 9, 16, 25] (squared values)
                        Task: Use a for loop to mutate array elements in place.
                        This exercise shows how to use mutable references and
                        loops for modifying data.
 */

fn main(){ 

    // This integer needs to be mutated since the variable
    // will change inside the for-loop
    let mut integer_array = [1, 2, 3, 4, 5];
    println!("Original integer array: {:?}", integer_array);
        // Notice the [&mut syntax] before the variable. This is
        // because we need to modify the elements of the array inside
        // the for-loop
        for element in &mut integer_array {
            // We need to dereference the element variable because if we don't
            // all we are changing is the address of where the value is actually stored.
            // to actually change the vale, this dereferencing syntax is necessary.
            *element = *element * *element;
        }
    println!("Mutated integer_array: {:?}", integer_array);
} 
