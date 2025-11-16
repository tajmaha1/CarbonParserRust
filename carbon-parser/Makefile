.PHONY: all build run test fmt clippy clean pre-commit help doc

# Colors for output
GREEN := \033[0;32m
NC := \033[0m # No Color

all: build

## Build the project
build:
    @echo "$(GREEN)Building the project...$(NC)"
    cargo build --release

## Run the program (example with a test file)
run:
    @echo "$(GREEN)Running the program...$(NC)"
    @echo 'fn main() -> i32 { var x: i32 = 42; return x; }' > example.carbon
    cargo run -- parse example.carbon --verbose
    @rm -f example.carbon

## Run all tests
test:
    @echo "$(GREEN)Running tests...$(NC)"
    cargo test --all

## Run tests with detailed output
test-verbose:
    @echo "$(GREEN)Running tests with detailed output...$(NC)"
    cargo test --all -- --nocapture

## Format the code
fmt:
    @echo "$(GREEN)Formatting the code...$(NC)"
    cargo fmt --all

## Check formatting without making changes
fmt-check:
    @echo "$(GREEN)Checking formatting...$(NC)"
    cargo fmt --all -- --check

## Lint the code
clippy:
    @echo "$(GREEN)Linting with Clippy...$(NC)"
    cargo clippy --all-targets --all-features -- -D warnings

## Generate documentation
doc:
    @echo "$(GREEN)Generating documentation...$(NC)"
    cargo doc --no-deps --open

## Generate documentation without opening
doc-build:
    @echo "$(GREEN)Generating documentation...$(NC)"
    cargo doc --no-deps

## Pre-commit checks (formatting + linting + tests)
pre-commit: fmt clippy test
    @echo "$(GREEN)✓ All checks passed successfully!$(NC)"

## Clean the build
clean:
    @echo "$(GREEN)Cleaning...$(NC)"
    cargo clean
    @rm -f example.carbon

## Audit dependencies for security issues
audit:
    @echo "$(GREEN)Auditing dependencies for security issues...$(NC)"
    cargo audit

## Update dependencies
update:
    @echo "$(GREEN)Updating dependencies...$(NC)"
    cargo update

## Install the project globally
install:
    @echo "$(GREEN)Installing carbon-parser...$(NC)"
    cargo install --path .

## Uninstall the globally installed project
uninstall:
    @echo "$(GREEN)Uninstalling carbon-parser...$(NC)"
    cargo uninstall carbon-parser

## Create a test file
create-example:
    @echo "$(GREEN)Creating test file example.carbon...$(NC)"
    @echo 'fn factorial(n: i32) -> i32 {' > example.carbon
    @echo '    var result: i32 = 1;' >> example.carbon
    @echo '    return result;' >> example.carbon
    @echo '}' >> example.carbon
    @echo '' >> example.carbon
    @echo 'fn main() -> i32 {' >> example.carbon
    @echo '    var x: i32 = 5;' >> example.carbon
    @echo '    var y: i32 = 10;' >> example.carbon
    @echo '    return x;' >> example.carbon
    @echo '}' >> example.carbon
    @echo "File created: example.carbon"

## Parse the test file
parse-example: create-example
    @echo "$(GREEN)Parsing example.carbon...$(NC)"
    cargo run -- parse example.carbon --verbose

## Check test coverage (requires cargo-tarpaulin)
coverage:
    @echo "$(GREEN)Analyzing test coverage...$(NC)"
    cargo tarpaulin --out Html --output-dir coverage

## Run benchmarks (if available)
bench:
    @echo "$(GREEN)Running benchmarks...$(NC)"
    cargo bench

## Show help
help:
    @echo "$(GREEN)Available commands:$(NC)"
    @echo "  make build         - Build the project"
    @echo "  make run           - Run the program with an example"
    @echo "  make test          - Run all tests"
    @echo "  make test-verbose  - Run tests with detailed output"
    @echo "  make fmt           - Format the code"
    @echo "  make fmt-check     - Check formatting"
    @echo "  make clippy        - Lint the code"
    @echo "  make doc           - Generate and open documentation"
    @echo "  make doc-build     - Generate documentation"
    @echo "  make pre-commit    - Run all pre-commit checks"
    @echo "  make clean         - Clean the build"
    @echo "  make audit         - Audit dependencies"
    @echo "  make update        - Update dependencies"
    @echo "  make install       - Install globally"
    @echo "  make uninstall     - Uninstall"
    @echo "  make create-example - Create a test file"
    @echo "  make parse-example - Parse the test file"
    @echo "  make coverage      - Analyze test coverage"
    @echo "  make help          - Show this help"