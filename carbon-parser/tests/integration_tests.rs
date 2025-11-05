use anyhow::Result;
use carbon_parser::{
    parse_carbon, parse_expression, parse_function_decl, parse_type_name, parse_var_decl,
};

mod function_decl_tests {
    use super::*;

    #[test]
    fn test_simple_function_without_params() -> Result<()> {
        let code = r#"fn test() -> i32 { return 42; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "Проста функція без параметрів повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_function_with_single_param() -> Result<()> {
        let code = r#"fn square(x: i32) -> i32 { return x; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "Функція з одним параметром повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_function_with_multiple_params() -> Result<()> {
        let code = r#"fn add(x: i32, y: i32, z: i32) -> i32 { return x; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "Функція з декількома параметрами повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_function_without_return_type() -> Result<()> {
        let code = r#"fn print_hello() { return 0; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "Функція без типу повернення повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_function_with_different_types() -> Result<()> {
        let code =
            r#"fn process(name: String, age: i32, active: bool) -> bool { return active; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "Функція з різними типами повинна парситися"
        );
        Ok(())
    }
}

mod var_decl_tests {
    use super::*;

    #[test]
    fn test_var_with_initialization() -> Result<()> {
        let code = "var x: i32 = 42;";
        let result = parse_var_decl(code);
        assert!(result.is_ok(), "Змінна з ініціалізацією повинна парситися");
        Ok(())
    }

