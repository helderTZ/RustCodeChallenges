trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

type Message = Vec<Letter>;

type Letter = Vec<Pulse>;

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;

        let mut msg = Vec::with_capacity(self.len());

        for c in self.chars() {
            let pulses = match c {
                'A' | 'a'   => vec![Short, Long],
                'B' | 'b'   => vec![Long, Short, Short, Short],
                'C' | 'c'   => vec![Long, Short, Long, Short],
                'D' | 'd'   => vec![Long, Short, Short],
                'E' | 'e'   => vec![Short],
                'F' | 'f'   => vec![Short, Short, Long, Short],
                'G' | 'g'   => vec![Long, Long, Short],
                'H' | 'h'   => vec![Short, Short, Short, Short],
                'I' | 'i'   => vec![Short, Short],
                'J' | 'j'   => vec![Short, Long, Long, Long],
                'K' | 'k'   => vec![Long, Short, Long],
                'L' | 'l'   => vec![Short, Long, Short, Short],
                'M' | 'm'   => vec![Long, Long],
                'N' | 'n'   => vec![Long, Short],
                'O' | 'o'   => vec![Long, Long, Long],
                'P' | 'p'   => vec![Short, Long, Long, Short],
                'Q' | 'q'   => vec![Long, Long, Short, Long],
                'R' | 'r'   => vec![Short, Long, Short],
                'S' | 's'   => vec![Short, Short, Short],
                'T' | 't'   => vec![Long],
                'U' | 'u'   => vec![Short, Short, Long],
                'V' | 'v'   => vec![Short, Short, Short, Long],
                'W' | 'w'   => vec![Short, Long, Long],
                'X' | 'x'   => vec![Long, Short, Short, Long],
                'Y' | 'y'   => vec![Long, Short, Long, Long],
                'Z' | 'z'   => vec![Long, Long, Short, Short],
                '1'         => vec![Short, Long, Long, Long, Long],
                '2'         => vec![Short, Short, Long, Long, Long],
                '3'         => vec![Short, Short, Short, Long, Long],
                '4'         => vec![Short, Short, Short, Short, Long],
                '5'         => vec![Short, Short, Short, Short, Short],
                '6'         => vec![Long, Short, Short, Short, Short],
                '7'         => vec![Long, Long, Short, Short, Short],
                '8'         => vec![Long, Long, Long, Short, Short],
                '9'         => vec![Long, Long, Long, Long, Short],
                '0'         => vec![Long, Long, Long, Long, Long],
                _           => continue,
            };
            msg.push(pulses);
        }

        msg
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        use Pulse::*;

        let expected = vec![
            /* H */ vec![Short, Short, Short, Short],
            /* e */ vec![Short],
            /* l */ vec![Short, Long, Short, Short],
            /* l */ vec![Short, Long, Short, Short],
            /* o */ vec![Long, Long, Long],
            /* w */ vec![Short, Long, Long],
            /* o */ vec![Long, Long, Long],
            /* r */ vec![Short, Long, Short],
            /* l */ vec![Short, Long, Short, Short],
            /* d */ vec![Long, Short, Short],
        ];

        let actual = "Hello, world".to_string().to_morse_code();
        assert_eq!(actual, expected);

        print!("Hello, world : ");
        print_morse_code(&actual);
    }

    #[test]
    fn whole_alphabet() {
        let alphabet = "abcdefghijklmnopqrstuvwxyz1234657890".to_string();

        alphabet.to_morse_code();
        let code = alphabet.to_uppercase().to_morse_code();

        print!("{} : ", alphabet);
        print_morse_code(&code);
    }

}