# Carbon Parser

A parser for Google's Carbon programming language, written in Rust using the Pest library.
## Description:

**Carbon Parser** is a tool for syntactic analysis of Carbon language code. The parser processes Carbon source code and builds an Abstract Syntax Tree (AST), which can be used for:

- Static code analysis.
- Creating compilers and interpreters.
- Code formatting tools.
- IDE integration and syntax highlighting.
- Code analysis and refactoring.

## Technical Description of the Parsing Process:

### Parsing Stages:

1. **Lexical Analysis**: Input text is tokenized into identifiers, keywords, and literals.
2. **Syntax Analysis**: Tokens are processed according to the grammatical rules of Carbon.
3. **AST Construction**: A tree-like structure representing the program's syntax is created.

### What is parsed:

The parser supports the following Carbon constructs:

- **Function Declarations**: `fn FunctionName(param: Type) -> ReturnType { ... }`
- **Variable Declarations**: `var variable_name: Type = value;`
- **Data Types**: basic types (`i32`, `f64`, `bool`, `String`)
- **Expressions**: arithmetic operations, function calls, literals
- **Comments**: single-line (`//`) and multi-line (`/* */`)

### Grammar Rules:
```pest
program = { SOI ~ (function_decl | var_decl)* ~ EOI }

function_decl = { "fn" ~ identifier ~ "(" ~ parameter_list? ~ ")" ~ ("->" ~ type_name)? ~ block }

var_decl = { "var" ~ identifier ~ ":" ~ type_name ~ ("=" ~ expression)? ~ ";" }

parameter_list = { parameter ~ ("," ~ parameter)* }
parameter = { identifier ~ ":" ~ type_name }

block = { "{" ~ statement* ~ "}" }
statement = { var_decl | expression ~ ";" | return_stmt }

expression = { literal | identifier | binary_expr | function_call }
```

### Using the Results:

The parsing result is a `Pairs<Rule>` from Pest, which represents the parse tree. This tree can be:

- Transformed into a typed AST for further processing.
- Used for syntax validation.
- Converted to other formats (JSON, XML).
- Applied for code analysis and metrics.

## Grammar Diagram:
```
Program
├── FunctionDecl*
│   ├── Identifier (назва функції)
│   ├── ParameterList
│   │   └── Parameter* (ім'я: тип)
│   ├── ReturnType (опціонально)
│   └── Block
│       └── Statement*
└── VarDecl*
    ├── Identifier (назва змінної)
    ├── TypeName
    └── Expression (опціонально)
```

## Installation:
```bash
cargo install carbon-parser
```

Or clone the repository:
```bash
git clone https://github.com/tajmaha1/carbon-parser
cd carbon-parser
cargo build --release
```

## Usage:

### CLI
```bash
# Parse a file
carbon-parser parse input.carbon

# Show help
carbon-parser help

# Show version
carbon-parser --version
```

### As a Library:
```rust
use carbon_parser::{parse_carbon, CarbonParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_code = r#"
        fn main() -> i32 {
            var x: i32 = 42;
            return x;
        }
    "#;
    
    let parse_tree = parse_carbon(source_code)?;
    println!("Парсинг успішний!");
    
    Ok(())
}
```

## Development:

### Running Tests:
```bash
make test
```

### Code Formatting:
```bash
make fmt
```

### Linting:
```bash
make clippy
```

### Running the Program:
```bash
make run
```

### Before Committing:
```bash
make pre-commit
```

## Testing:

The project contains unit tests for each grammar rule:

- Tests for function declarations.
- Tests for variable declarations.
- Tests for expressions.
- Tests for types.
- Tests for comments.

Run tests: `cargo test`

## Documentation:

Full documentation is available at [docs.rs/carbon-parser](https://docs.rs/carbon-parser)

Local documentation:
```bash
cargo doc --open
```

## License:

This project is distributed under the dual MIT/Apache-2.0 license.

## Author:

[Daniil Cherniavskyi] - [rembo9028@gmail.com]

## Contribution:

Pull requests are welcome. For significant changes, please open an issue first to discuss.