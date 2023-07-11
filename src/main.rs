use std::fs;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    let directs = fs::read_dir("./").unwrap();

    let config = multigrep::Config::build(&args, directs).unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = multigrep::run(config){
        eprintln!("You messed up buddy (probably). {e}");
        process::exit(1);
    }
    
}


