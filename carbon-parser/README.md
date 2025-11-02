# Carbon Parser

Парсер для мови програмування Carbon від Google, написаний на Rust з використанням бібліотеки Pest.

## Опис

**Carbon Parser** — це інструмент для синтаксичного аналізу коду мови Carbon. Парсер розбирає вихідний код Carbon та будує абстрактне синтаксичне дерево (AST), яке може використовуватися для:

- Статичного аналізу коду
- Створення компіляторів та інтерпретаторів
- Інструментів форматування коду
- IDE-інтеграції та підсвічування синтаксису
- Аналізу та рефакторингу коду

## Технічний опис процесу парсингу

### Етапи парсингу

1. **Лексичний аналіз**: Вхідний текст розбивається на токени (ідентифікатори, ключові слова, літерали)
2. **Синтаксичний аналіз**: Токени обробляються відповідно до граматичних правил Carbon
3. **Побудова AST**: Створюється деревоподібна структура, що представляє синтаксис програми

### Що парситься

Парсер підтримує наступні конструкції Carbon:

- **Декларації функцій**: `fn FunctionName(param: Type) -> ReturnType { ... }`
- **Декларації змінних**: `var variable_name: Type = value;`
- **Типи даних**: базові типи (`i32`, `f64`, `bool`, `String`)
- **Вирази**: арифметичні операції, виклики функцій, літерали
- **Коментарі**: однорядкові (`//`) та багаторядкові (`/* */`)

### Граматичні правила
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

### Використання результатів

Результат парсингу — це `Pairs<Rule>` від Pest, який представляє дерево розбору. Це дерево можна:

- Трансформувати в типізоване AST для подальшої обробки
- Використати для валідації синтаксису
- Перетворити в інші формати (JSON, XML)
- Застосувати для аналізу коду та метрик

## Діаграма граматики
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

## Встановлення
```bash
cargo install carbon-parser
```

Або клонуйте репозиторій:
```bash
git clone https://github.com/yourusername/carbon-parser
cd carbon-parser
cargo build --release
```

## Використання

### CLI
```bash
# Парсинг файлу
carbon-parser parse input.carbon

# Показати допомогу
carbon-parser help

# Показати інформацію про авторів
carbon-parser --version
```

### Як бібліотека
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

## Розробка

### Запуск тестів
```bash
make test
```

### Форматування коду
```bash
make fmt
```

### Лінтинг
```bash
make clippy
```

### Запуск програми
```bash
make run
```

### Перед комітом
```bash
make pre-commit
```

## Тестування

Проект містить модульні тести для кожного граматичного правила:

- Тести декларацій функцій
- Тести декларацій змінних
- Тести виразів
- Тести типів
- Тести коментарів

Запуск тестів: `cargo test`

## Документація

Повна документація доступна на [docs.rs/carbon-parser](https://docs.rs/carbon-parser)

Локальна документація:
```bash
cargo doc --open
```

## Ліцензія

Цей проект розповсюджується під подвійною ліцензією MIT/Apache-2.0.

## Автор

[Daniil Cherniavskyi] - [rembo9028@gmail.com]

## Внесок

Вітаються pull requests. Для значних змін спочатку відкрийте issue для обговорення.