fn main() {
    if let Err(error) = shield_cli::run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}
