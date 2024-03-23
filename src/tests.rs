#[cfg(test)]
mod tests {
    use crate::*;
    use crate::data_type::*;


    #[test]
    fn test_det() {
        let mat = Type::Matrix(vec![
            vec![4., 5., 7., 3.],
            vec![-34., 55.,-7., 103.],
            vec![4.345, 52., 17., 39.],
            vec![58.43,65.234, 7.43, 3.974],
        ], (4, 4));
        let answer: f64 = -725176.73344;
        let calc = mat.det(); 
        if let Type::Number(calculation) = calc {
            assert_eq!(calculation, answer);
        }
        if let Type::Error(msg) = calc {
            panic!("{}", msg);
        }
    }

    #[test]
    fn build_var() {
        let mut var_list: Vec<Variable> = Vec::new();
        let input = "let test_name = 54.4".to_string();

        build_variable(input, &mut var_list);
        assert_eq!(var_list.len(), 1);
    }

    #[test]
    fn var_name() {    
        let mut var_list: Vec<Variable> = Vec::new();
        let input = "let test_name = 54.4".to_string();

        build_variable(input, &mut var_list);
        assert_eq!(var_list[0].0, "test_name".to_string());
    }

    #[test]
    fn num_value() {
        let mut var_list: Vec<Variable> = Vec::new();
        let input = "let test_name = 54.4".to_string();

        build_variable(input, &mut var_list);
        assert_eq!(var_list[0].1, Type::Number(54.4));
    }

    #[test]
    fn string_value() {
        let mut var_list: Vec<Variable> = Vec::new();
        let string = "t[est string]".to_string();
        let input = format!(r#"let test_name = "{}""#, string); 

        build_variable(input, &mut var_list);
        assert_eq!(var_list[0].1, Type::String(string));
    }

    #[test]
    fn vec_value() {
        let mut var_list: Vec<Variable> = Vec::new();
        let string = "[4, 6, 7.4, -6]".to_string();
        let input = format!("let test_name = {}", string); 

        build_variable(input, &mut var_list);
        assert_eq!(var_list[0].1, Type::Vector(vec![4., 6., 7.4, -6.], 4));
    }

    #[test]
    fn mat_value() {
        let mut var_list: Vec<Variable> = Vec::new();
        let string = "[[4, 6, 7.4, -6], [6, -7.6545, 23, 0.123]]".to_string();
        let input = format!("let test_name = {}", string); 

        build_variable(input, &mut var_list);
        assert_eq!(var_list[0].1, Type::Matrix(
                vec![vec![4., 6., 7.4, -6.], 
                vec![6., -7.6545, 23., 0.123]],
                (2, 4)));
    }    
}
