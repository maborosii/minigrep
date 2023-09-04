use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // let query = args[1].clone();
        // let file_path = args[2].clone();

        args.next();
        let query = match args.next() {
            Some(args) => args,
            None => return Err("cannot  get query string"),
        };

        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("cannot  get query string"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
