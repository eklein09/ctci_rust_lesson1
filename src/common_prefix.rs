fn common_prefix(a: String, b: String) -> String {
    let mut common = String::new();
    for (a_char,b_char) in a.chars().zip(b.chars()) {
        if a_char == b_char {
            common.push(a_char);
        } else {
            break;
        }
    }
    common
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    match strs.len() {
        0 => String::from(""),
        _ => strs.into_iter().reduce(|a,b| {common_prefix(a,b)}).expect("empty").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = vec![String::from("flower"),String::from("flow"),String::from("flight")];
        assert_eq!(longest_common_prefix(strs),"fl");
    }

    #[test]
    fn test2() {
        let strs = vec![String::from("aa"),String::from("b"),String::from("c")];
        assert_eq!(longest_common_prefix(strs),"");
    }
}