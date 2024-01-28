use std::io;
use std::io::Write;

fn main() {

    temperature_menu();
    print!("Enter menu choice: ");
    let menu_choice = string_to_integer();

        if menu_choice == 1 {
            print!("Enter fahrenheit value: ");
            let fahrenheit_temperature = string_to_floating_point();
            let fahrenheit_conversion = fahrenheit_to_celsius(fahrenheit_temperature);
            println!("{} Fahrenheit = {} Celsius", fahrenheit_temperature, fahrenheit_conversion);
        }
        else if menu_choice == 2 {
            print!("Enter celsius value: ");
            let celsius_temperature = string_to_floating_point();
            let celsius_conversion = celsius_to_fahrenheit(celsius_temperature);
            println!("{} Celsius = {} Fahrenheit", celsius_temperature, celsius_conversion);
        }
        else {
            println!("Please choose 1 or 2.");
        }

}

fn temperature_menu() {
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit ");
}

fn string_to_integer() -> i32 {

    let mut string_to_integer_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_to_integer_buffer)
        .expect("Error: Cannot read string.");

    let string_to_integer_buffer: i32 = string_to_integer_buffer
        .trim()
        .parse()
        .expect("Error: Cannot convert string to i32 integer.");

return string_to_integer_buffer;
}

fn string_to_floating_point() -> f32 {

    let mut string_to_floating_point_buffer = String::new();

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut string_to_floating_point_buffer)
        .expect("Error: Cannot read string.");

    let string_to_floating_point_buffer: f32 = string_to_floating_point_buffer
        .trim()
        .parse()
        .expect("Error: Cannot convert string to f32 floating_point.");

return string_to_floating_point_buffer;
}

fn fahrenheit_to_celsius(fahrenheit_value: f32) -> f32 {
    let celsius_value = (fahrenheit_value - 32.0) * 5.0 / 9.0;

return celsius_value;
}

fn celsius_to_fahrenheit(celsius_value: f32) -> f32 {
    let fahrenheit_value = (celsius_value * 9.0/5.0) + 32.0;

return fahrenheit_value;
}
