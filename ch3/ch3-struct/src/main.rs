// Allows printlin! to print file
#[derive(Debug)]

struct File {
    name: String,
    data: vec<u8>,
}

fn main() {
    let f1 = File {
        name: String ::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    
}