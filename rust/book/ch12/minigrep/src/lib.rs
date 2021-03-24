use std::env;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Two arguments are required");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_with_valid_args() {
        let args = ["".to_string(), "foo".to_string(), "bar".to_string()];
        assert_eq!(
            Config::new(&args).unwrap(),
            Config {
                query: String::from("foo"),
                filename: String::from("bar"),
                case_sensitive: true
            }
        );
    }

    #[test]
    fn config_new_with_too_few_args() {
        let args = ["".to_string()];
        assert_eq!(
            Config::new(&args).unwrap_err(),
            "Two arguments are required"
        );
    }

    #[test]
    fn run_with_invalid_filename() {
        let config = Config {
            query: "".to_string(),
            filename: "".to_string(),
            case_sensitive: false,
        };
        assert!(run(config).is_err());
    }

    #[test]
    fn test_search_case_sensitivity() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
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
