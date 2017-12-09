#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

fn char_to_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

#[allow(dead_code)]
fn captcha(input: &str) -> u32 {
    let mut result = 0;
    let mut iter = input.trim().chars().peekable();
    while let Some(c) = iter.next() {
        match iter.peek() {
            Some(p) => {
                if c.eq(p) {
                    result += char_to_digit(c);
                }
            }
            None => {
                if input.starts_with(c) {
                    result += char_to_digit(c);
                }
            }
        }
    }
    result
}

#[allow(dead_code)]
fn captcha2(input: &str) -> u32 {
    let mut result = 0;
    let trimmed_input = input.trim();
    let length = trimmed_input.len();
    if length % 2 != 0 { return 2000000; }
    let shift = length / 2;
    for i in 0..length {
        match trimmed_input.get(i..i + 1) {
            Some(c) => {
                let current_char = c.chars().nth(0).unwrap();
                let shift_char_index;
                if i + shift < length {
                    shift_char_index = i + shift;
                } else {
                    shift_char_index = (i + shift) - length;
                }
                let shift_char = trimmed_input.get(shift_char_index..shift_char_index + 1).unwrap().chars().nth(0).unwrap();
                if current_char.eq(&shift_char) {
                    result += char_to_digit(current_char);
                }
            }
            None => {}
        }
    }
    result
}

#[cfg(not(test))]
#[allow(dead_code)]
fn main() {
    print!("enter captcha: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            println!("result: {}", captcha(&input));
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use day01::char_to_digit;
    use day01::captcha;
    use day01::captcha2;

    #[test]
    fn test_digit_conversion() {
        assert_eq!(char_to_digit('0'), 0)
    }

    #[test]
    fn test_captcha1_1122_is_3() {
        assert_eq!(captcha("1122"), 3);
    }

    #[test]
    fn test_captcha1_1111_is_4() {
        assert_eq!(captcha("1111"), 4);
    }

    #[test]
    fn test_captcha1_1234_is_0() {
        assert_eq!(captcha("1234"), 0);
    }

    #[test]
    fn test_captcha1_91212129_is_9() {
        assert_eq!(captcha("91212129"), 9);
    }

    #[test]
    fn test_captcha2_1212_is_6() {
        assert_eq!(captcha2("1212"), 6);
    }

    #[test]
    fn test_captcha2_1221_is_0() {
        assert_eq!(captcha2("1221"), 0);
    }

    #[test]
    fn test_captcha2_123425_is_4() {
        assert_eq!(captcha2("123425"), 4);
    }

    #[test]
    fn test_captcha2_123123_is_12() {
        assert_eq!(captcha2("123123"), 12);
    }

    #[test]
    fn test_captcha2_12131415_is_4() {
        assert_eq!(captcha2("12131415"), 4);
    }
}