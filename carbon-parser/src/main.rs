use carbon_parser::{parse_carbon, ParseError};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "carbon-parser")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Парсер для мови Carbon", long_about = None)]
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
                eprintln!("Помилка: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Authors => {
            println!("Carbon Parser v0.1.0");
            println!("Автор: Your Name <your.email@example.com>");
            println!("\nПарсер для мови програмування Carbon від Google");
            println!("Написаний на Rust з використанням бібліотеки Pest");
        }
    }
}

fn parse_file(path: &PathBuf, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Не вдалося прочитати файл '{}': {}",
            path.display(),
            e
        )
    })?;

    println!("Парсинг файлу: {}", path.display());
    println!("Розмір: {} байт", content.len());
    println!();

    match parse_carbon(&content) {
        Ok(pairs) => {
            println!("Парсинг успішний!");

            if verbose {
                println!("\nДерево розбору:");
                println!("{}", "=".repeat(60));
                for pair in pairs {
                    print_pair(pair, 0);
                }
            }

            Ok(())
        }
        Err(ParseError::PestError(e)) => {
            println!("Помилка парсингу:\n");
            eprintln!("{}", e);
            Err(Box::new(e))
        }
        Err(e) => {
            println!("Помилка: {}", e);
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