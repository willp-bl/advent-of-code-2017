#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
struct RegisterResult {
    max_at_end: i32,
    max_at_anytime: i32,
}

#[allow(dead_code)]
fn process(data: &str) -> RegisterResult {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_at_anytime = 0;
    for instruction in data.split("\n") {
        let command = &instruction[0..instruction.find(" if ").unwrap()];
        let mut com = command.split(" ");
        let com_reg = com.next().unwrap();
        let com_instruction = com.next().unwrap();
        let com_val: i32 = com.next().unwrap().parse().unwrap();
        let condition = &instruction[(instruction.find(" if ").unwrap() + 4)..];
        let mut cond = condition.split(" ");
        let cond_reg = cond.next().unwrap();
        let cond_check = cond.next().unwrap();
        let cond_val: i32 = cond.next().unwrap().parse().unwrap();
        if !registers.contains_key(cond_reg) {
            registers.insert(String::from(cond_reg), 0);
        }
        if !registers.contains_key(com_reg) {
            registers.insert(String::from(com_reg), 0);
        }
        let val = registers.get(cond_reg).unwrap().clone();
        let mut cont = false;
        match cond_check.as_ref() {
            ">" => { if val > cond_val { cont = true; } }
            "<" => { if val < cond_val { cont = true; } }
            ">=" => { if val >= cond_val { cont = true; } }
            "<=" => { if val <= cond_val { cont = true; } }
            "==" => { if val == cond_val { cont = true; } }
            "!=" => { if val != cond_val { cont = true; } }
            _ => { println!("instruction error: {}", cond_check); }
        }
        if cont {
            let cur_val = registers.get(com_reg).unwrap().clone();
            match com_instruction.as_ref() {
                "inc" => { registers.insert(String::from(com_reg), cur_val + com_val); }
                "dec" => { registers.insert(String::from(com_reg), cur_val - com_val); }
                _ => { println!("instruction error: {}", com_instruction); }
            }
            if registers.values().max().unwrap() > &max_at_anytime {
                max_at_anytime = *registers.values().max().unwrap();
            }
        }
    }
    let mut max_val: i32 = 0;
    let mut max_reg_name = "";
    for reg in registers.keys() {
        if registers.get(reg).unwrap() > &max_val {
            max_val = *registers.get(reg).unwrap();
            max_reg_name = reg;
        }
    }
    RegisterResult { max_at_end: max_val, max_at_anytime: max_at_anytime }
}

#[cfg(test)]
mod tests {
    use day08::process;

    #[test]
    fn test_register() {
        let data = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(process(data).max_at_end, 1)
    }

    #[test]
    fn test_register2() {
        let data = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(process(data).max_at_anytime, 10)
    }
}