pub fn from_new_string<'a>(str: &'a str) -> String {
    String::from(str)
}

pub fn new_string<'a>(str: &'a str) -> String {
    let mut value = String::new();
    value.push_str(str);
    return value;
}

pub fn new_to_string<'a>(str: &'a str) -> String {
    str.to_string()
}

pub fn concant_new_string<'a, 'b>(str1: &'a str, str2: &'b str) -> String {
    let mut str = String::from(str1);
    str.push_str(str2);
    return str;
}

pub fn easy_concat<'a, 'b>(str1: &'a str, str2: &'b str) -> String {
    String::from(str1) + str2
}

pub fn concant_three<'a>(str1: &'a str, str2: &'a str, str3: &'a str) -> String {
    format!("{}{}{}", str1, str2, str3)
}