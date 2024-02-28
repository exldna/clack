fn main() {
    let config = clack::Config::from_cli()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            std::process::exit(1);
        });
    println!("{:?}", clack::calc(config))
}
