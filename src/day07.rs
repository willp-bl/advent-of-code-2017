#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Write;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Program {
    name: String,
    weight: i32,
    supporting: Vec<String>
}

fn parse_circus(data: &str) -> HashSet<Program> {
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
    programs_map
}

#[allow(dead_code)]
fn find_root(data: &str) -> String {
    let programs_map = parse_circus(data);
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

fn calc_weight(programs_map: &HashSet<Program>, node_name: String) -> i32 {
    let node = get_program_for_node(programs_map, node_name).unwrap();
    let mut weight = node.weight;
    for p in node.supporting {
        weight += calc_weight(programs_map, p);
    }
    weight
}

fn get_program_for_node(programs_map: &HashSet<Program>, node: String) -> Option<Program> {
    for n in programs_map {
        if n.name == node {
            return Some(Program { name: n.name.clone(), weight: n.weight, supporting: n.supporting.clone() });
        }
    }
    None
}

#[allow(dead_code)]
fn find_corrected_weight(data: &str, root: &str) -> i32 {
    _find_corrected_weight(data, root, 0)
}

#[allow(dead_code)]
fn _find_corrected_weight(data: &str, root: &str, neighbouring_total_weight: i32) -> i32 {
    let programs_map = parse_circus(data);
    let root_node = get_program_for_node(&programs_map, String::from(root)).unwrap();

    // get the calculated weights for all the supporting programs
    let mut weights: HashMap<i32, Vec<Program>> = HashMap::new();
    for p in root_node.supporting {
        let node = get_program_for_node(&programs_map, String::from(p)).unwrap();
        let total_weight = calc_weight(&programs_map, node.name.clone());
        let old_weights = weights.clone();
        let new_node = Program { name: node.name.clone(), weight: node.weight, supporting: node.supporting.clone() };
        if old_weights.contains_key(&total_weight) {
            let mut old_val = old_weights.get(&total_weight).unwrap().clone();
            old_val.push(new_node);
            weights.insert(total_weight, old_val);
        } else {
            let mut v = Vec::new();
            v.push(new_node);
            weights.insert(total_weight, v);
        }
    }
    if weights.keys().count() != 1 {
        // one of the supporting programs is unbalanced
        let mut unbalanced = "foo";
        let mut balanced_weight = 0;
        for k in weights.keys() {
            if weights.get(k).unwrap().len() == 1 {
                // this is the unbalanced program
                unbalanced = &weights.get(k).unwrap().iter().next().unwrap().name;
            } else {
                // this is the correct calculated weight of a neighbouring program
                balanced_weight = *k;
            }
        }
        return _find_corrected_weight(data, unbalanced, balanced_weight);
    } else {
        // all the sub programs are balanced, which means the root node is not
        let mut w = 0;
        for k in weights.keys() {
            w = *k;
        }
        let calc = neighbouring_total_weight - w * (weights.get(&w).unwrap().iter().len() as i32);
        return calc;
    }
}

#[cfg(test)]
mod tests {
    use day07::find_root;
    use day07::find_corrected_weight;

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

    #[test]
    fn test_finding_corrected_weight() {
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

        assert_eq!(find_corrected_weight(test_data, "tknk"), 60)
    }
}