
struct Temperature {
    degrees: f32,
    scale: Scale,
}

enum Scale {
    Celsius,
    Fahrenheit,
}

impl Temperature {
    fn new(degrees: f32, scale: Scale) -> Self {
        Self { degrees, scale }
    }

    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit =>  (self.degrees - 32.0) * (5.0/9.0),
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * (9.0/5.0) + 32.0,
            Scale::Fahrenheit =>  self.degrees,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_fahrenheit() {
        let celsius = Temperature::new(0.0, Scale::Celsius);
        assert_eq!(32.0, celsius.to_fahrenheit());
    }

    #[test]
    fn fahrenheit_to_celsius() {
        let fahrenheit = Temperature::new(32.0, Scale::Fahrenheit);
        assert_eq!(0.0, fahrenheit.to_celsius());
    }
}
