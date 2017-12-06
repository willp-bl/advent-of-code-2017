#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[allow(dead_code)]
fn checksum(data: &str) -> i32 {
    let mut checksum = 0;
    for row in data.split("\n") {
        let mut vec: Vec<i32> = Vec::new();
        for entry in row.split("\t") {
            match entry.trim().parse() {
                Ok(value) => {
                    vec.push(value);
                }
                Err(error) => {
                    println!("error: {}", error);
                }
            }
        }
        checksum += vec.iter().max().unwrap() - vec.iter().min().unwrap();
    }
    checksum
}


#[allow(dead_code)]
fn checksum2(data: &str) -> i32 {
    let mut checksum = 0;
    for row in data.split("\n") {
        let mut vec: Vec<i32> = Vec::new();
        for entry in row.split("\t") {
            match entry.trim().parse() {
                Ok(value) => {
                    vec.push(value);
                }
                Err(error) => {
                    println!("error: {}", error);
                }
            }
        }
        let mut found = false;
        for x in vec.iter() {
            if found { break; }
            for y in vec.iter() {
                if x <= y { continue; }
                let val: f32 = (*x as f32) / (*y as f32);
                if val.floor().eq(&val) {
                    if val <= 0.0 || val.is_infinite() { continue; }
                    checksum += val as i32;
                    found = true;
                    break;
                }
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use day02::checksum;
    use day02::checksum2;

    #[test]
    fn test_checksum() {
        let test_data = "5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8";
        assert_eq!(checksum(test_data), 18)
    }

    #[test]
    fn test_checksum2() {
        let test_data = "5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5";
        assert_eq!(checksum2(test_data), 9)
    }

}