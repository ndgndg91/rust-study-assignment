use std::fmt::Display;

pub struct VectorWrapper {
    vec: Vec<i32>,
}

impl VectorWrapper {
    pub fn new() -> Self {
        let mut vector = vec![];
        Self { vec: vector }
    }

    pub fn push(&mut self, value: i32) {
        self.vec.push(value);
    }
}

impl Display for VectorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut display = String::new();
        for value in &self.vec {
            let string_value = value.to_string();
            display.push_str(&string_value);
            display.push(',');
        }

        display.remove(display.len() - 1);
        write!(f, "{}", display)

    }
}
