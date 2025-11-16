//! # Carbon Parser
//!
//! A high-performance parser for Google's [Carbon programming language](https://github.com/carbon-language/carbon-lang),
//! built with Rust using the Pest parsing framework.
//!
//! ## Overview
//!
//! This library provides a complete parsing solution for Carbon source code, enabling developers
//! to build tools, compilers, analyzers, and other applications that work with Carbon programs.
//! The parser is built on top of Pest, a PEG (Parsing Expression Grammar) parser that provides
//! excellent performance and clear error messages.
//!
//! ## Features
//!
//! - Fast and reliable parsing using Pest PEG parser
//! - Complete support for Carbon language constructs
//! - Type-safe AST manipulation through Rust's type system
//! - Detailed error messages with line and column information
//! - Zero-copy parsing for efficient memory usage
//! - Command-line interface for file parsing
//! - Comprehensive test coverage
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! carbon-parser = "0.1.0"
//! pest = "2.0"
//! ```
//!
//! ## Quick Start
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
//! match parse_carbon(code) {
//!     Ok(pairs) => {
//!         println!("Parsing successful");
//!         for pair in pairs {
//!             println!("Rule: {:?}", pair.as_rule());
//!         }
//!     }
//!     Err(e) => eprintln!("Parse error: {}", e),
//! }
//! ```
//!
//! ## Supported Language Constructs
//!
//! ### Functions
//!
//! The parser supports function declarations with parameters and return types:
//!
//! ```rust
//! use carbon_parser::parse_function_decl;
//!
//! // Simple function without parameters
//! let code = "fn test() -> i32 { return 42; }";
//! assert!(parse_function_decl(code).is_ok());
//!
//! // Function with single parameter
//! let code = "fn square(x: i32) -> i32 { return x; }";
//! assert!(parse_function_decl(code).is_ok());
//!
//! // Function with multiple parameters
//! let code = "fn add(x: i32, y: i32, z: i32) -> i32 { return x; }";
//! assert!(parse_function_decl(code).is_ok());
//!
//! // Function without return type
//! let code = "fn print_hello() { return 0; }";
//! assert!(parse_function_decl(code).is_ok());
//!
//! // Function with different parameter types
//! let code = "fn process(name: String, age: i32, active: bool) -> bool { return active; }";
//! assert!(parse_function_decl(code).is_ok());
//! ```
//!
//! ### Variables
//!
//! Variable declarations with type annotations and optional initialization:
//!
//! ```rust
//! use carbon_parser::parse_var_decl;
//!
//! // Variable with initialization
//! let code = "var x: i32 = 42;";
//! assert!(parse_var_decl(code).is_ok());
//!
//! // Variable without initialization
//! let code = "var y: bool;";
//! assert!(parse_var_decl(code).is_ok());
//!
//! // String variable
//! let code = r#"var name: String = "John";"#;
//! assert!(parse_var_decl(code).is_ok());
//!
//! // Float variable
//! let code = "var pi: f64 = 3.14;";
//! assert!(parse_var_decl(code).is_ok());
//!
//! // Variable with expression
//! let code = "var sum: i32 = 10 + 20;";
//! assert!(parse_var_decl(code).is_ok());
//! ```
//!
//! ### Expressions
//!
//! The parser handles various expression types including literals, identifiers, binary operations,
//! and function calls:
//!
//! ```rust
//! use carbon_parser::parse_expression;
//!
//! // Integer literal
//! assert!(parse_expression("42").is_ok());
//!
//! // Float literal
//! assert!(parse_expression("3.14").is_ok());
//!
//! // Boolean literals
//! assert!(parse_expression("true").is_ok());
//! assert!(parse_expression("false").is_ok());
//!
//! // String literal
//! assert!(parse_expression(r#""Hello, World!""#).is_ok());
//!
//! // Identifier
//! assert!(parse_expression("variable_name").is_ok());
//!
//! // Binary operations
//! assert!(parse_expression("10 + 20").is_ok());
//! assert!(parse_expression("10 + 20 * 30 - 5").is_ok());
//!
//! // Function call
//! assert!(parse_expression("calculate(x, y)").is_ok());
//!
//! // Comparison operators
//! assert!(parse_expression("x == y").is_ok());
//! assert!(parse_expression("x != y").is_ok());
//! assert!(parse_expression("x < y").is_ok());
//! assert!(parse_expression("x > y").is_ok());
//! ```
//!
//! ### Type System
//!
//! The parser supports both primitive and custom types:
//!
//! ```rust
//! use carbon_parser::parse_type_name;
//!
//! // Integer types
//! assert!(parse_type_name("i32").is_ok());
//! assert!(parse_type_name("i64").is_ok());
//!
//! // Float types
//! assert!(parse_type_name("f32").is_ok());
//! assert!(parse_type_name("f64").is_ok());
//!
//! // Boolean type
//! assert!(parse_type_name("bool").is_ok());
//!
//! // String type
//! assert!(parse_type_name("String").is_ok());
//!
//! // Custom types
//! assert!(parse_type_name("CustomType").is_ok());
//! ```
//!
//! ## Complete Programs
//!
//! The main parsing function handles complete Carbon programs:
//!
//! ```rust
//! use carbon_parser::parse_carbon;
//!
//! // Empty program
//! let code = "";
//! assert!(parse_carbon(code).is_ok());
//!
//! // Single function program
//! let code = r#"
//!     fn main() -> i32 {
//!         return 0;
//!     }
//! "#;
//! assert!(parse_carbon(code).is_ok());
//!
//! // Multiple functions
//! let code = r#"
//!     fn add(x: i32, y: i32) -> i32 {
//!         return x;
//!     }
//!     
//!     fn main() -> i32 {
//!         return 0;
//!     }
//! "#;
//! assert!(parse_carbon(code).is_ok());
//!
//! // Program with variables
//! let code = r#"
//!     var global_x: i32 = 100;
//!     
//!     fn main() -> i32 {
//!         var local_y: i32 = 200;
//!         return 0;
//!     }
//! "#;
//! assert!(parse_carbon(code).is_ok());
//!
//! // Program with comments
//! let code = r#"
//!     // This is a comment
//!     fn main() -> i32 {
//!         /* Multi-line
//!            comment */
//!         return 0;
//!     }
//! "#;
//! assert!(parse_carbon(code).is_ok());
//! ```
//!
//! ## Working with Parse Trees
//!
//! After parsing, you can traverse and inspect the resulting parse tree:
//!
//! ```rust
//! use carbon_parser::{parse_carbon, Rule};
//!
//! let code = r#"
//!     fn calculate(x: i32) -> i32 {
//!         var result: i32 = 42;
//!         return result;
//!     }
//! "#;
//!
//! let pairs = parse_carbon(code).unwrap();
//!
//! for pair in pairs {
//!     println!("Top-level rule: {:?}", pair.as_rule());
//!     println!("Text: {}", pair.as_str());
//!     
//!     // Traverse nested pairs
//!     for inner_pair in pair.into_inner() {
//!         println!("  Nested rule: {:?}", inner_pair.as_rule());
//!         println!("  Nested text: {}", inner_pair.as_str());
//!     }
//! }
//! ```
//!
//! ## Error Handling
//!
//! The parser provides detailed error messages indicating the exact location and nature of syntax errors:
//!
//! ```rust
//! use carbon_parser::parse_carbon;
//!
//! // Invalid syntax - missing closing parenthesis
//! let code = "fn main( { }";
//! assert!(parse_carbon(code).is_err());
//!
//! // Missing semicolon
//! let code = "var x: i32 = 42";
//! assert!(parse_carbon(code).is_err());
//!
//! // Invalid identifier starting with number
//! let code = "var 123invalid: i32 = 0;";
//! assert!(parse_carbon(code).is_err());
//!
//! // Proper error handling
//! match parse_carbon("fn broken( { }") {
//!     Ok(_) => println!("Success"),
//!     Err(e) => {
//!         eprintln!("Parse error occurred:");
//!         eprintln!("{}", e);
//!         // The error will show the exact line and column where parsing failed
//!     }
//! }
//! ```
//!
//! ## Command Line Interface
//!
//! The parser includes a CLI tool for parsing Carbon files:
//!
//! ```bash
//! # Parse a Carbon file
//! cargo run -- parse example.carbon
//!
//! # Parse with verbose output showing the parse tree
//! cargo run -- parse example.carbon --verbose
//!
//! # Show author information
//! cargo run -- authors
//! ```
//!
//! ## Advanced Usage
//!
//! ### Building a Syntax Highlighter
//!
//! ```rust
//! use carbon_parser::{parse_carbon, Rule};
//!
//! fn highlight_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
//!     let pairs = parse_carbon(code)?;
//!     let mut highlighted = String::new();
//!     
//!     for pair in pairs {
//!         match pair.as_rule() {
//!             Rule::function_decl => {
//!                 highlighted.push_str(&format!("<span class='function'>{}</span>", 
//!                     pair.as_str()));
//!             }
//!             Rule::var_decl => {
//!                 highlighted.push_str(&format!("<span class='variable'>{}</span>", 
//!                     pair.as_str()));
//!             }
//!             _ => highlighted.push_str(pair.as_str()),
//!         }
//!     }
//!     
//!     Ok(highlighted)
//! }
//! ```
//!
//! ### Code Analysis
//!
//! ```rust
//! use carbon_parser::{parse_carbon, Rule};
//! use std::collections::HashMap;
//!
//! fn count_functions(code: &str) -> Result<usize, Box<dyn std::error::Error>> {
//!     let pairs = parse_carbon(code)?;
//!     let mut count = 0;
//!     
//!     for pair in pairs {
//!         if matches!(pair.as_rule(), Rule::function_decl) {
//!             count += 1;
//!         }
//!     }
//!     
//!     Ok(count)
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! This parser is designed for optimal performance:
//!
//! - **Zero-copy parsing**: The parse tree references the original input string rather than
//!   copying data, minimizing memory allocations.
//! - **Lazy evaluation**: Parse tree nodes are created on-demand as you traverse the tree.
//! - **Efficient grammar**: The PEG grammar is optimized to minimize backtracking.
//!
//! For large files (>1MB), consider:
//! - Using streaming or incremental parsing if available
//! - Processing the parse tree in chunks
//! - Using the `--verbose` flag judiciously in the CLI tool
//!
//! ## Error Recovery
//!
//! When parsing fails, the error type provides detailed information:
//!
//! ```rust
//! use carbon_parser::{parse_carbon, ParseError};
//!
//! match parse_carbon("invalid code") {
//!     Err(ParseError::PestError(e)) => {
//!         // Pest error with line/column information
//!         eprintln!("Syntax error at: {}", e);
//!     }
//!     Err(ParseError::SyntaxError(msg)) => {
//!         // Custom syntax error
//!         eprintln!("Error: {}", msg);
//!     }
//!     Ok(_) => {}
//! }
//! ```
//!
//! ## Testing
//!
//! The library includes comprehensive integration tests covering:
//! - Function declarations with various parameter combinations
//! - Variable declarations with different types
//! - Expression parsing including operators and precedence
//! - Type system validation
//! - Complete program parsing
//! - Error detection and reporting
//!
//! Run tests with:
//! ```bash
//! cargo test
//! ```
//!
//! ## Grammar Reference
//!
//! The parser is based on a formal grammar defined in `carbon.pest`. Key grammar rules include:
//!
//! - `program`: Top-level rule matching complete Carbon programs
//! - `function_decl`: Function declarations
//! - `var_decl`: Variable declarations
//! - `expression`: All expression types
//! - `type_name`: Type annotations
//! - `statement`: Individual statements
//!
//! ## Contributing
//!
//! Contributions are welcome. Please ensure all tests pass and add tests for new features.
//!
//! ## Author
//!
//! Daniil Cherniavskyi
//!
//! ## License
//!
//! This project is available under standard open source licenses.

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Carbon parser implementation using Pest.
///
/// This struct is generated by the `pest_derive` macro and implements the parsing
/// logic defined in `carbon.pest`. You typically will not need to use this struct
/// directly; instead, use the provided parsing functions.
///
/// # Implementation Details
///
/// The parser uses a PEG (Parsing Expression Grammar) which provides:
/// - Deterministic parsing without ambiguity
/// - Clear error messages with position information
/// - Efficient parsing with linear time complexity
/// - Composable grammar rules
#[derive(Parser)]
#[grammar = "carbon.pest"]
pub struct CarbonParser;

