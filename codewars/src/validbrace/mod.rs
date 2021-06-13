#[allow(dead_code)]
fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
          match c {
              '(' => stack.push(c),
              '{' => stack.push(c),
              '[' => stack.push(c),
              ')' => if stack.pop() != Some('(') {return false;},
              '}' => if stack.pop() != Some('{') {return false;},
              ']' => if stack.pop() != Some('[') {return false;},
              _   => panic!("Invalid input")
          }
      }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
        expect_true("([{}])");
        expect_false(")(}{][");
    }
    fn expect_true(s: &str) {
        assert!(
            valid_braces(s),
            "Expected {s:?} to be valid. Got false",
            s = s
        );
    }
    fn expect_false(s: &str) {
        assert!(
            !valid_braces(s),
            "Expected {s:?} to be invalid. Got true",
            s = s
        );
    }
}
