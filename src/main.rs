use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config;
    match bsq::Config::new(&args) {
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            std::process::exit(1);
        }
        Ok(c) => config = c,
    };

    match bsq::run(config) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
