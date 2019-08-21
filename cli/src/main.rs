extern crate clap;

use clap::{Arg, App};


fn main() {
    let matches = App::new("cli")
        .version("0.1.0")
        .author("Brett Slaski <brettski@yahoo.com>")
        .about("Testing cli tools in Rust")
        .arg(Arg::with_name("url")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("A url we'll do something with"))
        .get_matches();
    let url = matches.value_of("url").unwrap();
    println!("{}", url);
}