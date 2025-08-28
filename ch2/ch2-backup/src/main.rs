use std::{fmt::LowerHex, fs::read_dir, thread::Builder};

fn main() {
    let path: &str = "C:\\Source";
    
    let log_builder: String;
    log_builder.push_str(&("Backup Path:".to_owned() + path));

    if let Ok(entries) = read_dir(path) {
        
        for entry in entries {
            
            let ent = entry.unwrap();
                log_builder.push_str(&("File Name: ".to_owned() + &ent.file_name().into_string().unwrap()));
            if let Ok(metadata) = ent.metadata() {
                // Now let's show our entry's permissions!
                println!("File Type{:?}: {:?}", ent.path(), metadata.file_type());
            } else {
                println!("Couldn't get metadata for {:?}", ent.path());
            }
            count += 1;
        }
    } else {
        println!("Experienced and error reading files from {}", path);
    }
}

// /// copy file in entirety from from_path to to_path
// fn copy_file(from_path: String, to_path: String) {}
