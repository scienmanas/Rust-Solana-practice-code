// Result
fn divide(a: f64, b: f:64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok( a / b)
    }
}

// Option
fn find_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i);
        }
    }
    None
}