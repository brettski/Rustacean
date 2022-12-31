use std::fs;
use std::error::Error;

// Run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("> Searching for {}", config.query);
    println!("> In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    println!("> With text:\n{contents}");
    Ok(())
}


// Config
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments. Expect: <search value> <file>");
        }
        // Using clone here to provide Config with it's own version of the arguments
        // does increase runtime cost a bit. It does help with clarity over using lifetime in
        // this simple example application. Oh, and I really don't understand lifetimes yet 😉.
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
