use std::io;
use std::io::Write;

fn main() {

    introduction();

    print!("Enter the n:th number: ");
    let user_number = string_to_integer();

    // I manually create the first three numbers of the Fibonacci sequence.
    let mut fibonacci_vector: Vec<usize> = Vec::new();
    fibonacci_vector.push(0);
    fibonacci_vector.push(1);
    fibonacci_vector.push(1);

    // If the user only asks for the first three number the first if-statement prints them out.
    // If the user asks for the fourth number or higher the else if-statement prints them out.
    let mut while_loop_counter1: usize = 0;
    while while_loop_counter1 <= user_number {
            if while_loop_counter1 < 3 {
                print!("{} ", fibonacci_vector[while_loop_counter1]);
                while_loop_counter1 += 1;
            }
            else if while_loop_counter1 > 2 {
                let next_fibonacci_value = fibonacci_vector[while_loop_counter1 - 1] + fibonacci_vector[while_loop_counter1 - 2];
                fibonacci_vector.push(next_fibonacci_value);
                print!("{} ", fibonacci_vector[while_loop_counter1]);
                while_loop_counter1 += 1;
            }

    }

    // Empty println! macro for formatting purposes.
    println!();

}

fn introduction() {
    println!("This program will generate an n:th number of the Fibonacci sequence.");
    println!("Please enter an indexing number of which Fibonacci sequence number you wish to see.");
}

fn string_to_integer() -> usize {

    let mut string_to_integer_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_to_integer_buffer)
        .expect("Error: Cannot read string");

    let string_to_integer_buffer: usize = string_to_integer_buffer
        .trim()
        .parse()
        .expect("Error: Cannot convert string to i32 integer.");

return string_to_integer_buffer;
}
