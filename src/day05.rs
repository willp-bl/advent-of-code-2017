#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[allow(dead_code)]
fn run(instructions: &mut Vec<i32>) -> i32 {
    let mut next_instruction = 0;
    let mut instruction_count = 0;
    loop {
        if next_instruction >= instructions.len() as i32 { break; }
        instruction_count += 1;
        let old_instruction = next_instruction;
        next_instruction = instructions[old_instruction as usize] + old_instruction;
        instructions[old_instruction as usize] += 1;
    }
    instruction_count
}

#[allow(dead_code)]
fn run2(instructions: &mut Vec<i32>) -> i32 {
    let mut next_instruction = 0;
    let mut instruction_count = 0;
    loop {
        if next_instruction >= instructions.len() as i32 { break; }
        instruction_count += 1;
        let old_instruction = next_instruction;
        next_instruction = instructions[old_instruction as usize] + old_instruction;
        if instructions[old_instruction as usize] >= 3 {
            instructions[old_instruction as usize] -= 1;
        } else {
            instructions[old_instruction as usize] += 1;
        }
    }
    println!("vec: {:?}", instructions);
    instruction_count
}

#[cfg(test)]
mod tests {
    use day05::run;
    use day05::run2;

    #[test]
    fn test_instructions() {
        let mut instructions: Vec<i32> = vec![0, 3, 0, 1, -3];
        assert_eq!(run(&mut instructions), 5)
    }

    #[test]
    fn test_instructions2() {
        let mut instructions: Vec<i32> = vec![0, 3, 0, 1, -3];
        assert_eq!(run2(&mut instructions), 10)
    }

    #[test]
    fn test_instructions3() {
//        assert_eq!(run2(&mut instructions), 5)
    }

}