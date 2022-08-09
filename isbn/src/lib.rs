use std::str::FromStr;

#[derive(Debug)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    InputTooLong,
    InputTooShort,
    ChecksumFailed,
    InvalidChar(usize, char)
}

impl FromStr for Isbn {
    type Err = IsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut isbn = Isbn {
            raw: String::from(s),
            digits: Vec::with_capacity(13)
        };

        // let mut num_digits = 0;
        // for i in 0..s.len()-1 {
        //     match s.chars().nth(i) {
        //         Some('0') => { num_digits += 1; isbn.digits.push(0_u8); },
        //         Some('1') => { num_digits += 1; isbn.digits.push(1_u8); },
        //         Some('2') => { num_digits += 1; isbn.digits.push(2_u8); },
        //         Some('3') => { num_digits += 1; isbn.digits.push(3_u8); },
        //         Some('4') => { num_digits += 1; isbn.digits.push(4_u8); },
        //         Some('5') => { num_digits += 1; isbn.digits.push(5_u8); },
        //         Some('6') => { num_digits += 1; isbn.digits.push(6_u8); },
        //         Some('7') => { num_digits += 1; isbn.digits.push(7_u8); },
        //         Some('8') => { num_digits += 1; isbn.digits.push(8_u8); },
        //         Some('9') => { num_digits += 1; isbn.digits.push(9_u8); },
        //         _ => {},
        //     }
        // }

        // if num_digits+1 < 13 { return Err(IsbnError::InputTooShort); }
        // if num_digits+1 > 13 { return Err(IsbnError::InputTooLong); }

        // another way of doing it
        // in this implementation we end up having passing the
        // last digit over to calculate_check_digit(), but in
        // that function the zip() will make it so it doesn't
        // go up to the last digit
        // (the previous implementation does not add the last
        // digit to the struct, which will work for the 1st impl
        // of calculate_check_digit())
        for (i, c) in s.char_indices() {
            match c {
                '-'         => continue,
                '0'..='9'   => isbn.digits.push(c.to_digit(10).unwrap() as u8),
                _           => return Err(IsbnError::InvalidChar(i, c)),
            }
        }

        if isbn.digits.len() < 13 { return Err(IsbnError::InputTooShort); }
        if isbn.digits.len() > 13 { return Err(IsbnError::InputTooLong); }

        if calculate_check_digit(&isbn.digits) != s.bytes().last().unwrap() as u8 {
            return Ok(isbn);
        } else {
            return Err(IsbnError::ChecksumFailed);
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    // let sum : u32 = digits.iter().enumerate()
    //     .map(|(i, x)| if (i+1) % 2 != 0 { return *x as u32; } else { return (*x * 3) as u32; })
    //     .sum();

    // another way of doing it (this works because zip only goes up
    // to the smallest of the two, therefore it ends up not checking
    // the last idgit of 'digits')
    const WEIGHTS : [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let sum : u32 = digits.iter()
        .zip(WEIGHTS.iter())
        .map(|(&x, &y)| (x * y) as u32)
        .sum();

    let result = 10 - (sum % 10);

    // if result == 10 {
    //     0
    // } else {
    //     result as u8
    // }

    // another way of doing the last step
    match result {
        10 => 0_u8,
        _ => result as u8,
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_correctly_calculate_check_digits() {
        let cases = [
            ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
            ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
        ];
    
        for (case, check) in cases.iter() {
            let actual = calculate_check_digit(case);
            println!("{:?} -> {}?  {}", &case, check, actual);
            assert_eq!(calculate_check_digit(case), *check)
        }
    }

    #[test]
    fn rust_in_action() {
        let _: Isbn = "978-3-16-148410-0".parse().unwrap();
    }
}
