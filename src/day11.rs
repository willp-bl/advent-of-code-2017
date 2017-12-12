#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct HexedResult {
    max_distance: i32,
    velocity: i32,
}

fn calc_distance(a:&Point, b:&Point) -> i32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() +
        (a.x + a.y - b.x - b.y).abs()) / 2
}

#[allow(dead_code)]
fn hexed(val: &str) -> HexedResult {
    let origin = Point { x:0, y:0 };
    let mut current_point = Point { x:0, y:0 };
    let mut max_distance = 0;
    for step in val.split(",") {
        match step.as_ref() {
            "n" => { current_point.y -= 1; }
            "ne" => { current_point.x += 1; current_point.y -= 1; }
            "se" => { current_point.x += 1; }
            "s" => { current_point.y += 1; }
            "sw" => { current_point.x -= 1; current_point.y += 1; }
            "nw" => { current_point.x -= 1; }
            _i => { println!("unknown direction: {}", _i)}
        }
        let current_distance = calc_distance(&origin, &current_point);
        if current_distance>max_distance {
            max_distance = current_distance;
        }
    }
    println!("origin: {:?}, current_point: {:?}", origin, current_point);
    HexedResult { max_distance: max_distance, velocity: calc_distance(&origin, &current_point) }
}

#[cfg(test)]
mod tests {
    use day11::hexed;

    #[test]
    fn test_hexed_1() {
        assert_eq!(hexed("ne,ne,ne").velocity, 3)
    }

    #[test]
    fn test_hexed_2() {
        assert_eq!(hexed("ne,ne,sw,sw").velocity, 0)
    }

    #[test]
    fn test_hexed_3() {
        assert_eq!(hexed("ne,ne,s,s").velocity, 2)
    }

    #[test]
    fn test_hexed_4() {
        assert_eq!(hexed("se,sw,se,sw,sw").velocity, 3)
    }

}