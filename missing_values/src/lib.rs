
pub fn sum_with_missing(values: Vec<Option<i32>>) -> i32 {
    // let mut sum : i32 = 0;
    // for i in values.into_iter() {
    //     match i {
    //         Some(i) => sum += i,
    //         None => {},
    //     }
    // }
    // sum

    values.iter()
        .map(|x| x.unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_missing() {
        let values : Vec<Option<i32>> = vec![Some(4), Some(1)];
        assert_eq!(sum_with_missing(values), 5);
    }

    #[test]
    fn all_missing() {
        let values : Vec<Option<i32>> = vec![None, None, None];
        assert_eq!(sum_with_missing(values), 0);
    }

    #[test]
    fn mixed() {
        let values : Vec<Option<i32>> = vec![Some(4), None, Some(1)];
        assert_eq!(sum_with_missing(values), 5);
    }

    #[test]
    fn empty() {
        let values : Vec<Option<i32>> = vec![];
        assert_eq!(sum_with_missing(values), 0);
    }
}