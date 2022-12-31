use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Command line property parsing error: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(2);
    }
}

