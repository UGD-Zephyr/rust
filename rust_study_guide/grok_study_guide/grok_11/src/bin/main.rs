/* Programmer:          Per Stoor
 * Date:                2025-01-12
 * Last changed:        2025-01-12
 * Type of program:     Input: A struct `Point { x: i32, y: i32 }` with `x = 3,
 *                      y = 4`
 *                      Expected Output: `5` (Euclidean distance from origin)
 *                      Task: Define a struct and calculate distance,
 *                      introducing you to custom data structures and basic 
 *                      calculations in Rust.
 */

// needed for .sqrt() method
use std::f64;

// #[derive(Debug)] is needed to make {:?} work because my struct is created
// by me. For Strings rust has this built in
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main(){ 
    
    // Hard coded two points (3,4) and (0,0) because my exampe question didn't
    // require user input
    let coordinate1 = Point {x: 3, y: 4};
    let coordinate2 = Point {x: 0, y: 0};
    let result = euclidian_distance(&coordinate1, &coordinate2);
    println!("The euclidian distance between {:?} and {:?} is: {}", coordinate1,
                                                                    coordinate2,
                                                                    result);
} 

// You can't do sqrt() on integers, that's why the functions return value
// becomes f64. You also need to borrow the parameters
// because otherwise println!() won't work since the function 
// ate up the variables
fn euclidian_distance(first_point: &Point, second_point: &Point) -> f64 {

    // This is a bit mathy...but basically you break up the formula for
    // calculating euclidian distance in the below way.
    let dx = (first_point.x - second_point.x) as f64;
    let dy = (first_point.y - second_point.y)as f64;
    let distance: f64 = (dx * dx + dy * dy).sqrt();

return distance;
}
