/* Programmer:          Per Stoor
 * Date:                2024-02-10
 * Last changed:        2024-02-10
 * Type of program:      
 */

#[derive(Debug)]
struct Rectangle {

    width:  u32,
    height: u32,

}

/*
 * Method of the struct Rectangle.
 * */
impl Rectangle {
    fn calculate_area(&self) -> u32 {

        let function_result = self.width * self.height;
        
return function_result;
    }
}

fn main(){ 

    let user_rectangle = Rectangle {
        width:  5,
        height: 8,
    };

    let area = user_rectangle
        .calculate_area();

    println!("The area is: {}", area);
} 
