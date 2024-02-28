mod config;
pub use config::Config;

mod arithmetic;
pub use arithmetic::{Bracket, Operator, Number};

mod tokenizer;
pub use tokenizer::Token;
pub use tokenizer::Tokenize;
pub use tokenizer::tokenize;
pub use tokenizer::Tokenizer;

pub fn calc(config: Config) -> Result<Number, &'static str> {
    config.query.chars().tokenize().evaluate()
}
