use std::collections::HashMap;
use evalexpr::{eval};

fn main() {
    let input = include_str!("../data/input.txt");
    let mut vars: HashMap<String, isize> = HashMap::new();
    vars.insert(String::from("greatest_during_process"), 0);

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // If we do not already have this key/val in our HashMap, create it.
        vars.entry(parts[0].to_string()).or_default();
        vars.entry(parts[4].to_string()).or_default();
        // Evaluate the conditional expression provided in the instruction.
        if eval(&format!("{} {} {}", vars[parts[4]], parts[5], parts[6].to_string().parse::<isize>().unwrap())).unwrap().as_boolean().unwrap() {
            match parts[1] {
                "inc" => *vars.get_mut(parts[0]).unwrap() += parts[2].to_string().parse::<isize>().unwrap(),
                "dec" => *vars.get_mut(parts[0]).unwrap() -= parts[2].to_string().parse::<isize>().unwrap(),
                _ => unreachable!()
            }
        }
        // Part 2: Keep track of the greatest value while processing the instructions.
        let greatest_during_process = vars.iter().max_by(|a, b| a.1.cmp(b.1));

        if *greatest_during_process.unwrap().1 > vars["greatest_during_process"] {
            *vars.get_mut("greatest_during_process").unwrap() = *greatest_during_process.unwrap().1;
        }
    }

    println!("Part 1 (greatest during process): {}", vars["greatest_during_process"]);
    vars.remove("greatest_during_process");
    let greatest_el = vars.iter().max_by(|a, b| a.1.cmp(b.1));
    println!("Part 2 (greatest element at end): {}", greatest_el.unwrap().1);
}
