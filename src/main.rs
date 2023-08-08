use std::env;
use std::process;
use mini_grep::Config;
fn main() {
    
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem while parsing config data: {err}");
        process::exit(1);
    });
    println!("In file {}",config.query);
    if let Err(e) = mini_grep::run(config) {
        println!("Encountered an Error while running the Application: {e}");
        process::exit(1);
    }
    

}


