use std::io;

fn main() {
    println!("Enter the Weight : (kg)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("i got error");
    let weight: f32 = input.trim().parse().expect("failed to parse");
    println!("Input : {}", weight);
    
    let my_weight_on_mars: f32 = calculate_weight_on_mars(weight);
    println!("Weight on Mars : {}kg", my_weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}