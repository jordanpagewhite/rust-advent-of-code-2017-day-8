use std::collections::HashMap;
use evalexpr::{eval};

fn main() {
    let input = include_str!("../data/input.txt");
    let mut vars: HashMap<String, isize> = HashMap::new();
    vars.insert(String::from("greatest_during_process"), 0);

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let operation_var = parts[0];
        let operation_type = parts[1].to_string();
        let operation_value = parts[2].to_string().parse::<isize>().unwrap();
        // nobody cares about the "if" in parts[3].
        let conditional_var = parts[4];
        let conditional_type = parts[5].to_string();
        let conditional_value = parts[6].to_string().parse::<isize>().unwrap();
        // If we do not already have this key/val in our HashMap, create it.
        vars.entry(operation_var.to_string()).or_default();
        vars.entry(conditional_var.to_string()).or_default();
        // Evaluate the conditional expression provided in the instruction.
        if eval(&format!("{} {} {}", vars[conditional_var], conditional_type.as_str(), conditional_value)).unwrap().as_boolean().unwrap() {
            match operation_type.as_str() {
                "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                _ => unreachable!()
            }
        }
        // Part 2: Keep track of the greatest value while processing the instructions.
        let greatest_during_process = vars.iter().max_by(|a, b| a.1.cmp(&b.1));

        if *greatest_during_process.unwrap().1 > vars["greatest_during_process"] {
            *vars.get_mut("greatest_during_process").unwrap() = *greatest_during_process.unwrap().1;
        }
    }

    println!("greatest during process: {}", vars["greatest_during_process"]);
    vars.remove("greatest_during_process");
    let greatest_el = vars.iter().max_by(|a, b| a.1.cmp(&b.1));
    println!("greatest element at end: {}", greatest_el.unwrap().1);
}
