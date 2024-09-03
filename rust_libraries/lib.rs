pub mod string_to_numbers {

    use std::io::{self, Write};

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
}
