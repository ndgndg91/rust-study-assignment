mod vector;
use vector::practice::VectorWrapper;

fn main() {
    vector_practice();
}

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

    println!("{:?}", string_wrapper);
}
