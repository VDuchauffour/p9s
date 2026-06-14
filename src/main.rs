fn main() {
    if let Err(e) = metron::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
