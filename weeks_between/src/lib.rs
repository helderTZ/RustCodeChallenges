use chrono::NaiveDate;

pub fn weeks_between(a: &str, b: &str) -> i32 {
    let dt_a = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let dt_b = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();

    (dt_b - dt_a).num_weeks() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_week() {
        let a = "2022-07-27";
        let b = "2022-07-28";
        assert_eq!(weeks_between(a, b), 0);
    }

    #[test]
    fn positive_week() {
        let a = "2022-07-05";
        let b = "2022-07-28";
        assert_eq!(weeks_between(a, b), 3);
    }

    #[test]
    fn negative_week() {
        let a = "2022-07-25";
        let b = "2022-07-05";
        assert_eq!(weeks_between(a, b), -2);
    }
}
