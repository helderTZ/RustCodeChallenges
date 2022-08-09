pub fn encode(text: &str) -> String {
    let mut encoded_string = String::with_capacity(text.len() * 2);
    let mut count = 1;
    for (c1, c2) in text.chars().zip(text[1..text.len()].chars()) {
        if c1 == c2 && count < 9{
            count += 1;
            continue;
        } else {
            encoded_string += &count.to_string();
            encoded_string.push(c1);
            count = 1;
        }
    }

    let last_char = text.chars().nth(text.len()-1).unwrap();
    if count > 1 {
        encoded_string += &count.to_string();
        encoded_string.push(last_char);
    } else {
        encoded_string.push('1');
        encoded_string.push(last_char);
    }

    encoded_string
}

pub fn decode(text: &str) -> String {
    let mut decoded_string = String::with_capacity(text.len() / 2);

    for i in (0..text.len()).step_by(2) {
        let reps = text[i..i+1].parse::<i32>().unwrap();
        for _ in 0..reps {
            decoded_string.push(text.chars().nth(i+1).unwrap());
        }
    }

    // another way of iterating
    // let mut chars = text.chars();   // get an iterator
    // while let (Some(n), Some(c)) = (chars.next(), chars.next()) {   // iterate twice
    //     let reps = n.to_digit(10).unwrap();
    //     for _ in 0..reps {
    //         decoded_string.push(c);
    //     }
    // }

    decoded_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        assert_eq!(encode("abc"), "1a1b1c");
    }
    
    #[test]
    fn round_trip() {
        let input = "LinkedIn";
        println!("{}", encode(input));
        assert_eq!(decode(&encode(input)), input);
    }
    
    #[test]
    fn long_run() {
        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
        assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
    }
}
