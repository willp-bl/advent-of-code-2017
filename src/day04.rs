#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use std::iter::FromIterator;

#[allow(dead_code)]
fn is_passphrase_valid(val: &str) -> bool {
    let mut words: HashSet<String> = HashSet::new();
    for word in val.split(" ") {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted_char_string = String::from_iter(chars);
        if words.contains(&sorted_char_string) { return false; }
        words.insert(sorted_char_string);
    }
    true
}

#[cfg(test)]
mod tests {
    use day04::is_passphrase_valid;

    #[test]
    fn test_valid() {
        assert_eq!(is_passphrase_valid("aa bb cc dd ee"), true)
    }

    #[test]
    fn test_invalid() {
        assert_eq!(is_passphrase_valid("aa bb cc dd aa"), false)
    }

    #[test]
    fn test_valid2() {
        assert_eq!(is_passphrase_valid("aa bb cc dd aaa"), true)
    }

    #[test]
    fn count_valid() {
        let data = "";
        let mut count = 0;
        for row in data.split("\n") {
            if is_passphrase_valid(row) { count += 1 }
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn test_anagram_valid3() {
        assert_eq!(is_passphrase_valid("abcde fghij"), true)
    }

    #[test]
    fn test_anagram_invalid3() {
        assert_eq!(is_passphrase_valid("abcde xyz ecdab"), false)
    }

    #[test]
    fn test_anagram_valid4() {
        assert_eq!(is_passphrase_valid("a ab abc abd abf abj"), true)
    }

    #[test]
    fn test_anagram_valid5() {
        assert_eq!(is_passphrase_valid("iiii oiii ooii oooi oooo"), true)
    }

    #[test]
    fn test_anagram_invalid4() {
        assert_eq!(is_passphrase_valid("oiii ioii iioi iiio"), false)
    }
}