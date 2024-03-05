fn eye(n: usize) -> Vec<Vec<isize>>{
    let mut result = vec![vec![0;n]; n];

    for i in 0..n {
        result[i][i] = 1;
    }

    result
}

fn dot(vec_1: Vec<isize>, vec_2: Vec<isize>) -> isize {
    vec_1.iter().zip(vec_2.iter()).fold(0, |acc, (a,b)| acc + a * b) 
}

fn det(mat: Vec<Vec<isize>>) -> isize {
    let len = mat.len();
    if len == 2 {
        return mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
    };

    let topless = cut_row(&mat, 0);
    
    (0..len).fold(0, |acc, i| acc + match i % 2 {
        0 => mat[0][i] * det(cut_column(&topless, i)),
        1 => -mat[0][i] * det(cut_column(&topless, i)),
        _ => panic!(),
    })
    //let mut sub_mat;
    //let mut sum = 0;
    //for i in 0..len {
    //    sub_mat = cut_column(&topless, i);
    //    
    //    sum += match i % 2 {
    //        0 => mat[0][i] * det(sub_mat),
    //        1 => -mat[0][i] * det(sub_mat),
    //        _ => panic!(),
    //    };
    //}
    //sum
}

fn cut_row(mat: &Vec<Vec<isize>>, index: usize) -> Vec<Vec<isize>> {
    let mut result = mat.to_owned();
    result.remove(index);
    result
}

fn cut_column(mat: &Vec<Vec<isize>>, index: usize) -> Vec<Vec<isize>> {
    let mut result: Vec<Vec<isize>> = Vec::new();
    let mut next_row: Vec<isize>;
    for row in mat.iter() {
        next_row = row.to_owned(); 
        next_row.remove(index);
        result.push(next_row);
    }
    result
}

fn main() {
    let running: bool = true;
    let vec_a = vec![1, -3, 4, 6];
    let vec_b = vec![4, 5, -2, 1];
    let vec_c = vec![5, 6, 9, -10];
    let vec_d = vec![2, -4, 5, 2];
    let mat_a = vec![vec_a, vec_b, vec_c, vec_d];
    let ans = det(mat_a); //2007
    println!("{}", ans);
//    while running {
//        todo!();
//    }
}