/// Errors that can occur during parsing.
///
/// This enum encapsulates both syntax errors and internal parser errors,
/// providing a unified error handling interface for all parsing operations.
#[derive(Error, Debug)]
pub enum ParseError {
    /// A syntax error in the Carbon source code.
    ///
    /// This error is used for high-level syntax violations that are caught
    /// during semantic analysis rather than during initial parsing.
    #[error("Syntax error: {0}")]
    SyntaxError(String),

    /// An error from the Pest parser.
    ///
    /// This error includes detailed information about:
    /// - The exact line and column where the error occurred
    /// - What the parser expected at that position
    /// - The actual input that was found
    ///
    /// # Example Error Message
    ///
    /// ```text
    /// --> 1:8
    ///   |
    /// 1 | fn test {
    ///   |        ^---
    ///   |
    ///   = expected `(`
    /// ```
    #[error("Parser error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

/// Result type for parsing operations.
///
/// This type alias provides a convenient way to work with parsing results
/// throughout the library. All parsing functions return this type.
pub type ParseResult<T> = Result<T, ParseError>;

/// Parses a complete Carbon program.
///
/// This is the main entry point for parsing Carbon source code. It expects
/// a complete, valid Carbon program which may contain multiple top-level
/// declarations including functions, variables, and other constructs.
///
/// # Arguments
///
/// * `input` - The complete Carbon source code as a string slice
///
/// # Returns
///
/// Returns a `ParseResult` containing an iterator over the top-level parse tree
/// nodes on success, or a detailed `ParseError` on failure.
///
/// # Grammar Rule
///
/// This function uses the `program` grammar rule from `carbon.pest`.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use carbon_parser::parse_carbon;
///
/// let code = r#"
///     fn main() -> i32 {
///         return 0;
///     }
/// "#;
///
/// let result = parse_carbon(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Multiple Functions
///
/// ```rust
/// use carbon_parser::parse_carbon;
///
/// let code = r#"
///     fn add(x: i32, y: i32) -> i32 {
///         return x;
///     }
///     
///     fn main() -> i32 {
///         return 0;
///     }
/// "#;
///
/// let result = parse_carbon(code);
/// assert!(result.is_ok());
/// ```
///
/// ## With Variables
///
/// ```rust
/// use carbon_parser::parse_carbon;
///
/// let code = r#"
///     var counter: i32 = 0;
///     
///     fn increment(x: i32) -> i32 {
///         return x;
///     }
/// "#;
///
/// let result = parse_carbon(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Error Handling
///
/// ```rust
/// use carbon_parser::parse_carbon;
///
/// let invalid = "fn broken( { }";
/// match parse_carbon(invalid) {
///     Ok(_) => panic!("Should have failed"),
///     Err(e) => println!("Expected error: {}", e),
/// }
/// ```
pub fn parse_carbon(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::program, input).map_err(ParseError::from)
}

