#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[allow(dead_code)]
fn swirly_manhattan(val: i32) -> i32 {
    // this is how
    let square_root_min = (val as f32).cbrt().floor() as i32;
    let square_root = square_root_min.pow(3);
    let diff = val - square_root;
    let m = diff % square_root_min;
    let sides = diff/square_root_min;
    println!("square_root: {}, square_root_min: {}, diff: {}, mod: {}, sides: {}", square_root, square_root_min, diff, m, sides);
    let result = square_root - ((m-1)/2);
    result
}

#[cfg(test)]
mod tests {
    use day03::swirly_manhattan;

    #[test]
    fn test_swirly_manhattan_1_is_0() {
        assert_eq!(swirly_manhattan(1), 0)
    }

    #[test]
    fn test_swirly_manhattan_12_is_3() {
        assert_eq!(swirly_manhattan(12), 3)
    }

    #[test]
    fn test_swirly_manhattan_23_is_2() {
        assert_eq!(swirly_manhattan(23), 2)
    }

    #[test]
    fn test_swirly_manhattan_1024_is_31() {
        assert_eq!(swirly_manhattan(1024), 31)
    }

    // FIXME: delete before committing
    #[test]
    fn test_swirly_manhattan() {
    }

}