use scanf::scanf;
mod data_type;
mod generators;
mod tests;
use crate::generators::*;
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
        write!(f, "{} = {:?}", self.0, self.1)
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
