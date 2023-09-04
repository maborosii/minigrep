mod config;

pub use config::Config;
use std::error::Error;
use std::fs;

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    let query_insensitive = query.to_lowercase();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query_insensitive) {
    //         results.push(line);
    //     }
    // }
    // results
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_insensitive))
        .collect()
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_test() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
