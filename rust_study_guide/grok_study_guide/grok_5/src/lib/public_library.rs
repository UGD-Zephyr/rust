/* Programmer:          Per Stoor
 * Date:                2024-10-12
 * Last changed:        2024-10-17
 * Type of program:     Personal library file for rust modules.
 */

/*
 * Module containing string conversion functions.
 * */
pub mod string_conversions {

    use std::io::{self, Write};

        // function for reading a string.
        // maybe work on the logic of this function some more...a lot of ChatGPT suggestions in
        // here and there could be a better way.
        pub fn input_to_string () -> String {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            let trimmed_string_buffer = string_buffer.trim();
                if !trimmed_string_buffer.is_empty() {
                    return trimmed_string_buffer.to_string();
                }
                else {
                    println!("Error: Empty or invalid input");
                }
            };

    }
        // function for string to i8 conversions.
        pub fn string_to_i8 () -> i8 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<i8>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to i16 conversions.
        pub fn string_to_i16 () -> i16 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<i16>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to i32 conversions.
        pub fn string_to_i32 () -> i32 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<i32>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }
        
        // function for string to i64 conversions.
        pub fn string_to_i64 () -> i64 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<i64>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to i128 conversions.
        pub fn string_to_i128 () -> i128 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<i128>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to isize conversions.
        pub fn string_to_isize () -> isize {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<isize>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to u8 conversions.
        pub fn string_to_u8 () -> u8 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<u8>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to u16 conversions.
        pub fn string_to_u16 () -> u16 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<u16>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to u32 conversions.
        pub fn string_to_u32 () -> u32 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<u32>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to u64 conversions.
        pub fn string_to_u64 () -> u64 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<u64>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to u128 conversions.
        pub fn string_to_u128 () -> u128 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<u128>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to usize conversions.
        pub fn string_to_usize () -> usize {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<usize>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to f32 conversions.
        pub fn string_to_f32 () -> f32 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<f32>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

        // function for string to f64 conversions.
        pub fn string_to_f64 () -> f64 {
            let mut string_buffer = String::new();

            io::stdout()
                .flush()
                .expect("Error: Failed flushing stdout");

            let string_buffer = loop {
                string_buffer.clear();

            io::stdin()
                .read_line(&mut string_buffer)
                .expect("Error: Failed to read string from stdin");

            match string_buffer
                .trim()
                .parse::<f64>() {
                    Ok(parse_success)   => break parse_success,
                    Err(_)              => println!("Error: Did not enter a number"),
                };
            };

        return string_buffer;
    }

}

/*
 * Module containing system call functions.
 * */
pub mod system_calls {

    use std::process::Command;

        pub fn clear_screen(){
            if cfg!(unix){
            let _ = Command::new("clear")
                .status();
        }
        else if cfg!(windows){
            let _ = Command::new("cmd")
                .arg("/c")
                .arg("cls")
                .status();
        }
}
}
