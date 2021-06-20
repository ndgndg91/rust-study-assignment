pub mod practice;
use practice::{new_string, new_to_string, from_new_string, concant_three, concant_new_string,easy_concat};

pub fn string_practice() {
    let nam_dong_gil = new_string("남동길");
    println!("{}", nam_dong_gil);
    let ndgndg91 = new_to_string("ndgndg91");
    println!("{}", ndgndg91);
    let giri = from_new_string("giri");
    println!("{}", giri);
    let hello_world = concant_new_string("hello", " world!");
    println!("{}", hello_world);

    let giri = concant_new_string(&giri, " ");
    let hello_world = concant_new_string(&giri, &hello_world);
    println!("{}", hello_world);

    let nam_dong_gil = easy_concat(&nam_dong_gil, " ");
    let hello_world = easy_concat(&nam_dong_gil, &hello_world);
    println!("{}", hello_world);

    let last_result = concant_three(&ndgndg91, " ", &hello_world);
    println!("{}", last_result);
}