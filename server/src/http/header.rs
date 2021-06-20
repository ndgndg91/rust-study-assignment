use std::collections::HashMap;

#[derive(Debug)]
pub struct Header<'buf> {
    pub headers: HashMap<&'buf str, &'buf str>
}

impl<'buf> From<&'buf str> for Header<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut headers = HashMap::new();
    
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                continue;
            }

            if line.is_empty() {
                break;
            }

            let mut key = "";
            let mut val = "";

            if let Some(index) = line.find(": ") {
                key = &line[..index];
                val = &line[index + 1..];
            }
        
            headers.insert(key, val);
        }

        Self{
            headers
        }
    }
}