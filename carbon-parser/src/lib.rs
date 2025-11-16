//! # Carbon Parser
//!
//! A library for parsing Google's Carbon programming language.
//!
//! ## Example Usage
//!
//! ```rust
//! use carbon_parser::parse_carbon;
//!
//! let code = r#"
//!     fn main() -> i32 {
//!         var x: i32 = 42;
//!         return x;
//!     }
//! "#;
//!
//! let result = parse_carbon(code);
//! assert!(result.is_ok());

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Carbon parser generated using Pest
#[derive(Parser)]
#[grammar = "carbon.pest"]
pub struct CarbonParser;

/// Parsing errors
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Parser error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

pub type ParseResult<T> = Result<T, ParseError>;

/// Parses Carbon code and returns a parse tree
///
/// # Arguments
///
/// * `input` - Input Carbon code as a string
///
/// # Returns
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Parsing result
///
/// # Example
///
/// ```rust
/// use carbon_parser::parse_carbon;
///
/// let code = "fn test() -> i32 { return 42; }";
/// let result = parse_carbon(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_carbon(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::program, input).map_err(ParseError::from)
}

/// Parses a function declaration
/// A function declaration has the following form:
/// ```carbon
/// fn function_name(param1: Type1, param2: Type2) -> ReturnType {
///     // function body
/// }
/// ```
///
/// # Arguments
///
/// * `input` - Input code with a function declaration
///
/// # Returns
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Parsing result
///
/// # Example
///
/// ```rust
/// use carbon_parser::parse_function_decl;
///
/// let code = "fn add(x: i32, y: i32) -> i32 { return x; }";
/// let result = parse_function_decl(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_function_decl(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::function_decl, input).map_err(ParseError::from)
}

/// Parses a variable declaration
/// A variable declaration has the following form:
/// ```carbon
/// var variable_name: Type = initial_value;
/// ```
///
/// # Arguments
///
/// * `input` - Input code with a variable declaration
///
/// # Returns
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Parsing result
///
/// # Example
///
/// ```rust
/// use carbon_parser::parse_var_decl;
///
/// let code = "var x: i32 = 42;";
/// let result = parse_var_decl(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_var_decl(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::var_decl, input).map_err(ParseError::from)
}

/// Parses an expression
///
/// Expressions include literals, identifiers, binary operations, and function calls.
///
/// # Arguments
///
/// * `input` - Input code with an expression
///
/// # Returns
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Parsing result
///
/// # Example
///
/// ```rust
/// use carbon_parser::parse_expression;
///
/// let code = "42 + x * 2";
/// let result = parse_expression(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_expression(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::expression, input).map_err(ParseError::from)
}

/// Parses a type name
///
/// Supported types: i32, i64, f32, f64, bool, String, and custom types.
///
/// # Arguments
///
/// * `input` - Input code with a type name
///
/// # Returns
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Parsing result
///
/// # Example
///
/// ```rust
/// use carbon_parser::parse_type_name;
///
/// let code = "i32";
/// let result = parse_type_name(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_type_name(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::type_name, input).map_err(ParseError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let code = r#"
            fn main() -> i32 {
                return 42;
            }
        "#;
        assert!(parse_carbon(code).is_ok());
    }

    #[test]
    fn test_function_with_params() {
        let code = "fn add(x: i32, y: i32) -> i32 { return x; }";
        assert!(parse_function_decl(code).is_ok());
    }

    #[test]
    fn test_variable_declaration() {
        let code = "var x: i32 = 42;";
        assert!(parse_var_decl(code).is_ok());
    }

    #[test]
    fn test_expression() {
        let code = "42 + 10 * 2";
        assert!(parse_expression(code).is_ok());
    }

    #[test]
    fn test_type_names() {
        assert!(parse_type_name("i32").is_ok());
        assert!(parse_type_name("bool").is_ok());
        assert!(parse_type_name("String").is_ok());
    }
}