use scanf::scanf;
mod data_type;
mod mat_opps;

fn id(n: usize) -> Vec<Vec<f32>>{
    let mut result = vec![vec![0.;n]; n];

    for i in 0..n {
        result[i][i] = 1.;
    }

    result
}

fn diag(size: usize, shift: isize) -> Vec<Vec<f32>> {
    let mut result = vec![vec![0.; size]; size];
    
    for i in 0..(size- shift.abs() as usize) {
        let index = i + shift.abs() as usize;
        if shift > 0 {
            result[index as usize][i] = 1.;
        } else {
            result[i][index as usize] = 1.
        }
    }
    result
}

fn dot(vec_1: &Vec<f32>, vec_2: &Vec<f32>) -> f32 {
    vec_1.iter().zip(vec_2.iter()).fold(0., |acc, (a,b)| acc + a * b) 
}

fn det(mat: Vec<Vec<f32>>) -> f32 {
    let len = mat.len();
    if len == 2 {
        return mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
    };

    let topless = cut_row(mat.to_owned(), 0);
    
    (0..len).fold(0., |acc, i| acc + match i % 2 {
        0 => mat[0][i] * det(cut_column(&topless, i)),
        1 => -mat[0][i] * det(cut_column(&topless, i)),
        _ => panic!(),
    })
}

fn cross(vec_a: Vec<f32>, vec_b: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::new();
    let topless = vec![vec_a, vec_b];
    for i in 0..topless.len() {
        result.push(match i % 2 {
            0 => det(cut_column(&topless, i)),
            1 => -det(cut_column(&topless, i)),
            _ => panic!(),
        });
    }
    result
}

fn cut_row(mat: Vec<Vec<f32>>, index: usize) -> Vec<Vec<f32>> {
    let mut result = mat.to_owned();
    result.remove(index);
    result
}

fn cut_column(mat: &Vec<Vec<f32>>, index: usize) -> Vec<Vec<f32>> {
    let mut result: Vec<Vec<f32>> = Vec::new();
    let mut next_row: Vec<f32>;
    for row in mat.iter() {
        next_row = row.to_owned(); 
        next_row.remove(index);
        result.push(next_row);
    }
    result
}

fn add_mat(mat_a: Vec<Vec<f32>>, mat_b: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let width = mat_a.len();
    let height = mat_a[0].len();
    if mat_b.len() != width || mat_b[0].len() != height {
        panic!();
    }
    let mut result = mat_a;
    for i in 0..width {
        for j in 0..height {
            result[i][j] += mat_b[i][j];
        }
    }
    result
}

fn add_vec(vec_a: Vec<f32>, vec_b: Vec<f32>) -> Vec<f32> {
    let len = vec_a.len();
    if vec_b.len() != len {
        panic!();
    }
    let mut result = vec_a;
    for i in 0..len {
        result[i] += vec_b[i];
    }
    result
}

enum Commands {
    Quit,
    Unknown,
    Dot,
    Cross,
    Det,
    Diag,
    Id,
}

struct Variable (String, data_type::Type);

fn get_command(input: String) -> Commands {
    if input == "quit" || input == "q" {
        Commands::Quit
    } else if false {
        Commands::Dot
    } else {
        Commands::Unknown
    }
}

fn arg_type(arg: String) -> data_type::Type {
    todo!();
}

fn main() {
    let vec_a = vec![1., -3., 4., 6.];
    let vec_b = vec![4., 5., -2., 1.];
    let vec_c = vec![5., 6., 9., -10.];
    let vec_d = vec![2., -4., 5., 2.];
    let mat_a = vec![vec_a.to_owned(), 
        vec_b.to_owned(), 
        vec_c.to_owned(), 
        vec_d.to_owned()];

    let mat_b = mat_opps::id(5);

    let mat_c = add_mat(mat_a, mat_b);
    let mut running: bool = true;
    let mut command: Commands;
    let mut args: Vec<String> = Vec::new();
    let mut in_str: String = String::new();

    println!("{:?}", vec_new);

    while running {
        in_str = String::new();

        println!("Testing 123");
        print!("    > ");
        if scanf!("{}", in_str).is_ok() {
            command = get_command(in_str.to_owned());

            match command {
                Commands::Quit => {
                    running = false;
                    println!("Now exiting.");
                },
                Commands::Unknown => {
                    println!("Your command was unknown, please try again");
                },
                Commands::Dot => {
                    if args.len() != 2 {
                        println!("Invalid number of Args");
                    }
                
                },
                Commands::Cross => {
                },
                Commands::Det => {},
                Commands::Diag => {},
                Commands::Id => {},
            }
        } else {
            println!("Error, not a valid input. Please try again");
        }
    }
}
