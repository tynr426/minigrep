use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
    case_sensitive: bool,
}
impl Config {
    pub fn new1(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let filename = args[1].to_string();
        let query = args[2].to_string();
        return Ok(Config {
            filename,
            query,
            case_sensitive: false,
        });
    }
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config {
            filename,
            query,
            case_sensitive: case_sensitive,
        });
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&self.filename)?;
         let results = search_case_insensitive(&self.query,&content);
        for line in results {
            println!("{}", line);
        }
        Ok(())
    }
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