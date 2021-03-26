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
    pub fn new<I>(mut args: I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("A query string must be specified"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("A filename must be specified"),
        };
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

fn closure_search<'a, F>(matcher: F, contents: &'a str) -> Vec<&'a str>
where
    F: Fn(&&str) -> bool,
{
    contents.lines().filter(matcher).collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    closure_search(|line| line.contains(query), contents)
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    closure_search(|line| line.to_lowercase().contains(&query), contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_with_valid_args() {
        let args = vec!["".to_string(), "foo".to_string(), "bar".to_string()];
        assert_eq!(
            Config::new(args.into_iter()).unwrap(),
            Config {
                query: String::from("foo"),
                filename: String::from("bar"),
                case_sensitive: true
            }
        );
    }

    #[test]
    fn config_new_without_query() {
        let args = vec!["".to_string()];
        assert_eq!(
            Config::new(args.into_iter()).unwrap_err(),
            "A query string must be specified"
        );
    }

    #[test]
    fn config_new_without_filename() {
        let args = vec!["".to_string(), "".to_string()];
        assert_eq!(
            Config::new(args.into_iter()).unwrap_err(),
            "A filename must be specified"
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