    #[test]
    fn test_var_without_initialization() -> Result<()> {
        let code = "var y: bool;";
        let result = parse_var_decl(code);
        assert!(
            result.is_ok(),
            "Змінна без ініціалізації повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_var_with_string_type() -> Result<()> {
        let code = r#"var name: String = "John";"#;
        let result = parse_var_decl(code);
        assert!(
            result.is_ok(),
            "Змінна типу String з ініціалізацією повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_var_with_float() -> Result<()> {
        let code = "var pi: f64 = 3.14;";
        let result = parse_var_decl(code);
        assert!(
            result.is_ok(),
            "Змінна з float значенням повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_var_with_expression() -> Result<()> {
        let code = "var sum: i32 = 10 + 20;";
        let result = parse_var_decl(code);
        assert!(result.is_ok(), "Змінна з виразом повинна парситися");
        Ok(())
    }
}

mod expression_tests {
    use super::*;

    #[test]
    fn test_integer_literal() -> Result<()> {
        let code = "42";
        let result = parse_expression(code);
        assert!(result.is_ok(), "Цілочисельний літерал повинен парситися");
        Ok(())
    }

    #[test]
    fn test_float_literal() -> Result<()> {
        let code = "3.14";
        let result = parse_expression(code);
        assert!(result.is_ok(), "Float літерал повинен парситися");
        Ok(())
    }

    #[test]
    fn test_boolean_literal() -> Result<()> {
        assert!(parse_expression("true").is_ok());
        assert!(parse_expression("false").is_ok());
        Ok(())
    }

    #[test]
    fn test_string_literal() -> Result<()> {
        let code = r#""Hello, World!""#;
        let result = parse_expression(code);
        assert!(result.is_ok(), "Рядковий літерал повинен парситися");
        Ok(())
    }

    #[test]
    fn test_identifier() -> Result<()> {
        let code = "variable_name";
        let result = parse_expression(code);
        assert!(result.is_ok(), "Ідентифікатор повинен парситися");
        Ok(())
    }

    #[test]
    fn test_binary_addition() -> Result<()> {
        let code = "10 + 20";
        let result = parse_expression(code);
        assert!(result.is_ok(), "Бінарне додавання повинно парситися");
        Ok(())
    }

    #[test]
    fn test_complex_binary_expression() -> Result<()> {
        let code = "10 + 20 * 30 - 5";
        let result = parse_expression(code);
        assert!(
            result.is_ok(),
            "Складний бінарний вираз повинен парситися"
        );
        Ok(())
    }

    #[test]
    fn test_function_call() -> Result<()> {
        let code = "calculate(x, y)";
        let result = parse_expression(code);
        assert!(result.is_ok(), "Виклик функції повинен парситися");
        Ok(())
    }

    #[test]
    fn test_comparison_operators() -> Result<()> {
        assert!(parse_expression("x == y").is_ok());
        assert!(parse_expression("x != y").is_ok());
        assert!(parse_expression("x < y").is_ok());
        assert!(parse_expression("x > y").is_ok());
        Ok(())
    }
}

mod type_name_tests {
    use super::*;

    #[test]
    fn test_integer_types() -> Result<()> {
        assert!(parse_type_name("i32").is_ok());
        assert!(parse_type_name("i64").is_ok());
        Ok(())
    }

    #[test]
    fn test_float_types() -> Result<()> {
        assert!(parse_type_name("f32").is_ok());
        assert!(parse_type_name("f64").is_ok());
        Ok(())
    }

    #[test]
    fn test_boolean_type() -> Result<()> {
        let result = parse_type_name("bool");
        assert!(result.is_ok(), "Тип bool повинен парситися");
        Ok(())
    }

    #[test]
    fn test_string_type() -> Result<()> {
        let result = parse_type_name("String");
        assert!(result.is_ok(), "Тип String повинен парситися");
        Ok(())
    }

    #[test]
    fn test_custom_type() -> Result<()> {
        let result = parse_type_name("CustomType");
        assert!(result.is_ok(), "Користувацький тип повинен парситися");
        Ok(())
    }
}

mod program_tests {
    use super::*;

    #[test]
    fn test_empty_program() -> Result<()> {
        let code = "";
        let result = parse_carbon(code);
        assert!(result.is_ok(), "Порожня програма повинна парситися");
        Ok(())
    }

    #[test]
    fn test_single_function_program() -> Result<()> {
        let code = r#"
            fn main() -> i32 {
                return 0;
            }
        "#;
        let result = parse_carbon(code);
        assert!(
            result.is_ok(),
            "Програма з однією функцією повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_multiple_functions_program() -> Result<()> {
        let code = r#"
            fn add(x: i32, y: i32) -> i32 {
                return x;
            }
            
            fn main() -> i32 {
                return 0;
            }
        "#;
        let result = parse_carbon(code);
        assert!(
            result.is_ok(),
            "Програма з декількома функціями повинна парситися"
        );
        Ok(())
    }

    #[test]
    fn test_program_with_variables() -> Result<()> {
        let code = r#"
            var global_x: i32 = 100;
            
            fn main() -> i32 {
                var local_y: i32 = 200;
                return 0;
            }
        "#;
        let result = parse_carbon(code);
        assert!(result.is_ok(), "Програма зі змінними повинна парситися");
        Ok(())
    }

    #[test]
    fn test_program_with_comments() -> Result<()> {
        let code = r#"
            // Це коментар
            fn main() -> i32 {
                /* Багаторядковий
                   коментар */
                return 0;
            }
        "#;
        let result = parse_carbon(code);
        assert!(result.is_ok(), "Програма з коментарями повинна парситися");
        Ok(())
    }

    #[test]
    fn test_complex_program() -> Result<()> {
        let code = r#"
            var counter: i32 = 0;
            
            fn increment(x: i32) -> i32 {
                return x;
            }
            
            fn main() -> i32 {
                var result: i32 = 42;
                return result;
            }
        "#;
        let result = parse_carbon(code);
        assert!(result.is_ok(), "Складна програма повинна парситися");
        Ok(())
    }
}

/// Тести для негативних випадків
mod error_tests {
    use super::*;

    #[test]
    fn test_invalid_syntax() {
        let code = "fn main( { }";
        let result = parse_carbon(code);
        assert!(
            result.is_err(),
            "Невалідний синтаксис повинен викликати помилку"
        );
    }

    #[test]
    fn test_missing_semicolon() {
        let code = "var x: i32 = 42";
        let result = parse_var_decl(code);
        assert!(
            result.is_err(),
            "Відсутність крапки з комою повинна викликати помилку"
        );
    }

    #[test]
    fn test_invalid_identifier() {
        let code = "var 123invalid: i32 = 0;";
        let result = parse_var_decl(code);
        assert!(
            result.is_err(),
            "Невалідний ідентифікатор повинен викликати помилку"
        );
    }
}