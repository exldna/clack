fn main() {
    let config = clack::Config::cli()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            std::process::exit(1);
        });
    let result = clack::calc(config).
        unwrap_or_else(|err| {
            eprintln!("Failed to evaluate query: {err}");
            std::process::exit(0);
        });
    println!("{result:?}")
}