/// Parses a single function declaration.
///
/// Use this function when you need to parse an individual function definition
/// in isolation, without requiring a complete program context. This is useful
/// for testing, code generation, incremental parsing, or working with code fragments.
///
/// # Carbon Function Syntax
///
/// A function declaration in Carbon has the following structure:
///
/// ```text
/// fn function_name(param1: Type1, param2: Type2) -> ReturnType {
///     // function body statements
///     return expression;
/// }
/// ```
///
/// Components:
/// - `fn` keyword to start the declaration
/// - Function name (identifier)
/// - Parameter list in parentheses (may be empty)
/// - Optional return type preceded by `->`
/// - Function body in braces
///
/// # Arguments
///
/// * `input` - A string slice containing exactly one function declaration
///
/// # Returns
///
/// Returns a `ParseResult` containing the parsed function declaration tree.
///
/// # Grammar Rule
///
/// This function uses the `function_decl` grammar rule from `carbon.pest`.
///
/// # Examples
///
/// ## Function Without Parameters
///
/// ```rust
/// use carbon_parser::parse_function_decl;
///
/// let code = "fn test() -> i32 { return 42; }";
/// let result = parse_function_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Function With Single Parameter
///
/// ```rust
/// use carbon_parser::parse_function_decl;
///
/// let code = "fn square(x: i32) -> i32 { return x; }";
/// let result = parse_function_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Function With Multiple Parameters
///
/// ```rust
/// use carbon_parser::parse_function_decl;
///
/// let code = "fn add(x: i32, y: i32, z: i32) -> i32 { return x; }";
/// let result = parse_function_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Function Without Return Type
///
/// ```rust
/// use carbon_parser::parse_function_decl;
///
/// let code = "fn print_hello() { return 0; }";
/// let result = parse_function_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Function With Mixed Parameter Types
///
/// ```rust
/// use carbon_parser::parse_function_decl;
///
/// let code = "fn process(name: String, age: i32, active: bool) -> bool { return active; }";
/// let result = parse_function_decl(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_function_decl(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::function_decl, input).map_err(ParseError::from)
}

