use counter::Counter;

use std::collections::HashSet;

mod compression;
mod urlify;

fn main() {}

fn is_unique(s: &str) -> bool {
    let mut encountered: HashSet<char> = HashSet::new();
    let mut unique: bool = true;
    for c in s.chars() {
        if encountered.contains(&c) {
            unique = false;
            break;
        } else {
            encountered.insert(c);
        }
    }

    unique
}

fn is_anagram(a: &str, b: &str) -> bool {
    let count_a = a.chars().collect::<Counter<_>>();
    let count_b = b.chars().collect::<Counter<_>>();
    count_a == count_b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_unique() {
        assert_eq!(is_unique("abc"), true);
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(is_unique("bbc"), false);
    }

    #[test]
    fn anagram_true() {
        assert!(is_anagram("abc", "cba"));
    }

    #[test]
    fn anagram_false() {
        assert!(!is_anagram("ab", "cb"));
    }
}
