use chrono::prelude::*;

struct ImportantEvent {
    what: String,
    when: Date<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when <= Local::today()
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.ymd(2022, 12, 25),
    };

    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("🎅");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn not_deadline() {
        let event = ImportantEvent {
            what: "event".to_string(),
            when: Local::today() + Duration::days(30),
        };

        assert_eq!(false, event.is_passed());
    }

    #[test]
    fn is_deadline() {
        let event = ImportantEvent {
            what: "event".to_string(),
            when: Local::today(),
        };

        assert_eq!(true, event.is_passed());
    }

    #[test]
    fn past_deadline() {
        let event = ImportantEvent {
            what: "event".to_string(),
            when: Local::today() - Duration::days(1),
        };

        assert_eq!(true, event.is_passed());
    }
}