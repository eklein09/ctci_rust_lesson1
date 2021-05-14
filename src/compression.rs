fn string_compression(s: &str) -> String {
    let mut compressed = String::new();
    let mut current_char: char = s.chars().nth(0).unwrap();
    let mut chars_encountered: u32 = 1;
    for (i,c) in s.chars().enumerate() {
        if i == 0 {
            continue;
        } else {
            if c == current_char {
                chars_encountered += 1;
            } else {
                compressed.push(current_char);
                    compressed.push(std::char::from_digit(chars_encountered,10).unwrap());
                current_char = c;
                chars_encountered = 1;
            }
        }
    }

    compressed.push(current_char);
    compressed.push(std::char::from_digit(chars_encountered,10).unwrap());
    compressed
}

#[cfg(test)]
#[test]
fn test_compression() {
    let result = string_compression("aabcccccaaa");
    println!("{}",result);
    assert_eq!(result,"a2b1c5a3");
}