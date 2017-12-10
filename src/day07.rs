#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Program {
    name: String,
    weight: i32,
    supporting: Vec<String>
}

#[allow(dead_code)]
fn find_root(data: &str) -> String {
    let mut programs_map: HashSet<Program> = HashSet::new();
    for row in data.split("\n") {
        let program_name = &row[0..row.find(" ").unwrap()];
        let weight = &row[row.find("(").unwrap() + 1..row.find(")").unwrap()];
        let mut programs: Vec<String> = Vec::new();
        if row.find("-> ").is_some() {
            let supporting = &row[row.find("-> ").unwrap() + 3..];
            for val in supporting.split(" ") {
                programs.push(val.replace(",", "").to_lowercase());
            }
        }
        let p = Program { name: program_name.to_lowercase(), weight: weight.parse().unwrap(), supporting: programs };
        programs_map.insert(p);
    }
    let mut possible_roots: HashSet<String> = HashSet::new();
    for p in programs_map.iter() {
        if p.supporting.is_empty() { continue; }
        possible_roots.insert(p.name.to_lowercase());
    }
    let mut to_remove: Vec<String> = Vec::new();
    for p in programs_map {
        if p.supporting.is_empty() { continue; }
        for r in possible_roots.iter() {
            if p.supporting.contains(&r) {
                to_remove.push(r.to_lowercase());
            }
        }
    }
    for r in to_remove {
        possible_roots.remove(&r);
    }
    if possible_roots.len() != 1 { return String::from("error"); }
    possible_roots.iter().next().unwrap().to_lowercase()
}

#[cfg(test)]
mod tests {
    use day07::find_root;

    #[test]
    fn test_finding_root() {
        let test_data = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(find_root(test_data), "tknk")
    }
}