/// Parses a variable declaration statement.
///
/// Variable declarations in Carbon require explicit type annotations and may
/// optionally include an initializer expression. Variables use the `var` keyword
/// and must specify their type before use.
///
/// # Carbon Variable Syntax
///
/// A variable declaration has the following structure:
///
/// ```text
/// var variable_name: Type = initial_value;
/// ```
///
/// Components:
/// - `var` keyword to start the declaration
/// - Variable name (identifier)
/// - Type annotation preceded by `:`
/// - Optional initializer with `=` and expression
/// - Terminating semicolon
///
/// # Arguments
///
/// * `input` - A string slice containing exactly one variable declaration
///
/// # Returns
///
/// Returns a `ParseResult` containing the parsed variable declaration tree.
///
/// # Grammar Rule
///
/// This function uses the `var_decl` grammar rule from `carbon.pest`.
///
/// # Examples
///
/// ## Variable With Initialization
///
/// ```rust
/// use carbon_parser::parse_var_decl;
///
/// let code = "var x: i32 = 42;";
/// let result = parse_var_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Variable Without Initialization
///
/// ```rust
/// use carbon_parser::parse_var_decl;
///
/// let code = "var y: bool;";
/// let result = parse_var_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## String Variable
///
/// ```rust
/// use carbon_parser::parse_var_decl;
///
/// let code = r#"var name: String = "John";"#;
/// let result = parse_var_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Float Variable
///
/// ```rust
/// use carbon_parser::parse_var_decl;
///
/// let code = "var pi: f64 = 3.14;";
/// let result = parse_var_decl(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Variable With Expression
///
/// ```rust
/// use carbon_parser::parse_var_decl;
///
/// let code = "var sum: i32 = 10 + 20;";
/// let result = parse_var_decl(code);
/// assert!(result.is_ok());
/// ```
pub fn parse_var_decl(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::var_decl, input).map_err(ParseError::from)
}

