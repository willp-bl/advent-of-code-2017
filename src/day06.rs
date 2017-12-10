#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
struct AllocatorResult {
    reallocations: i32,
    cycles_for_infinite_loop: i32,
}

#[allow(dead_code)]
fn reallocator(banks: &mut Vec<i32>) -> AllocatorResult {
    let mut reallocations = 0;
    let bank_count: i32 = banks.len() as i32;
    let mut previous_banks: HashMap<Vec<i32>, i32> = HashMap::new();
    previous_banks.insert(banks.clone(), reallocations);
    loop {
        reallocations += 1;
        let mut max: i32 = 0;
        for i in 0..banks.len() {
            if banks[i] > banks[max as usize] {
                max = i as i32
            }
        }
        let old_bank_size = banks[max as usize];
        banks[max as usize] = 0;
        for i in 0..old_bank_size {
            banks[((max + i + 1) % bank_count) as usize] += 1;
        }
        if previous_banks.contains_key(banks) { break; }
        previous_banks.insert(banks.clone(), reallocations);
    }
    AllocatorResult { reallocations: reallocations, cycles_for_infinite_loop: reallocations - previous_banks[banks] }
}

#[cfg(test)]
mod tests {
    use day06::reallocator;

    #[test]
    fn test_reallocator() {
        let mut banks: Vec<i32> = vec![0, 2, 7, 0];
        assert_eq!(reallocator(&mut banks).reallocations, 5)
    }

    #[test]
    fn test_reallocator2() {
        let mut banks: Vec<i32> = vec![0, 2, 7, 0];
        assert_eq!(reallocator(&mut banks).cycles_for_infinite_loop, 4)
    }
}