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
    let power = square_root_min.pow(2); // we know the location of this square
    let square_side_length = if power.eq(&val) { square_root_min - 1 } else { square_root_min + 1 };
    let length_remaining_after_power = val - power;
    let square_sides = if length_remaining_after_power > 0 { length_remaining_after_power / square_side_length } else { 0 };
    let half_height = (square_side_length) / 2;
    let remaining_length = if square_side_length > 0 { length_remaining_after_power % square_side_length } else { 0 };
//    println!("power: {}, square_root_min: {}, remaining_length: {}, square_side_length: {}, square_sides: {}, half_height: {}", power, square_root_min, remaining_length, square_side_length, square_sides, half_height);
    let x;
    if square_sides == 0 {
        x = half_height;
    } else if square_sides == 2 {
        x = -half_height
    } else if square_sides == 1 {
        x = half_height - remaining_length;
    } else if square_sides == 3 {
        x = remaining_length - half_height;
    } else {
        x = 0;
    };

    let y;
    if square_sides == 3 {
        y = -half_height;
    } else if square_sides == 1 {
        y = half_height;
    } else if square_sides == 2 {
        y = half_height - remaining_length
    } else if square_sides == 0 {
        y = remaining_length - half_height
    } else {
        y = 0
    }
//    println!("x: {}, y: {}", x, y);
    Point { x: x, y: y }
}

#[allow(dead_code)]
fn swirly_manhattan2_get_value(max: i32) -> i32 {
    swirly_manhattan2(1, max)
}

#[allow(dead_code)]
fn swirly_manhattan2(next: i32, max: i32) -> i32 {
    let mut i = next;
    let mut manhattan = vec![vec![0; 32]; 32];
    let mut sum = 0;
    while i <= max {
        let point = swirly_manhattan_point(i);
        let x = point.x + (32 / 2);
        let y = point.y + (32 / 2);
        sum = if i == 1 {
            1
        } else {
            let mut total = 0;
            for x2 in -1..2 {
                for y2 in -1..2 {
                    total += manhattan[(x + x2) as usize][(y + y2) as usize];
                }
            }
            total
        };
        println!("sum: {}", sum);
        manhattan[x as usize][y as usize] = sum;
        if i == max {
            break;
        } else {
            i += 1;
        }
    }
    return sum;
}

#[allow(dead_code)]
fn get_next_biggest_swirly_manhattan(max: i32) -> i32 {
    let mut result = 0;
    let mut i = 0;
    while result <= max {
        result = swirly_manhattan2(1, i);
        if result > max { break }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use day03::Point;
    use day03::swirly_manhattan;
    use day03::swirly_manhattan2_get_value;
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
    fn test_swirly_manhattan_point_16() {
        assert_eq!(swirly_manhattan_point(16), Point { x: -1, y: 2 })
    }

    #[test]
    fn test_swirly_manhattan_point_20() {
        assert_eq!(swirly_manhattan_point(20), Point { x: -2, y: -1 })
    }

    #[test]
    fn test_swirly_manhattan_point_5() {
        assert_eq!(swirly_manhattan_point(5), Point { x: -1, y: 1 })
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
        assert_eq!(swirly_manhattan2_get_value(1), 1)
    }

    #[test]
    fn test_swirly_manhattan2_2_is_1() {
        assert_eq!(swirly_manhattan2_get_value(2), 1)
    }

    #[test]
    fn test_swirly_manhattan2_3_is_2() {
        assert_eq!(swirly_manhattan2_get_value(3), 2)
    }

    #[test]
    fn test_swirly_manhattan2_4_is_4() {
        assert_eq!(swirly_manhattan2_get_value(4), 4)
    }

    #[test]
    fn test_swirly_manhattan2_5_is_5() {
        assert_eq!(swirly_manhattan2_get_value(5), 5)
    }

    #[test]
    fn test_swirly_manhattan2_6_is_10() {
        assert_eq!(swirly_manhattan2_get_value(6), 10)
    }

    #[test]
    fn test_swirly_manhattan2_7_is_11() {
        assert_eq!(swirly_manhattan2_get_value(7), 11)
    }

    #[test]
    fn test_swirly_manhattan2_21_is_362() {
        assert_eq!(swirly_manhattan2_get_value(21), 362)
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