use std::collections::HashMap;

fn main() {
    // Part 1: 1
    // Part 2: 10
    // let input = include_str!("../data/example.txt");
    // Part 1: 6343
    // Part 2: 7184
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

        // If we do not already have this "variable" in our HashMap, create it.
        if !vars.contains_key(operation_var) {
            vars.insert(operation_var.to_string(), 0);
        }
        // If we do not already have this "variable" in our HashMap, create it.
        if !vars.contains_key(conditional_var) {
            vars.insert(conditional_var.to_string(), 0);
        }
        // We now can be certain that the "variables" include in the operation and conditional
        // portion of this instruction exist in our `vars` HashMap.
        match conditional_type.as_str() {
            ">" => {
                if vars[conditional_var] > conditional_value {
                    match operation_type.as_str() {
                        "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                        "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                        _ => {}
                    }
                }
            },
            "<" => {
                if vars[conditional_var] < conditional_value {
                    match operation_type.as_str() {
                        "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                        "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                        _ => {}
                    }
                }
            },
            ">=" => {
                if vars[conditional_var] >= conditional_value {
                    match operation_type.as_str() {
                        "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                        "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                        _ => {}
                    }
                }
            },
            "<=" => {
                if vars[conditional_var] <= conditional_value {
                    match operation_type.as_str() {
                        "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                        "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                        _ => {}
                    }
                }
            },
            "==" => {
                if vars[conditional_var] == conditional_value {
                    match operation_type.as_str() {
                        "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                        "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                        _ => {}
                    }
                }
            },
            "!=" => {
                if vars[conditional_var] != conditional_value {
                    match operation_type.as_str() {
                        "inc" => *vars.get_mut(operation_var).unwrap() += operation_value,
                        "dec" => *vars.get_mut(operation_var).unwrap() -= operation_value,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        // Part 2
        let greatest_during_process = vars.iter()
            .max_by(|a, b| a.1.cmp(&b.1));

        if *greatest_during_process.unwrap().1 > vars["greatest_during_process"] {
            *vars.get_mut("greatest_during_process").unwrap() = *greatest_during_process.unwrap().1;
        }
    }

    let greatest_el = vars.iter()
        .max_by(|a, b| a.1.cmp(&b.1));
        //.map(|(k, _v)| k);

    println!("greatest during process: {:?}", vars["greatest_during_process"]);
    println!("{:?}", greatest_el.unwrap());
}
