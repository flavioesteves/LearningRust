use std::io;

fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    //  borrow_string(&input);
    //  own_string(input);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {} kg", mars_weight);
}

fn borrow_string(s: &String) {
    println!("borrow_string: {}", s);
}

fn own_string(s: String) {
    println!("own_string:  {}", s);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
