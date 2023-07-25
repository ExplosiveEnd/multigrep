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

        let typed: String = check_type(&path).parse().unwrap();

        println!("Inputting {:}", typed);

        match typed.as_str(){
            "docx" => {
                let mut file = Docx::open(path).expect("Cannot open file. ::");
                let mut buf = String::new();
                file.read_to_string(&mut buf).expect("Couldn't read open file. ::");
                println!("Opening DOCX ::");
                println!("{buf}\n");
            },

            "xlsx" => {
                let mut file = Xlsx::open(path).expect("Cannot open file. ::");
                let mut buf = String::new();
                file.read_to_string(&mut buf).expect("Couldn't read open file. ::");
                println!("Opening XLSX ::");
                println!("{buf}\n");
            },

            "pptx" => {
                let mut file = Pptx::open(path).expect("Cannot open file. ::");
                let mut buf = String::new();
                file.read_to_string(&mut buf).expect("Couldn't read open file. ::");
                println!("Opening PPTX ::");
                println!("{buf}\n");
            },

            "DS_Store" | "lock" => {
                println!("Not openable ::\n");
            },

            _ => {
                let content = fs::read_to_string(path).unwrap();
                let mut line_nums: Vec<i32> = Vec::new();

                match search(&config.query, &content, &mut line_nums){
                    Some(res) =>{
                        println!("Opening FILE ::");
                        for index in 0..res.len() {
                            println!("[{:}]> {:}", line_nums.get(index).unwrap(), res.get(index).unwrap());
                        }
                        println!();
                    },
                    None => {
                        println!("Found nothing. ::\n");
                    }
                };

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

fn search<'a>(needle: &String, content: &'a String, line_nums: &mut Vec<i32>) -> Option<Vec<&'a str>>{
    let mut result = Vec::new();
    let mut index = 0;
    let query = needle.to_lowercase();
    for line in content.lines(){
        index+=1;
        if line.to_lowercase().contains(&query){
            result.push(line);
            line_nums.push(index);
        }
    }

    match result.len(){
        0 => {
            None
        },
        _ => {
            println!("Found {:} line(s)", result.len());
            Some(result)
        }
    }

}


pub struct Config {
    query: String,
    file_dir: fs::ReadDir,
}

impl Config {
    pub fn build(args: &[String], directs: fs::ReadDir) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough args");
        }
        Ok(Config{query: args.get(1).unwrap().clone(), file_dir: directs})
    }
}