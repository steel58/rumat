use crate::det;

#[derive(Debug)]
pub enum Type {
    Number (f64),
    Bool (bool),
    String (String),
    Vector (Vec<f64>),
    Matrix (Vec<Vec<f64>>),
    Error (String),
}

impl Type {
    pub fn add(&self, other: Type) -> Type{
        match self {
            Self::Number(_) => self.num_add(other),
            Self::Bool(_) => Self::Error("Type: 'Bool' cannot be added".to_string()),
            Self::String(_) => Type::Error("Type: 'String' cannot be added".to_string()),
            Self::Vector(_) => self.vec_add(other),
            Self::Matrix(_) => self.mat_add(other),
            Self::Error(_) => Type::Error("Type: 'Error' cannot be added".to_string()),
        }
    }

    pub fn vec_add(&self, vec_b: Type) -> Self {
        let mut vec_1: Vec<f64>; 
        let mut vec_2: Vec<f64>; 

        if let Self::Vector(vec) = self {
            vec_1 = vec.to_owned();
        } else {
            return Type::Error("This vector is invalid".to_string());
        }
        if let Self::Vector(vec) = vec_b {
            vec_2 = vec;
        } else {
            return Type::Error("This vector is invalid".to_string());
        }
        let len = vec_1.len();
        if vec_2.len() != len {
           return Type::Error("Cannot add vectors of differing lengths.".to_string());
        }
        let mut result = vec_1;
        for i in 0..len {
            result[i] += vec_2[i];
        }
        return Type::Vector(result.to_owned());
    }

    pub fn num_add(&self, num_b: Type) -> Self {
        let mut num_1: f64;
        let mut num_2: f64;
        
        if let Self::Number(num) = self {
            num_1 = num.to_owned();
        } else {
            return Type::Error("One or more numbers are invalid.".to_string());
        }
        if let Self::Number(num) = num_b {
            num_2 = num
        } else {
            return Type::Error("One or more numbers are invalid.".to_string());
        }
        
        return Type::Number(num_1 + num_2);
    }

    pub fn mat_add(&self, mat_2: Type) -> Self {
        let mut mat_a: Vec<Vec<f64>>;
        let mut mat_b: Vec<Vec<f64>>;
        
        if let Self::Matrix(mat) = self {

            mat_a = mat.to_owned();
        } else {
            return Type::Error("One or more matracies are invalid.".to_string());
        }

        if let Self::Matrix(mat) = mat_2 {
            mat_b = mat.to_owned();
        } else {
            return Type::Error("One of more matracies are invalid.".to_string());
        }

        let width = mat_a.len();
        let height = mat_a[0].len();

        if mat_b.len() != width || mat_b[0].len() != height {
            return Type::Error("Cannot add matracies of differing sizes.".to_string());
        }

        let mut result = mat_a;

        for i in 0..width {
            for j in 0..height {
                result[i][j] += mat_b[i][j];
            }
        }
        return Type::Matrix(result);
    }


    pub fn det(&self) -> Type {
        let mat: Vec<Vec<f64>>;
        if let Self::Matrix(m) = self {
            mat = m.to_owned();
        } else { 
            return Type::Error("Cannot take determinant of non-matrix type".to_owned());
        }

        let len = mat.len();
        if mat[0].len() != len {
            return Type::Error("Matrix must be square for determinant".to_owned());
        }

        let result: f64 = det(mat);
        return Type::Number(result);
        
    }


}
