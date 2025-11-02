.PHONY: all build run test fmt clippy clean pre-commit help doc

# Колір для виводу
GREEN := \033[0;32m
NC := \033[0m # No Color

all: build

## Збірка проекту
build:
	@echo "$(GREEN)Збірка проекту...$(NC)"
	cargo build --release

## Запуск програми (приклад з тестовим файлом)
run:
	@echo "$(GREEN)Запуск програми...$(NC)"
	@echo 'fn main() -> i32 { var x: i32 = 42; return x; }' > example.carbon
	cargo run -- parse example.carbon --verbose
	@rm -f example.carbon

## Запуск всіх тестів
test:
	@echo "$(GREEN)Запуск тестів...$(NC)"
	cargo test --all

## Запуск тестів з детальним виводом
test-verbose:
	@echo "$(GREEN)Запуск тестів з детальним виводом...$(NC)"
	cargo test --all -- --nocapture

## Форматування коду
fmt:
	@echo "$(GREEN)Форматування коду...$(NC)"
	cargo fmt --all

## Перевірка форматування без змін
fmt-check:
	@echo "$(GREEN)Перевірка форматування...$(NC)"
	cargo fmt --all -- --check

## Лінтинг коду
clippy:
	@echo "$(GREEN)Лінтинг з Clippy...$(NC)"
	cargo clippy --all-targets --all-features -- -D warnings

## Генерація документації
doc:
	@echo "$(GREEN)Генерація документації...$(NC)"
	cargo doc --no-deps --open

## Генерація документації без відкриття
doc-build:
	@echo "$(GREEN)Генерація документації...$(NC)"
	cargo doc --no-deps

## Перевірка перед комітом (форматування + лінтинг + тести)
pre-commit: fmt clippy test
	@echo "$(GREEN)✓ Всі перевірки пройдені успішно!$(NC)"

## Очистка збірки
clean:
	@echo "$(GREEN)Очистка...$(NC)"
	cargo clean
	@rm -f example.carbon

## Перевірка безпеки залежностей
audit:
	@echo "$(GREEN)Перевірка безпеки залежностей...$(NC)"
	cargo audit

## Оновлення залежностей
update:
	@echo "$(GREEN)Оновлення залежностей...$(NC)"
	cargo update

## Встановлення проекту глобально
install:
	@echo "$(GREEN)Встановлення carbon-parser...$(NC)"
	cargo install --path .

## Видалення встановленого проекту
uninstall:
	@echo "$(GREEN)Видалення carbon-parser...$(NC)"
	cargo uninstall carbon-parser

## Створення тестового файлу
create-example:
	@echo "$(GREEN)Створення тестового файлу example.carbon...$(NC)"
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
	@echo "Файл створено: example.carbon"

## Парсинг тестового файлу
parse-example: create-example
	@echo "$(GREEN)Парсинг example.carbon...$(NC)"
	cargo run -- parse example.carbon --verbose

## Перевірка покриття тестами (потребує cargo-tarpaulin)
coverage:
	@echo "$(GREEN)Аналіз покриття тестами...$(NC)"
	cargo tarpaulin --out Html --output-dir coverage

## Бенчмарки (якщо є)
bench:
	@echo "$(GREEN)Запуск бенчмарків...$(NC)"
	cargo bench

## Показати допомогу
help:
	@echo "$(GREEN)Доступні команди:$(NC)"
	@echo "  make build         - Збірка проекту"
	@echo "  make run           - Запуск програми з прикладом"
	@echo "  make test          - Запуск всіх тестів"
	@echo "  make test-verbose  - Запуск тестів з детальним виводом"
	@echo "  make fmt           - Форматування коду"
	@echo "  make fmt-check     - Перевірка форматування"
	@echo "  make clippy        - Лінтинг коду"
	@echo "  make doc           - Генерація та відкриття документації"
	@echo "  make doc-build     - Генерація документації"
	@echo "  make pre-commit    - Всі перевірки перед комітом"
	@echo "  make clean         - Очистка збірки"
	@echo "  make audit         - Перевірка безпеки"
	@echo "  make update        - Оновлення залежностей"
	@echo "  make install       - Встановлення глобально"
	@echo "  make uninstall     - Видалення"
	@echo "  make create-example - Створення тестового файлу"
	@echo "  make parse-example - Парсинг тестового файлу"
	@echo "  make coverage      - Аналіз покриття"
	@echo "  make help          - Показати цю допомогу"