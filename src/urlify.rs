use regex::Regex;

pub fn urlify(a: &str) -> String {
    let re = Regex::new(" ").unwrap();
    re.replace_all(a.trim(), "%20").into_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn urlify_john_smith() {
        assert_eq!(super::urlify("Mr John Smith "), "Mr%20John%20Smith")
    }
}

