use regex::Regex;

pub fn urlify(a: &str) -> String {
    let re = Regex::new(" ").unwrap();
    re.replace_all(a.trim(), "%20").into_owned()
}