pub fn median(list: Vec<f32>) -> Option<f32> {
    if list.is_empty() { return None; }
    if list.len() == 1 { return Some(list[0]); }

    let mut sorted_list = list.clone();
    sorted_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if sorted_list.len() % 2 == 0 { // even
        Some(
            ( sorted_list[sorted_list.len()/2 - 1] + 
              sorted_list[sorted_list.len()/2] ) / 2.0
        )
    }
    else { // odd
        Some(sorted_list[sorted_list.len()/2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn odd_not_sorted() {
        let list = vec![1.0, 5.0, 4.0];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn even_not_sorted() {
        let list = vec![3.0, 1.5, 8.8, 5.0];
        assert_eq!(median(list), Some(4.0));
    }
    #[test]
    fn odd_sorted() {
        let list = vec![1.0, 4.0, 5.0];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn even_sorted() {
        let list = vec![1.5, 3.0, 5.0, 8.8];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn empty() {
        let list = vec![];
        assert_eq!(median(list), None);
    }

    #[test]
    fn single() {
        let list = vec![5.8];
        assert_eq!(median(list), Some(5.8));
    }
}
