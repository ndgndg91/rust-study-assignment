mod vector;
use vector::practice::VectorWrapper;

fn main() {
    let mut wrapper = VectorWrapper::new();
    wrapper.push(0);
    wrapper.push(1);
    wrapper.push(10);

    println!("{}", wrapper);
}
