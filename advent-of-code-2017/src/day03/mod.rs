#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
fn swirly_manhattan(val: i32) -> i32 {
    let point = swirly_manhattan_point(val);
    point.x.abs() + point.y.abs()
}

#[allow(dead_code)]
fn swirly_manhattan_point(val: i32) -> Point {
    let square_root_min = (val as f32).sqrt().floor() as i32;
    let square_root_min = if square_root_min % 2 == 1 {
        square_root_min
    } else {
        square_root_min - 1
    };
    let power = square_root_min.pow(2);
    let half_height = (square_root_min + 1) / 2;
    let modulus = (val - power) % (square_root_min + 1);
    println!("power: {}, square_root_min: {}, modulus: {}", power, square_root_min, modulus);
    let x = square_root_min % half_height;
    let z = (square_root_min - 1) / 2;
    let y = if modulus > z { modulus - z } else { z };
    println!("x: {}, y: {}", x, y);
    //FIXME: calculate actual location from origin (+/-) on each axis
    Point { x: x, y: y }
}

#[allow(dead_code)]
fn swirly_manhattan2(square: i32) -> i32 {
    // use swirly_manhattan to get coordinates,
    // then use coordinates to calculate values around the current one
    // by storing it in an array (ugh)
    if square.eq(&1) { return 1; }
    square
}

#[allow(dead_code)]
fn get_next_biggest_swirly_manhattan(value: i32) -> i32 {
    let mut square = 0;
    let mut result = 0;
    while result <= value {
        square += 1;
        result = swirly_manhattan2(square);
    }
    result
}

#[cfg(test)]
mod tests {
    use day03::Point;
    use day03::swirly_manhattan;
    use day03::swirly_manhattan2;
    use day03::swirly_manhattan_point;

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
    fn test_swirly_manhattan_9_is_2() {
        assert_eq!(swirly_manhattan(9), 2)
    }

    #[test]
    fn test_swirly_manhattan_25_is_4() {
        assert_eq!(swirly_manhattan(25), 4)
    }

    #[test]
    fn test_swirly_manhattan_21_is_4() {
        assert_eq!(swirly_manhattan(21), 4)
    }

    #[test]
    fn test_swirly_manhattan_point_1() {
        assert_eq!(swirly_manhattan_point(1), Point { x: 0, y: 0 })
    }

    #[test]
    fn test_swirly_manhattan_point_12() {
        assert_eq!(swirly_manhattan_point(12), Point { x: 2, y: 1 })
    }

    #[test]
    fn test_swirly_manhattan_point_23() {
        assert_eq!(swirly_manhattan_point(23), Point { x: 0, y: -2 })
    }

    #[test]
    fn test_swirly_manhattan_point_9() {
        assert_eq!(swirly_manhattan_point(9), Point { x: 1, y: -1 })
    }

    #[test]
    fn test_swirly_manhattan_point_25() {
        assert_eq!(swirly_manhattan_point(25), Point { x: 2, y: -2 })
    }

    #[test]
    fn test_swirly_manhattan_point_21() {
        assert_eq!(swirly_manhattan_point(21), Point { x: -2, y: -2 })
    }

    #[test]
    fn test_swirly_manhattan_1024_is_31() {
        assert_eq!(swirly_manhattan(1024), 31)
    }

    #[test]
    fn test_swirly_manhattan2_1_is_1() {
        assert_eq!(swirly_manhattan2(1), 1)
    }

    #[test]
    fn test_swirly_manhattan2_2_is_1() {
        assert_eq!(swirly_manhattan2(2), 1)
    }

    #[test]
    fn test_swirly_manhattan2_3_is_2() {
        assert_eq!(swirly_manhattan2(3), 2)
    }

    #[test]
    fn test_swirly_manhattan2_4_is_4() {
        assert_eq!(swirly_manhattan2(4), 4)
    }

    #[test]
    fn test_swirly_manhattan2_5_is_5() {
        assert_eq!(swirly_manhattan2(5), 5)
    }

    #[test]
    fn test_swirly_manhattan2_6_is_10() {
        assert_eq!(swirly_manhattan2(6), 10)
    }

    #[test]
    fn test_swirly_manhattan2_7_is_11() {
        assert_eq!(swirly_manhattan2(7), 11)
    }

    #[test]
    fn test_swirly_manhattan2_21_is_362() {
        assert_eq!(swirly_manhattan2(21), 362)
    }

    // FIXME: delete before committing
    #[test]
    fn test_swirly_manhattan() {
    }

    // FIXME: delete before committing
    #[test]
    fn test_swirly_manhattan2() {
    }
}