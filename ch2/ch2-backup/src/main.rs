use chrono::Local;
use std::fs::OpenOptions;
use std::{fs::read_dir, io::Write, path::Path};

fn main() {
    //let path: &str = "C:\\Source";
    let path: &str = "/home/mcdizzle/source/learn_rust";

    let mut log_builder = String::new();

    log_builder.push_str(&("Backup Path:".to_owned() + path));

    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            let ent = entry.unwrap();

            log_builder.push_str(&format!(
                "\r\nFile Name: {}",
                &ent.file_name().into_string().unwrap()
            ));

            if let Ok(metadata) = ent.metadata() {
                // Now let's show our entry's permissions!
                log_builder.push_str(&format!(
                    "\r\nFile Type{:?}: {:?}",
                    ent.path(),
                    metadata.file_type()
                ));
            } else {
                log_builder.push_str(&format!("\r\nCouldn't get metadata for {:?}", ent.path()));
            }
        }
    } else {
        log_builder.push_str(&format!(
            "\r\nExperienced an error reading files from {}",
            path
        ));
    }

    let out_path = "out.txt";
    if !Path::new(&out_path).exists() {
        if let Ok(mut r) = std::fs::File::create_new(out_path) {
            if let Ok(r2) = r.write(log_builder.as_bytes()) {
                print!("{}", r2);
            }
        }
    } else {
        let now = Local::now();
        let short = now.format("%Y-%m-%d %H:%M").to_string();
        println!("{}", short);

        let file = OpenOptions::new().append(true).create(true).open(out_path);

        _ = writeln!(file.unwrap(), "\r\n{}\r\n{}", short, log_builder);
    }
}