/// Parses an expression.
///
/// Expressions are fundamental building blocks that compute or represent values.
/// This parser handles a comprehensive range of expression types with proper
/// operator precedence and associativity rules.
///
/// # Supported Expression Types
///
/// - **Literals**: Integer, float, boolean, and string constants
/// - **Identifiers**: Variable and function names
/// - **Binary Operations**: Arithmetic, comparison, and logical operators
/// - **Function Calls**: Invocations with argument lists
/// - **Parenthesized Expressions**: For grouping and precedence control
///
/// # Operator Precedence
///
/// Operators are evaluated in the following order (highest to lowest):
///
/// 1. Parentheses: `( )`
/// 2. Unary operators
/// 3. Multiplicative: `*`, `/`, `%`
/// 4. Additive: `+`, `-`
/// 5. Comparison: `<`, `>`, `<=`, `>=`
/// 6. Equality: `==`, `!=`
/// 7. Logical AND: `&&`
/// 8. Logical OR: `||`
///
/// # Arguments
///
/// * `input` - A string slice containing exactly one expression
///
/// # Returns
///
/// Returns a `ParseResult` containing the parsed expression tree.
///
/// # Grammar Rule
///
/// This function uses the `expression` grammar rule from `carbon.pest`.
///
/// # Examples
///
/// ## Literals
///
/// ```rust
/// use carbon_parser::parse_expression;
///
/// // Integer literal
/// assert!(parse_expression("42").is_ok());
///
/// // Float literal
/// assert!(parse_expression("3.14").is_ok());
///
/// // Boolean literals
/// assert!(parse_expression("true").is_ok());
/// assert!(parse_expression("false").is_ok());
///
/// // String literal
/// assert!(parse_expression(r#""Hello, World!""#).is_ok());
/// ```
///
/// ## Identifiers
///
/// ```rust
/// use carbon_parser::parse_expression;
///
/// let code = "variable_name";
/// let result = parse_expression(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Binary Operations
///
/// ```rust
/// use carbon_parser::parse_expression;
///
/// // Addition
/// assert!(parse_expression("10 + 20").is_ok());
///
/// // Complex arithmetic with precedence
/// assert!(parse_expression("10 + 20 * 30 - 5").is_ok());
/// ```
///
/// ## Function Calls
///
/// ```rust
/// use carbon_parser::parse_expression;
///
/// let code = "calculate(x, y)";
/// let result = parse_expression(code);
/// assert!(result.is_ok());
/// ```
///
/// ## Comparison Operators
///
/// ```rust
/// use carbon_parser::parse_expression;
///
/// assert!(parse_expression("x == y").is_ok());
/// assert!(parse_expression("x != y").is_ok());
/// assert!(parse_expression("x < y").is_ok());
/// assert!(parse_expression("x > y").is_ok());
/// ```
pub fn parse_expression(input: &str) -> ParseResult<pest::iterators::Pairs<Rule>> {
    CarbonParser::parse(Rule::expression, input).map_err(ParseError::from)
}

