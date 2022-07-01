use std::error::Error;
use std::{fs, env};

pub struct Config {
    pub query: String, 
    pub filename: String,
    pub case_sensitive: bool,
}
  
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough parameters");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config { query, filename, case_sensitive })
    }
}
  
  
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    let searchresults = if config.case_sensitive {
    search_sensitive(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };
    for line in searchresults {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_sensitive<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive <'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let lowercase_query = query.to_ascii_lowercase();
    for line in content.lines() {
        if line.to_ascii_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {

        let query = "duct";
        let contents = "\
        do yo need some Duct tape
        Rust is safe, productive and fast
        pick three
        ";

        assert_eq!(vec![
            "        Rust is safe, productive and fast"
        ], search_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "ruSt";
        let contents = "\
        do yo need some duct tape
        Rust is safe, productive and fast
        pick three
        Trust me,
        ";
        assert_eq!(vec!["        Rust is safe, productive and fast",
        "        Trust me,"
        ], search_insensitive(query, contents));

    }

}