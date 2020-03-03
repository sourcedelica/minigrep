use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;


fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
