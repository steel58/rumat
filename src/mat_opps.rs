pub fn id(n: usize) -> Vec<Vec<f64>>{
    let mut result = vec![vec![0.;n]; n];

    for i in 0..n {
        result[i][i] = 1.;
    }

    result
}

pub fn diag(size: usize, shift: isize) -> Vec<Vec<f64>> {
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

pub fn dot(vec_1: &Vec<f64>, vec_2: &Vec<f64>) -> f64 {
    vec_1.iter().zip(vec_2.iter()).fold(0., |acc, (a,b)| acc + a * b) 
}

pub fn det(mat: Vec<Vec<f64>>) -> f64 {
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

pub fn cross(vec_a: Vec<f64>, vec_b: Vec<f64>) -> Vec<f64> {
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

pub fn cut_row(mat: Vec<Vec<f64>>, index: usize) -> Vec<Vec<f64>> {
    let mut result = mat.to_owned();
    result.remove(index);
    result
}

pub fn cut_column(mat: &Vec<Vec<f64>>, index: usize) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    let mut next_row: Vec<f64>;
    for row in mat.iter() {
        next_row = row.to_owned(); 
        next_row.remove(index);
        result.push(next_row);
    }
    result
}

pub fn add_mat(mat_a: Vec<Vec<f64>>, mat_b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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

pub fn add_vec(vec_a: Vec<f64>, vec_b: Vec<f64>) -> Vec<f64> {
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
