use crate::{Type, Variable};

pub fn id(n: usize) -> Type {
    let mut result = vec![vec![0.;n]; n];

    for i in 0..n {
        result[i][i] = 1.;
    }

    Type::Matrix(result, (n, n))
}

pub fn diag(size: usize, shift: isize) -> Type {
    let mut result = vec![vec![0.; size]; size];
    
    for i in 0..(size- shift.abs() as usize) {
        let index = i + shift.abs() as usize;
        if shift > 0 {
            result[index as usize][i] = 1.;
        } else {
            result[i][index as usize] = 1.
        }
    }
    Type::Matrix(result, (size, size))
}

pub fn build_variable(input: String, var_list: &mut Vec<Variable>) {
    let parts: Vec<&str> = input.split(" ").collect();

    if !parts.contains(&"=") {
        println!("Variable assignment did not contain '=' operator.");
        return ();
    } else if parts[2] != "=" {
        println!("Variable names may not contain spaces.");
        return ();
    }

    let data: Vec<&str> = parts[3..].iter()
        .map(|wrd| *wrd).collect(); 

    let value = arg_type(data);
    let new_var = Variable::new(parts[1], value);

    println!("\n      {}\n", new_var);

    var_list.push(new_var);
}


fn arg_type(arg: Vec<&str>) -> Type {
    let invalid = Type::Error("Entered data was not a recognized type.".to_string());
    let first: String = arg[0].to_string();
    if arg.len() == 1 {
        if first == "true" {
            return Type::Bool(true);
        } else if first == "false" {
            return Type::Bool(false);
        } else if first.parse::<f64>().is_ok() {
            return Type::Number(first.parse::<f64>().unwrap());
        }
    } 

    let string: String = arg.iter()
        .map(|wrd| format!("{} ", wrd))
        .collect();

    if first.chars().nth(0).unwrap_or(' ') == '"' {
        let pure_string: Vec<String> = string.split('"')
            .map(|x| x.to_string())
            .collect();
        let final_str = pure_string.iter().nth(1).unwrap();

        return Type::String(final_str.to_owned());
    } 

    let no_space: String = string.chars().filter(|c| *c != ' ').collect();

    if valid_parens(&no_space) && no_space.chars().nth(0).unwrap_or(' ') == '[' {
        if no_space.chars().nth(1).unwrap_or(' ') == '[' {
            //Matrix
            let rows: Vec<String> = no_space.split('[')
                .filter(|r| !r.is_empty()) 
                .map(|str| str.chars()
                     .filter(|c| *c != ']')
                     .collect())
                .collect();
            let elements: Vec<Vec<f64>> = rows.iter()
                .map(|r| r.split(',')
                     .filter(|elem| !elem.is_empty())
                     .map(|val| val.parse::<f64>()
                          .unwrap_or(0.0 ))
                     .collect())
                .collect();

            return Type::Matrix(elements.clone(), 
                                (elements.len(), 
                                 elements[0].len()));
        } else {
            //Vec
            let csv: String = no_space
                .trim_matches('[')
                .trim_matches(']')
                .to_string();

            let vector: Vec<f64> = csv.split(',')
                .map(|val| val.parse::<f64>().unwrap_or(0.0))
                .collect(); 

            return Type::Vector(vector.clone(), vector.len());
        }
    }

    invalid
}

fn valid_parens(string: &String) -> bool {
    let mut count = 0;
    for c in string.chars() {
        match c {
            '[' => {count += 1;},
            ']' => {count -= 1;},
            _ => {()}
        }
        if count < 0 {
            return false;
        }
    }
    return count == 0;
}
