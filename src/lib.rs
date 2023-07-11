use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::error::Error;
use dotext::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    for path in config.file_dir.into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().path())
        .filter(|r| r.is_file()) {
        println!("Checking: {:?}", path);

        let t: String = check_type(&path).parse().unwrap();

        println!("Inputting {:}", t);

        match t.as_str(){
            "docx" => {
                let mut file = Docx::open(path).expect("Cannot open file. ::");
                let mut buf = String::new();
                file.read_to_string(&mut buf).expect("Couldn't read open file. ::");
                println!("Opening DOCX ::");
                println!("{buf}\n");
            },

            "txt" | "py" | "cpp" | "rs" | "md" => {
                let content = fs::read_to_string(path).unwrap();
                let res = search(&config.query, &content);

                println!("Opening FILE ::");
                for lines in res{
                    println!("{lines}");
                }
                println!("");
            },

            _ => {
                println!("Not openable ::\n");
            },
        };
    }



    Ok(())
}

fn check_type(path: &PathBuf) -> String{
    let hold: String = String::from(path.display().to_string());
    let ret: &Vec<&str> = &hold.split(".").collect();
    print!("Type Check: {:?}\n", ret);
    return ret.get(2).unwrap().to_string();
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


pub struct Config {
    query: String,
    file_dir: fs::ReadDir,
}

impl Config {
    pub fn build<'a>(args: &'a[String], directs: fs::ReadDir) -> Result<Config, &'a str>{
        if args.len() < 2 {
            return Err("not enough args");
        }
        Ok(Config{query: args.get(1).unwrap().clone(), file_dir: directs})
    }
}