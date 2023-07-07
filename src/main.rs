use std::fs;
use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::error::Error;
use std::process;
use dotext::*; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let directs = fs::read_dir("./").unwrap();

    let config = Config::build(&args, directs).unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprintln!("You messed up buddy (probably). {e}");
        process::exit(1);
    }
    
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    for path in config.file_dir.into_iter()
    .filter(|x| x.is_ok())
    .map(|x| x.unwrap().path())
    .filter(|r| r.is_file()) {
            println!("Checking: {:?}", path);
            
            let mut t: String = match check_type(&path){

                "docx" => {
                    let mut file = Docx::open(path).expect("Cannot open file. ::");
                    let mut buf = String::new();
                    let _ = file.read_to_string(&mut buf);
                    buf
                },
                _ => {
                    let content: String = fs::read_to_string(path)?; 
                    let result: Vec<&str> = search(&config.query, &content);
                    for lines in result{
                        println!("{lines}");
                    }
                    content
                },
            };
    }

    Ok(())
}

fn check_type<'a>(path: &PathBuf) -> &'a str{
    let hold: &str = &path.display().to_string();
    let ret: Vec<&str> = hold.split(".").collect();
    return ret.get(1).unwrap().copy();
}

fn search<'a>(needle: &String, content: &'a String) -> Vec<&'a str>{
    let mut result = Vec::new();
    let query = needle.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}


struct Config {
    query: String,
    file_dir: fs::ReadDir,
}

impl Config {
    fn build<'a>(args: &'a[String], directs: fs::ReadDir) -> Result<Config, &'a str>{
        if args.len() < 2 {
            return Err("not enough args");
        }
        Ok(Config{query: args.get(1).unwrap().clone(), file_dir: directs})
    }
}
