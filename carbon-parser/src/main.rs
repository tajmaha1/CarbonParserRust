use carbon_parser::{parse_carbon, ParseError};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "carbon-parser")]
#[command(author = "Daniil Cherniavskyi")]
#[command(version = "0.1.2")]
#[command(about = "A parser for the Carbon language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(value_name = "FILE")]
        file: PathBuf,

        #[arg(short, long)]
        verbose: bool,
    },

    Authors,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file, verbose } => {
            if let Err(e) = parse_file(&file, verbose) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Authors => {
            println!("Carbon Parser v0.1.2");
            println!("Author: Daniil Cherniavskyi");
            println!("\nParser for Google's Carbon programming language");
            println!("Built with Rust and the Pest library");
        }
    }
}

fn parse_file(path: &PathBuf, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Failed to read file '{}': {}",
            path.display(),
            e
        )
    })?;

    println!("Parsing file: {}", path.display());
    println!("Size: {} bytes", content.len());
    println!();

    match parse_carbon(&content) {
        Ok(pairs) => {
            println!("Parsing successful!");

            if verbose {
                println!("\nParse tree:");
                println!("{}", "=".repeat(60));
                for pair in pairs {
                    print_pair(pair, 0);
                }
            }

            Ok(())
        }
        Err(ParseError::PestError(e)) => {
            println!("Parse error:\n");
            eprintln!("{}", e);
            Err(Box::new(e))
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(Box::new(e))
        }
    }
}
fn print_pair(pair: pest::iterators::Pair<carbon_parser::Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!(
        "{}{:?}: {}",
        indent_str,
        pair.as_rule(),
        pair.as_str().trim()
    );

    for inner_pair in pair.into_inner() {
        print_pair(inner_pair, indent + 1);
    }
}