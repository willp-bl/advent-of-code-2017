#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[allow(dead_code)]
fn function(val: i32) -> i32 {
    val
}

#[cfg(test)]
mod tests {
    use dayxx::function;

    #[test]
    fn test_new() {
        assert_eq!(function(1), 1)
    }

}