#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[allow(dead_code)]
fn knot_hash(input: Vec<u32>, s: u32) -> u32 {
    let mut current_pos: u32 = 0;
    let mut skip_size: u32 = 0;
    let mut elements: Vec<u32> = Vec::new();
    for i in 0..s {
        elements.push(i);
    }
    let elements_len: u32 = elements.len() as u32;
    for length in input {
        for n in 0..(length / 2) {
            let temp_index = ((current_pos + n) % elements_len) as usize;
            let swap_index = ((current_pos + length - n - 1) % elements_len) as usize;
            let temp = elements[temp_index];
            elements[temp_index] = elements[swap_index];
            elements[swap_index] = temp;
        }
        current_pos += length + skip_size;
        skip_size += 1;
    }
    elements[0] as u32 * elements[1] as u32
}

#[allow(dead_code)]
fn knot_hash2(input: &str) -> String {
    let additional_lengths: Vec<u32> = vec![17, 31, 73, 47, 23];
    let mut chars = input.chars();
    let mut lengths: Vec<u32> = Vec::new();
    while let Some(ch) = chars.next() {
        lengths.push(ch as u32);
    }
    for i in additional_lengths.iter() {
        lengths.push(*i);
    }
    let mut elements: Vec<u32> = Vec::new();
    for i in 0..256 {
        elements.push(i);
    }
    let elements_len: u32 = elements.len() as u32;
    let mut current_pos: u32 = 0;
    let mut skip_size: u32 = 0;
    for _i in 0..64 {
        for length in lengths.clone() {
            for n in 0..(length / 2) {
                let temp_index = ((current_pos + n) % elements_len) as usize;
                let swap_index = ((current_pos + length - n - 1) % elements_len) as usize;
                let temp = elements[temp_index];
                elements[temp_index] = elements[swap_index];
                elements[swap_index] = temp;
            }
            current_pos += length + skip_size;
            skip_size += 1;
        }
    }
    let mut output: String = String::from("");
    for x in 0..16 {
        let mut v = 0;
        for y in 0..16 {
            v ^= elements[x * 16 + y];
        }
        output.push_str(&format!("{:01$x}", v, 2));
    }

    output
}

#[cfg(test)]
mod tests {
    use day10::knot_hash;
    use day10::knot_hash2;

    #[test]
    fn test_knot_hash() {
        assert_eq!(knot_hash(vec![3, 4, 1, 5], 5), 12)
    }

    #[test]
    fn test_knot_hash2_1() {
        assert_eq!(knot_hash2(""), "a2582a3a0e66e6e86e3812dcb672a272")
    }

    #[test]
    fn test_knot_hash2_2() {
        assert_eq!(knot_hash2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd")
    }

    #[test]
    fn test_knot_hash2_3() {
        assert_eq!(knot_hash2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d")
    }

    #[test]
    fn test_knot_hash2_4() {
        assert_eq!(knot_hash2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e")
    }
}