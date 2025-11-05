//! # Carbon Parser
//!
//! Бібліотека для парсингу мови програмування Carbon від Google.
//!
//! ## Приклад використання
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
//! ```

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Парсер Carbon, згенерований за допомогою Pest
#[derive(Parser)]
#[grammar = "carbon.pest"]
pub struct CarbonParser;

/// Помилки парсингу
#[derive(Error, Debug)]
pub enum ParseError {
    /// Синтаксична помилка під час парсингу
    #[error("Синтаксична помилка: {0}")]
    SyntaxError(String),

    /// Помилка від Pest парсера
    #[error("Помилка парсера: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

/// Результат парсингу
pub type ParseResult<T> = Result<T, ParseError>;

/// Парсить код Carbon та повертає дерево розбору
///
/// # Аргументи
///
/// * `input` - Вхідний код Carbon як рядок
///
/// # Повертає
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Результат парсингу
///
/// # Приклад
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

/// Парсить декларацію функції
///
/// Декларація функції має форму:
/// ```carbon
/// fn function_name(param1: Type1, param2: Type2) -> ReturnType {
///     // тіло функції
/// }
/// ```
///
/// # Аргументи
///
/// * `input` - Вхідний код з декларацією функції
///
/// # Повертає
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Результат парсингу
///
/// # Приклад
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

/// Парсить декларацію змінної
///
/// Декларація змінної має форму:
/// ```carbon
/// var variable_name: Type = initial_value;
/// ```
///
/// # Аргументи
///
/// * `input` - Вхідний код з декларацією змінної
///
/// # Повертає
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Результат парсингу
///
/// # Приклад
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

/// Парсить вираз
///
/// Вирази включають літерали, ідентифікатори, бінарні операції та виклики функцій.
///
/// # Аргументи
///
/// * `input` - Вхідний код з виразом
///
/// # Повертає
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Результат парсингу
///
/// # Приклад
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

/// Парсить ім'я типу
///
/// Підтримувані типи: i32, i64, f32, f64, bool, String та користувацькі типи.
///
/// # Аргументи
///
/// * `input` - Вхідний код з іменем типу
///
/// # Повертає
///
/// * `ParseResult<pest::iterators::Pairs<Rule>>` - Результат парсингу
///
/// # Приклад
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