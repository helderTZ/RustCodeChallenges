pub fn unique<T: Ord>(vec: &mut Vec<T>) -> &mut Vec<T> {
    if vec.len() == 0 { return vec; }

    vec.sort();
    vec.dedup();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicates() {
        let mut vec = vec![1, 2, 3];
        assert_eq!(unique(&mut vec), &[1, 2, 3]);
    }

    #[test]
    fn with_duplicates() {
        let mut vec = vec![1, 2, 1];
        assert_eq!(unique(&mut vec), &[1, 2]);
    }

    #[test]
    fn empty() {
        let mut vec : Vec<u64> = vec![];
        assert_eq!(unique(&mut vec), &[]);
    }
}
