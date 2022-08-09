use chrono::NaiveDate;

fn parse_3_letter_month(text: &str) -> Option<u32> {
    let months : [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "oct", "sep", "nov", "dec"
    ];
    let text = text.to_lowercase();

    for (i, m) in months.iter().enumerate() {
        if m == &text {
            return Some((i as u32) + 1);
        }
    }

    None
}

fn has_3letter_month(text: &str) -> bool {
    let months : [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "oct", "sep", "nov", "dec"
    ];
    let text = text.to_lowercase();

    for m in months {
        if text.contains(m) {
            return true;
        }
    }

    false
}

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|x| x.is_ascii_digit())
}

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`. 
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    let delimiter: char;
    if text.contains("/") {
        delimiter = '/';
    } else if text.contains("-") {
        delimiter = '-';
    } else if text.contains(".") {
        delimiter = '.';
    } else {
        return None;
    }
    let parts = text.split(delimiter);

    let has_3letter_months = has_3letter_month(&text);

    let mut year : Option<i32> = None;
    let mut month : Option<u32> = None;
    let mut day : Option<u32> = None;

    for p in parts {
        if has_3letter_months && month.is_none() && p.len() == 3 {
            month = parse_3_letter_month(p);
        } else if year.is_none() && p.len() == 4 {
            year = p.parse::<i32>().ok();
        } else if !has_3letter_months &&  month.is_none() && p.len() == 2 {
            month = Some(p.parse::<u32>().unwrap());
        } else if day.is_none() && p.len() == 2 {
            day = p.parse::<u32>().ok();
        }
    }

    if year.is_none() || month.is_none() || day.is_none() {
        return None;
    }

    Some(NaiveDate::from_ymd(year.unwrap(), month.unwrap(), day.unwrap()))

    // // another way to implement

    // // getting the individual fields
    // let fields : Vec<_> = text
    //     .split(['/', '-', '.', ' '].as_slice())
    //     .collect();

    // let mut year : Option<i32> = None;
    // let mut month : Option<u32> = None;
    // let mut day : Option<u32> = None;

    //  //getting the month if in letters
    // for field in fields.iter() {
    //     if field.len() < 3 {
    //         continue;
    //     }

    //     let m = match &field.to_lowercase()[..3] {
    //         "jan" => 1,
    //         "feb" => 2,
    //         "mar" => 3,
    //         "apr" => 4,
    //         "may" => 5,
    //         "jun" => 6,
    //         "jul" => 7,
    //         "aug" => 8,
    //         "sep" => 9,
    //         "oct" => 10,
    //         "nov" => 11,
    //         "dev" => 12,
    //         _ => continue,
    //     };

    //     month = Some(m);
    // }

    //  //getting rest of the fields
    // for field in fields.iter() {
    //     if is_year(field) {
    //         year = field.parse::<i32>().ok();
    //         continue;
    //     }

    //     if month.is_some() {
    //         day = field.parse::<u32>().ok();
    //     } else {
    //         month = field.parse::<u32>().ok();
    //     }
    // }

    // match (year, month, day) {
    //     (Some(y), Some(m), Some(d)) => {
    //         NaiveDate::from_ymd_opt(y, m, d)
    //     },
    //     (Some(y), Some(m), None) => {
    //         NaiveDate::from_ymd_opt(y, m, 1)
    //     },
    //     _ => None,
    // }
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
}

#[test]
fn dmy_dot() {
    assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
}

#[test]
fn mdy_dot() {
    assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}