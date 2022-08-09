// if T implements the Display trait,
// we can use it in println
pub fn info<T: std::fmt::Display>(text: &T) {
    println!("{}", text);
}

// problem: allocates memory
pub fn info2<T: ToString>(text: &T) {
    println!("{}", text.to_string());
}

// if T behaves as a slice, we can avoid
// allocating memory
pub fn info3<T: AsRef<str>>(text: &T) {
    println!("{}", text.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn using_string1() {
        let text = String::from("Hello from String, info");
        info(&text);
    }

    #[test]
    fn using_str1() {
        let text = "Hello from &str, info";
        info(&text);
    }

    #[test]
    fn using_string2() {
        let text = String::from("Hello from String, info");
        info2(&text);
    }

    #[test]
    fn using_str2() {
        let text = "Hello from &str, info";
        info2(&text);
    }

    #[test]
    fn using_string3() {
        let text = String::from("Hello from String, info");
        info3(&text);
    }

    #[test]
    fn using_str3() {
        let text = "Hello from &str, info";
        info3(&text);
    }
}
