use std::io;
// Main function is the entry point for Rust program
// Runs first
// Naming conventation is snake_case

fn main() {
    //println!("Hello, world!"); //chars
    //println!("Number: {}", 100); // {} is replaced with the second argument
    //println!("Number: {}, String: {}", 100, "String");

    // How to create empty string
    let mut input = String::new();
    //-- L19 Start --
    //let mut s = input; //Rule 3 example
    //-- L19 End --
    //-- L20 Start --
    //let s1 = &input;
    //let s2 = &mut input;
    //println!("{} {}", s1, s2);
    //-- L20 End --

    io::stdin().read_line(&mut input);
    print!("Text input: {}", input);
    let input_weight: f32 = 100.0;
    let mut mars_weight = calculate_weight_on_mars(input_weight);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
