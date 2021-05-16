extern crate itertools;

use itertools::Itertools;

fn string_compression(s: &str) -> String {
    let mut output = String::new();

    for (key, group) in &s.chars().group_by(|y| *y) {
        output.push(key);
        output.push(std::char::from_digit(group.count() as u32, 10).unwrap());
    }
    if output.len() > s.len() {
        String::from(s)
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compression() {
        let result = super::string_compression("aabcccccaaa");
        println!("{}", result);
        assert_eq!(result, "a2b1c5a3");
    }
    
    #[test]
    fn test_compression2() {
        let result = super::string_compression("aaa");
        println!("{}", result);
        assert_eq!(result, "a3");
    }
    
    #[test]
    fn test_compression3() {
        let result = super::string_compression("a");
        println!("{}", result);
        assert_eq!(result, "a");
    }
}
