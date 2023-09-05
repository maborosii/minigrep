use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let a = "demo".into();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("parsing params err:{err}");
        process::exit(1);
    });

    if let Err(_e) = run(config) {
        println!("err in read file");
        process::exit(1);
    }
}
