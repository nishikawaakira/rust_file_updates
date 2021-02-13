use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::env;
use chrono::{DateTime, TimeZone, Local};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ディレクトリ名を指定してください");
        return;
    }

    let mut fileinfo: FileInfo = FileInfo::new();
    match &fileinfo.read_dir(&args[1]) {
        Ok(_) => fileinfo.print(),
        Err(e) => eprintln!("{}", e)
    }

}


struct FileInfo {
    map: HashMap<DateTime<Local>, Vec<String>>,
}

impl FileInfo {
    pub fn new() -> Self {
        let map: HashMap<DateTime<Local>, Vec<String>> = HashMap::new();
        FileInfo {map: map}
    }

    pub fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<String>{

        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            if metadata.is_dir() {
                if let Err(e) = &self.read_dir(entry.path().display().to_string()) {
                    eprintln!("{}", e);
                }
            } else {
                let modified = metadata.modified();
                if modified.is_err() {
                    continue;
                }
                
                let duration = modified.unwrap().duration_since(std::time::SystemTime::UNIX_EPOCH);
                if duration.is_err() {
                    continue;
                }

                let datetime: DateTime<Local> = Local.timestamp(duration.unwrap().as_secs() as i64, 0);    
                if let Some(files) = self.map.get_mut(&datetime) {
                    files.push(entry.path().display().to_string());
                } else {
                    let filename = vec![entry.path().display().to_string()];
                    self.map.insert(datetime, filename);
                }
            }
        }

        Ok(String::from("Ok"))
    }

    pub fn print(&self) {
        // std::vec::Vec<
        // (&chrono::datetime::DateTime<chrono::offset::local::Local>
        // , &std::vec::Vec<std::string::String>)>
        let mut sorted: Vec<_> = self.map.iter().collect();
        sorted.sort_by_key(|a| a.0);
        
        for (key, values) in sorted.iter() {
            for value in values.iter() {
                println!("{} {}", key, value);
            }
        }
        
    }
}

