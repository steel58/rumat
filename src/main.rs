use scanf::scanf;
mod data_type;
mod mat_opps;
mod tests;
use crate::mat_opps::*;
use crate::data_type::*;
use std::fmt;

enum Commands {
    Quit,
    Unknown,
    Dot,
    Cross,
    Det,
    Diag,
    Id,
    Build,
}

#[derive(Debug)]
struct Variable (String, data_type::Type);

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.0, self.1)
    }
}

fn get_command(input: &String) -> Commands {
    let parts: Vec<&str> = input.split(" ").collect();
    
    if parts[0] == "quit" || parts[0] == "q" {
        Commands::Quit
    } else if parts[0] == "let" {
        Commands::Build
    } else if false {
        Commands::Dot
    } else {
        Commands::Unknown
    }
}

impl Variable {
    fn new(name: &str, value: Type) -> Self {
        Self(
            name.to_string(),
            value,
            )
    }
}


fn build_variable(input: String, var_list: &mut Vec<Variable>) {
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
                .map(|mut str| {str.to_string().pop(); str.to_string()})
                .collect();
            let elements: Vec<Vec<f64>> = rows.iter()
                .map(|r| r.split(',')
                     .map(|val| val.parse::<f64>()
                          .unwrap_or(0.0 ))
                     .collect())
                .collect();

            return Type::Matrix(elements);
        } else {
            //Vec
            let csv: String = no_space
                .trim_matches('[')
                .trim_matches(']')
                .to_string();

            let vector: Vec<f64> = csv.split(',')
                .map(|val| val.parse::<f64>().unwrap_or(0.0))
                .collect(); 

            return Type::Vector(vector);
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

fn main() {
    let mut running: bool = true;
    let mut command: Commands;
    let mut in_str: String;
    let mut prepped_str: String;
    let mut variables: Vec<Variable> = Vec::new();

    while running {
        in_str = String::new();

        print!("    > ");
        if scanf!("{}", in_str).is_ok() {
            prepped_str = in_str.trim().chars().collect();
            command = get_command(&prepped_str);

            match command {
                Commands::Quit => {
                    running = false;
                    println!("Now exiting.");
                },
                Commands::Unknown => {
                    println!("Your command was unknown, please try again");
                },
                Commands::Dot => {},
                Commands::Cross => {},
                Commands::Det => {},
                Commands::Diag => {},
                Commands::Id => {},
                Commands::Build => {
                    build_variable(prepped_str, &mut variables);
                },
            }
        } else {
            println!("Error, not an input. Please try again");
        }
    }
}
