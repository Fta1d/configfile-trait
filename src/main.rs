use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

#[derive(Debug)]
struct Logger {
    ver: String,
    thread: String,
    os: String
}

pub trait ConfigFile {
    fn load_cfg(&mut self, path: &str) -> Result<(), Box<dyn Error>>;
}

impl ConfigFile for Logger {
    fn load_cfg(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file= File::open("cfg.txt").map_err(|e| {
            eprintln!("Error during opening file: {}", e);
            e
        })?;

        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    continue;
                }

                if let Some((key, value)) = trimmed.split_once(':') {
                    match key.trim() {
                        "ver" => self.ver = value.trim().to_string(),
                        "thread" => self.thread = value.trim().to_string(),
                        "os" => self.os = value.trim().to_string(),
                        _ => {} 
                    }
                }

            } else {
                eprintln!("Error during reading line, skipping...");
                continue;
            }
            
        }
        Ok(())
    }
}

fn main() {
    let mut logger = Logger {
        ver: String::new(),
        thread: String::new(),
        os: String::new(),
    };

    if let Err(e) = logger.load_cfg("config.cfg") {
        eprintln!("Error loading config: {}", e);
    } else {
        println!("Config loaded: {:?}", logger);
    }

}
