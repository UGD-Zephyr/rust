/* Programmer:      Per Stoor
 * Date:            2024-02-10
 * Last changed:    2024-02-10
 * Type of program: More struct examples.
 */

#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

fn main(){ 

    let user_rectangle = Rectangle {
        width:  20,
        height: 15,
    };

    let rectangle_area = calculate_area(&user_rectangle);
    println!("The area of the rectangle is {} units.", rectangle_area);

    println!("debugged user_rectangle: {:#?}", user_rectangle);

} 

fn calculate_area(rectangle: &Rectangle) -> u32{

    let result = rectangle.width * rectangle.height;

return result;
}
