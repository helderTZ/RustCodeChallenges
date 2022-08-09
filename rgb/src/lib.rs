use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
enum RgbError {
    InvalidLength,
    InvalidChar,
    InvalidLeadChar,
}

trait RgbChannels {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 { self.r }
    fn b(&self) -> u8 { self.g }
    fn g(&self) -> u8 { self.b }
}

impl FromStr for Rgb {
    type Err = RgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.chars().nth(0).unwrap() != '#' { return Err(RgbError::InvalidLeadChar); }

        let no_prefix = s.trim_start_matches("#");

        if no_prefix.len() != 6 { return Err(RgbError::InvalidLength); }

        let r = u8::from_str_radix(&no_prefix[0..2], 16);
        let g = u8::from_str_radix(&no_prefix[2..4], 16);
        let b = u8::from_str_radix(&no_prefix[4..6], 16);

        Ok(Rgb {
            r : r.or_else(|_| Err(RgbError::InvalidChar))?,
            g : g.or_else(|_| Err(RgbError::InvalidChar))?,
            b : b.or_else(|_| Err(RgbError::InvalidChar))?,
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_color() {
        let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

        for ((r, g), b) in colors {
            let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let color: Rgb = hex.parse().unwrap();
            assert_eq!(hex, format!("{}", color));
        }
    }

    #[test]
    #[should_panic]
    fn too_short () {
        let _: Rgb = "1234".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn not_a_hex_code () {
        let _: Rgb = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_literals () {
        let _: Rgb = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn no_leading_hash() {
        let _: Rgb = "aabbcc".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let _: Rgb = "00gg00".parse().unwrap();
    }
}
