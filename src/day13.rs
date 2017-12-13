#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

fn parse_scan(input: &str) -> HashMap<i32, i32> {
    let mut scan: HashMap<i32, i32> = HashMap::new();
    for row in input.split("\n") {
        let layer = &row[0..row.find(":").unwrap()];
        let depth = &row[(row.find(":").unwrap() + 1)..];
        scan.insert(layer.trim().parse().unwrap(), depth.trim().parse().unwrap());
    }
    let max = *scan.keys().max().unwrap() + 1;
    for i in 0..max {
        match scan.contains_key(&i) {
            true => {
                // nothing
            }
            false => {
                scan.insert(i, 0);
            }
        }
    }
    scan
}

#[allow(dead_code)]
fn calc_trip_severity(input: &str) -> i32 {
    _calc_trip_severity(input, 0)
}

#[allow(dead_code)]
fn calc_safe_trip_start_time(input: &str) -> i32 {
    let mut offset = 0;
    loop {
        let result = _calc_trip_severity(input, offset);
        println!("offset: {}, result: {}", offset, result);
        if result == 0 {
            break;
        } else {
            offset += 1;
        }
        if offset > 500000 { break }
    }
    offset
}

#[allow(dead_code)]
fn _calc_trip_severity(input: &str, start_picosecond: i32) -> i32 {
    let scan = parse_scan(input);
    let mut severity = 0;
    let print = true;
    let max = *scan.keys().max().unwrap() + 1;
    for picosecond in start_picosecond..max + start_picosecond {
        let layer = &picosecond - start_picosecond;
        match scan.get(&layer) {
            Some(depth) => {
                if print { println!("picosecond: {}, layer: {}, depth: {}", picosecond, layer, depth); }
                if *depth != 0 {
                    let up = (picosecond / (depth - 1)) % 2 == 1;
                    let d2;
                    if up {
                        d2 = depth - 1 - picosecond % (depth - 1);
                    } else {
                        d2 = picosecond % (depth - 1);
                    }
                    if print {
                        for i in 0..*depth {
                            if i == d2 {
                                print!("x");
                            } else {
                                print!(".");
                            }
                        }
                    }

                    if d2 == 0 {
                        severity += depth * layer;
                    }
                } else {
                    if print { print!("."); }
                }
            }
            None => {}
        }
        if print { println!(); }
    }
    severity
}

#[cfg(test)]
mod tests {
    use day13::calc_trip_severity;
    use day13::calc_safe_trip_start_time;

    #[test]
    fn test_calc_trip_severity_1() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(calc_trip_severity(input), 24)
    }

    #[test]
    #[ignore]
    fn test_calc_safe_trip_start_time_1() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(calc_safe_trip_start_time(input), 10)
    }


}