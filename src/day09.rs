#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
struct StreamResult {
    score: i32,
    garbage_count: i32,
}

#[allow(dead_code)]
fn stream(val: &str) -> StreamResult {
    let mut depth = 0;
    let mut score = 0;
    let mut chars = val.chars();
    let mut garbage_zone = false;
    let mut garbage_count = 0;
    while let Some(ch) = chars.next() {
        match ch {
            '{' => { if !garbage_zone { depth += 1; } else { garbage_count += 1 } }
            '}' => {
                if !garbage_zone {
                    score += depth;
                    depth -= 1;
                } else { garbage_count += 1 }
            }
            '!' => {
                chars.next();
            }
            '<' => { if !garbage_zone { garbage_zone = true; } else { garbage_count += 1 } }
            '>' => { garbage_zone = false; }
            _ => { if garbage_zone { garbage_count += 1 } }
        }
    }
    StreamResult { score: score, garbage_count: garbage_count }
}

#[cfg(test)]
mod tests {
    use day09::stream;

    #[test]
    fn test_stream1() {
        assert_eq!(stream("{}").score, 1)
    }

    #[test]
    fn test_stream2() {
        assert_eq!(stream("{{{}}}").score, 6)
    }

    #[test]
    fn test_stream3() {
        assert_eq!(stream("{{},{}}").score, 5)
    }

    #[test]
    fn test_stream4() {
        assert_eq!(stream("{{{},{},{{}}}}").score, 16)
    }

    #[test]
    fn test_stream5() {
        assert_eq!(stream("{<a>,<a>,<a>,<a>}").score, 1)
    }

    #[test]
    fn test_stream6() {
        assert_eq!(stream("{{<ab>},{<ab>},{<ab>},{<ab>}}").score, 9)
    }

    #[test]
    fn test_stream7() {
        assert_eq!(stream("{{<!!>},{<!!>},{<!!>},{<!!>}}").score, 9)
    }

    #[test]
    fn test_stream8() {
        assert_eq!(stream("{{<a!>},{<a!>},{<a!>},{<ab>}}").score, 3)
    }

    #[test]
    fn test_garbage1() {
        assert_eq!(stream("<>").garbage_count, 0)
    }

    #[test]
    fn test_garbage2() {
        assert_eq!(stream("<random characters>").garbage_count, 17)
    }

    #[test]
    fn test_garbage3() {
        assert_eq!(stream("<<<<>").garbage_count, 3)
    }

    #[test]
    fn test_garbage4() {
        assert_eq!(stream("<{!>}>").garbage_count, 2)
    }

    #[test]
    fn test_garbage5() {
        assert_eq!(stream("<!!>").garbage_count, 0)
    }

    #[test]
    fn test_garbage6() {
        assert_eq!(stream("<!!!>>").garbage_count, 0)
    }

    #[test]
    fn test_garbage7() {
        assert_eq!(stream("<{o\"i!a,<{i<a>").garbage_count, 10)
    }
}