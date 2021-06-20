pub mod practice;
pub use practice::VectorWrapper;

#[allow(dead_code)]
pub fn vector_practice() {
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
