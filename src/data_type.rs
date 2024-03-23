use crate::det;

#[derive(Debug)]
pub enum Type {
    Number (f64),
    Bool (bool),
    String (String),
    Vector (Vec<f64>, usize),
    Matrix (Vec<Vec<f64>>, (usize, usize)),
    Error (String),
}


impl PartialEq<Type> for Type {
    fn eq(&self, other: &Type) -> bool {
         match (self, other) {
            (Self::Number(a), Self::Number(b)) => {a == b},
            (Self::Bool(a), Self::Bool(b)) => {a == b},
            (Self::String(a), Self::String(b)) => {a == b},
            (Self::Vector(a, sza), Self::Vector(b, szb)) => {a == b},
            (Self::Matrix(a, sza), Self::Matrix(b, szb)) => {a == b},
            _ => false,
        }
    }
}

impl Type {
    pub fn add(&self, other: Type) -> Type{
        match self {
            Self::Number(_) => self.num_add(other),
            Self::Bool(_) => Self::Error("Type: 'Bool' cannot be added".to_string()),
            Self::String(_) => Type::Error("Type: 'String' cannot be added".to_string()),
            Self::Vector(_, _) => self.vec_add(other),
            Self::Matrix(_, _) => self.mat_add(other),
            Self::Error(_) => Type::Error("Type: 'Error' cannot be added".to_string()),
        }
    }

    pub fn vec_add(&self, vec_b: Type) -> Self {
        let mut vec_1: Vec<f64>; 
        let sza: usize; 
        let mut vec_2: Vec<f64>;
        let szb: usize;

        let mut vec_2: Vec<f64>; if let Self::Vector(vec, sz) = self {
            vec_1 = vec.to_owned();
            sza = sz.to_owned();
        } else {
            return Type::Error("This vector is invalid".to_string());
        }
        if let Self::Vector(vec, sz) = vec_b {
            vec_2 = vec;
            szb = sz;
        } else {
            return Type::Error("This vector is invalid".to_string());
        }

        if sza != szb {
           return Type::Error("Cannot add vectors of differing lengths.".to_string());
        }
        let mut result = vec_1;
        for i in 0..sza {
            result[i] += vec_2[i];
        }
        return Type::Vector(result.to_owned(), sza);
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
        let sza: (usize, usize);
        let mut mat_b: Vec<Vec<f64>>;
        let szb: (usize, usize);
        
        if let Self::Matrix(mat, sz) = self {
            mat_a = mat.to_owned();
            sza = sz.to_owned();
        } else {
            return Type::Error("One or more matracies are invalid.".to_string());
        }

        if let Self::Matrix(mat, sz) = mat_2 {
            mat_b = mat.to_owned();
            szb = sz.to_owned(); 
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
        return Type::Matrix(result, sza);
    }


    pub fn det(&self) -> Type {
        let mat: Vec<Vec<f64>>;
        let sz: (usize, usize);
        if let Self::Matrix(m, s) = self {
            mat = m.to_owned();
            sz = s.to_owned();
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

    pub fn size_of(&self) -> Type {
        match self {
            Self::Number(n) => Self::Number(*n),
            Self::Bool(_) => Self::Error("Type: 'Bool' does not have size".to_string()),
            Self::String(s) => Self::Number(s.len() as f64),
            Self::Vector(_, sz) => Self::Number(*sz as f64),
            Self::Matrix(_, sz) => Self::Vector(vec![sz.0 as f64, sz.1 as f64], 2),
            Self::Error(_) => Type::Error("Type: 'Error' does not have atribute size".to_string()),
        }
    }

}
