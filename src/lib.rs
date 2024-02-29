mod config;

pub use config::Config;

pub mod arithmetic;

mod tokenizer;

pub use tokenizer::ReversePolishNotation;
pub use tokenizer::Token;
pub use tokenizer::Tokenizer;

mod parser;

pub use parser::tokenize;
pub use parser::Parser;
pub use parser::Tokenize;

pub fn calc(config: Config) -> Result<arithmetic::Number, &'static str> {
    tokenize(&config.query).evaluate()
}
