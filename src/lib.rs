use std::fs;
use std::error::Error;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query: &str = "HaYsTaCk";
        let contents: &str = "\
        testline \nNeedle in a haystack\nyoubehaysacking";
        assert_eq!(vec!["Needle in a haystack"],search_case_insensitive(query, contents));
    }
}

pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    let mut lines_found = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            lines_found.push(line);
        }
    }
    lines_found
}

pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str>{
    let mut lines_found = Vec::new();
    let lower_query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&lower_query){
            lines_found.push(line);
        }
    }
    lines_found
}


pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }
    // better implementation Lifetime ends here with code below
    //
    // for line in search(&config.query, &contents) {
    //   println!("{line}");
    //}
    Ok(())

}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments provided need 1.querystring 2.filepath");
        }
        let add_arguments = &args[3..];
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        for arg in add_arguments {
            match &arg {
                &s if s == "--i" =>  {
                    ignore_case = true;
                },
                _s => println!("No Arguments found"),

            }
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { 
             query,
             file_path,
             ignore_case
        })
    }
}
