pub struct Config {
    pub query: String,
}

impl Config {
    pub fn query(query: &str) -> Config {
        Config {
            query: query.to_string(),
        }
    }

    pub fn cli() -> Result<Config, &'static str> {
        let mut args = std::env::args();
        args.next(); // skip exe path
        Config::new(args)
    }

    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        Ok(Config { query })
    }
}
