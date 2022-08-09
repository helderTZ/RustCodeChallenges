pub fn sort_usernames(usernames: &mut [&str]) {
    usernames.sort_by_cached_key(|x| { x.to_lowercase() });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut usernames = vec!["Todd", "amy"];
        sort_usernames(&mut usernames);
        assert_eq!(usernames, vec!["amy", "Todd"]);
    }
}
