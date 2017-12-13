#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[allow(dead_code)]
fn function(val: &str) -> i32 {
    val.len() as i32
}

#[cfg(test)]
mod tests {
    use dayxx::function;

    #[test]
    fn test_new() {
        let input = "1";
        assert_eq!(function(input), 1)
    }

}