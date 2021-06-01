mod vector;
mod string;
use vector::practice::VectorWrapper;
use string::practice::{new_string, new_to_string, from_new_string, concant_new_string, easy_concat, concant_three};

fn main() {
    // vector_practice();
    string_practice();
}

fn string_practice() {
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

#[allow(dead_code)]
fn vector_practice() {
    let mut wrapper = VectorWrapper::new();
    wrapper.push(0);
    wrapper.push(1);
    wrapper.push(10);

    println!("{:?}", wrapper);
    
    let third: i32 = wrapper.get(2).unwrap();
    println!("{}", third);

    let mut string_wrapper = VectorWrapper::new();
    string_wrapper.push(String::from("ndgndg91"));
    string_wrapper.push("giri".to_owned());
    string_wrapper.push("nam-dong-gil".to_string());

    println!("{:?}", string_wrapper);
}
