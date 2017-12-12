#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
struct PlumbingResult {
    linked_programs_from_zero: i32,
    groups: i32,
}

fn find_linked_programs(index: i32, programs: &HashMap<i32, Vec<i32>>) -> HashSet<i32> {
    let mut linked_programs: HashSet<i32> = HashSet::new();
    for i in programs.get(&index).unwrap().iter() {
        linked_programs.insert(*i);
    }
    loop {
        let mut new_programs: HashSet<i32> = HashSet::new();
        for i in linked_programs.clone() {
            for j in programs.get(&i).unwrap().iter() {
                new_programs.insert(*j);
            }
        }
        if new_programs.difference(&linked_programs).count() == 0 {
            break;
        } else {
            for i in new_programs.iter() {
                linked_programs.insert(*i);
            }
        }
    }
    linked_programs
}

#[allow(dead_code)]
fn plumber(input: &str) -> PlumbingResult {
    let mut programs: HashMap<i32, Vec<i32>> = HashMap::new();

    // parse
    for program in input.split("\n") {
        let command: i32 = program[0..program.find("<->").unwrap()].trim().parse().unwrap();
        let mut links: Vec<i32> = Vec::new();
        for link in program[(program.find("<->").unwrap() + 3)..].trim().split(" ") {
            links.push(link.trim().replace(",", "").parse().unwrap());
        }
        programs.insert(command, links);
    }

    // process programs linked from zero
    let mut ignore_programs: HashSet<i32> = HashSet::new();
    let linked_programs_from_zero = find_linked_programs(0, &programs);

    // find groupings
    let mut all_programs: HashSet<i32> = HashSet::new();
    for i in programs.clone().keys() {
        all_programs.insert(*i);
    }

    for i in linked_programs_from_zero.clone() {
        ignore_programs.insert(i);
    }

    let mut groups = 1;
    loop {
        let ip = ignore_programs.clone();
        let mut left = all_programs.difference(&ip);
        let linked_programs;
        match left.next() {
            Some(i) => {
                linked_programs = find_linked_programs(*i, &programs);
            }
            None => {
                break;
            }
        }
        if linked_programs.len() > 0 {
            groups += 1;
        }
        for j in linked_programs {
            ignore_programs.insert(j);
        }
    }

    PlumbingResult { linked_programs_from_zero: linked_programs_from_zero.len() as i32, groups: groups }
}

#[cfg(test)]
mod tests {
    use day12::plumber;

    #[test]
    fn test_plumber_1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        let result = plumber(input);
        assert_eq!(result.linked_programs_from_zero, 6);
        assert_eq!(result.groups, 2);
    }
}