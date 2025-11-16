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
            "A simple function without parameters should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_function_with_single_param() -> Result<()> {
        let code = r#"fn square(x: i32) -> i32 { return x; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "A function with a single parameter should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_function_with_multiple_params() -> Result<()> {
        let code = r#"fn add(x: i32, y: i32, z: i32) -> i32 { return x; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "A function with multiple parameters should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_function_without_return_type() -> Result<()> {
        let code = r#"fn print_hello() { return 0; }"#;
        let result = parse_function_decl(code);
        assert!(
            result.is_ok(),
            "A function without a return type should parse successfully"
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
            "A function with different parameter types should parse successfully"
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
        assert!(result.is_ok(), "A variable with initialization should parse successfully");
        Ok(())
    }

    #[test]
    fn test_var_without_initialization() -> Result<()> {
        let code = "var y: bool;";
        let result = parse_var_decl(code);
        assert!(
            result.is_ok(),
            "A variable without initialization should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_var_with_string_type() -> Result<()> {
        let code = r#"var name: String = "John";"#;
        let result = parse_var_decl(code);
        assert!(
            result.is_ok(),
            "A variable of type String with initialization should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_var_with_float() -> Result<()> {
        let code = "var pi: f64 = 3.14;";
        let result = parse_var_decl(code);
        assert!(
            result.is_ok(),
            "A variable with a float value should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_var_with_expression() -> Result<()> {
        let code = "var sum: i32 = 10 + 20;";
        let result = parse_var_decl(code);
        assert!(result.is_ok(), "A variable with an expression should parse successfully");
        Ok(())
    }
}

mod expression_tests {
    use super::*;

    #[test]
    fn test_integer_literal() -> Result<()> {
        let code = "42";
        let result = parse_expression(code);
        assert!(result.is_ok(), "An integer literal should parse successfully");
        Ok(())
    }

    #[test]
    fn test_float_literal() -> Result<()> {
        let code = "3.14";
        let result = parse_expression(code);
        assert!(result.is_ok(), "A float literal should parse successfully");
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
        assert!(result.is_ok(), "A string literal should parse successfully");
        Ok(())
    }

    #[test]
    fn test_identifier() -> Result<()> {
        let code = "variable_name";
        let result = parse_expression(code);
        assert!(result.is_ok(), "An identifier should parse successfully");
        Ok(())
    }

    #[test]
    fn test_binary_addition() -> Result<()> {
        let code = "10 + 20";
        let result = parse_expression(code);
        assert!(result.is_ok(), "Binary addition should parse successfully");
        Ok(())
    }

    #[test]
    fn test_complex_binary_expression() -> Result<()> {
        let code = "10 + 20 * 30 - 5";
        let result = parse_expression(code);
        assert!(
            result.is_ok(),
            "A complex binary expression should parse successfully"
        );
        Ok(())
    }

    #[test]
    fn test_function_call() -> Result<()> {
        let code = "calculate(x, y)";
        let result = parse_expression(code);
        assert!(result.is_ok(), "A function call should parse successfully");
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
        assert!(result.is_ok(), "The type bool should parse successfully");
        Ok(())
    }

    #[test]
    fn test_string_type() -> Result<()> {
        let result = parse_type_name("String");
        assert!(result.is_ok(), "The type String should parse successfully");
        Ok(())
    }

    #[test]
    fn test_custom_type() -> Result<()> {
        let result = parse_type_name("CustomType");
        assert!(result.is_ok(), "A custom type should parse successfully");
        Ok(())
    }
}

mod program_tests {
    use super::*;

    #[test]
    fn test_empty_program() -> Result<()> {
        let code = "";
        let result = parse_carbon(code);
        assert!(result.is_ok(), "An empty program should parse successfully");
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
            "A program with a single function should parse successfully"
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
            "A program with multiple functions should parse successfully"
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
        assert!(result.is_ok(), "A program with variables should parse successfully");
        Ok(())
    }

    #[test]
    fn test_program_with_comments() -> Result<()> {
        let code = r#"
            // This is a comment
            fn main() -> i32 {
                /* Multi-line
                   comment */
                return 0;
            }
        "#;
        let result = parse_carbon(code);
        assert!(result.is_ok(), "A program with comments should parse successfully");
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
        assert!(result.is_ok(), "A complex program should parse successfully");
        Ok(())
    }
}

mod error_tests {
    use super::*;

    #[test]
    fn test_invalid_syntax() {
        let code = "fn main( { }";
        let result = parse_carbon(code);
        assert!(
            result.is_err(),
            "Invalid syntax should produce an error"
        );
    }

    #[test]
    fn test_missing_semicolon() {
        let code = "var x: i32 = 42";
        let result = parse_var_decl(code);
        assert!(
            result.is_err(),
            "A missing semicolon should produce an error"
        );
    }

    #[test]
    fn test_invalid_identifier() {
        let code = "var 123invalid: i32 = 0;";
        let result = parse_var_decl(code);
        assert!(
            result.is_err(),
            "An invalid identifier should produce an error"
        );
    }
}