/// Parses a type name.
///
/// Carbon has a static type system that includes both built-in primitive types
/// and support for user-defined custom types. This function validates and parses
/// type names used in variable declarations, function parameters, and return types.
///
/// # Built-in Primitive Types
///
/// - **Signed Integers**: `i32`, `i64`
/// - **Unsigned Integers**: `u32`, `u64` (if supported by grammar)
/// - **Floating-Point**: `f32`, `f64`
/// - **Boolean**: `bool`
/// - **String**: `String`
///
/// # Custom Types
///
/// User-defined types follow standard identifier naming rules and can represent
/// structs, classes, interfaces, or other custom type definitions.
///
/// # Arguments
///
/// * `input` - A string slice containing exactly one type name
///
/// # Returns
///
/// Returns a `ParseResult` containing the parsed type name.
///
/// # Grammar Rule
///
/// This function uses the `type_name` grammar rule from `carbon.pest`.
///
/// # Examples
///
/// ## Integer Types
///
/// ```rust
/// use carbon_parser::parse_type_name;
///
/// assert!(parse_type_name("i32").is_ok());
/// assert!(parse_type_name("i64").is_ok());
/// ```
///
/// ## Float Types
///
/// ```rust
/// use carbon_parser::parse_type_name;
///
/// assert!(parse_type_name("f32").is_ok());
/// assert!(parse_type_name("f64").is_ok());
/// ```
///
/// ## Boolean Type
///
/// ```rust
/// use carbon_parser::parse_type_name;
///
/// let result = parse_type_name("bool");
/// assert!(result.is_ok());
/// ```
///
/// ## String Type
///
/// ```rust
/// use carbon_parser::parse_type_name;
///
/// let result = parse_type_name("String");
/// assert!(result.is_ok());
/// ```
///
/// ## Custom Type
///
/// ```rust
/// use carbon_parser::parse_type_name;
///
/// let result = parse_type_name("CustomType");
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

    #[test]
    fn test_complex_expression() {
        let code = "(a + b) * c - d / e";
        assert!(parse_expression(code).is_ok());
    }

    #[test]
    fn test_function_with_complex_body() {
        let code = r#"
            fn fibonacci(n: i32) -> i32 {
                var a: i32 = 0;
                var b: i32 = 1;
                var i: i32 = 0;
                return a;
            }
        "#;
        assert!(parse_function_decl(code).is_ok());
    }

    #[test]
    fn test_multiple_functions() {
        let code = r#"
            fn first() -> i32 { return 1; }
            fn second() -> i32 { return 2; }
        "#;
        assert!(parse_carbon(code).is_ok());
    }

    #[test]
    fn test_error_handling() {
        let invalid = "fn broken() {";
        assert!(parse_carbon(invalid).is_err());
    }

    #[test]
    fn test_empty_program() {
        let code = "";
        assert!(parse_carbon(code).is_ok());
    }

    #[test]
    fn test_program_with_comments() {
        let code = r#"
            // This is a comment
            fn main() -> i32 {
                /* Multi-line
                   comment */
                return 0;
            }
        "#;
        assert!(parse_carbon(code).is_ok());
    }

    #[test]
    fn test_variable_without_initialization() {
        let code = "var y: bool;";
        assert!(parse_var_decl(code).is_ok());
    }

    #[test]
    fn test_function_without_return_type() {
        let code = "fn print_hello() { return 0; }";
        assert!(parse_function_decl(code).is_ok());
    }

    #[test]
    fn test_comparison_operators() {
        assert!(parse_expression("x == y").is_ok());
        assert!(parse_expression("x != y").is_ok());
        assert!(parse_expression("x < y").is_ok());
        assert!(parse_expression("x > y").is_ok());
    }

    #[test]
    fn test_boolean_literals() {
        assert!(parse_expression("true").is_ok());
        assert!(parse_expression("false").is_ok());
    }

    #[test]
    fn test_string_literal() {
        let code = r#""Hello, World!""#;
        assert!(parse_expression(code).is_ok());
    }

    #[test]
    fn test_function_call() {
        let code = "calculate(x, y)";
        assert!(parse_expression(code).is_ok());
    }

    #[test]
    fn test_invalid_syntax() {
        let code = "fn main( { }";
        assert!(parse_carbon(code).is_err());
    }

    #[test]
    fn test_missing_semicolon() {
        let code = "var x: i32 = 42";
        assert!(parse_var_decl(code).is_err());
    }

    #[test]
    fn test_custom_type() {
        assert!(parse_type_name("CustomType").is_ok());
    }

    #[test]
    fn test_float_types() {
        assert!(parse_type_name("f32").is_ok());
        assert!(parse_type_name("f64").is_ok());
    }

    #[test]
    fn test_program_with_variables() {
        let code = r#"
            var global_x: i32 = 100;
            
            fn main() -> i32 {
                var local_y: i32 = 200;
                return 0;
            }
        "#;
        assert!(parse_carbon(code).is_ok());
    }
}