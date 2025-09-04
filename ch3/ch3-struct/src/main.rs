#[derive(Debug)]

// Declare a struct to compose our File
struct File {
    name: String,
    data: Vec<u8>,
}

// Use impl method definition to make the ctor
impl File {
    fn new(name: &str) -> File {
        // <1> As `File::new()` is a completely normal function--rather than something blessed by the language--we need to tell Rust that it will be returning a `File` from this function
        File {
            // <2>
            name: String::from(name), // <2> `File::new()` does little more than encapsulate the object creation syntax
            data: Vec::new(),         // <2>
        }
    }

    // fn len(&self) -> usize {  // <3> `File::len()` takes an implicit argument `self`. You'll notice that there is no explicit argument provided on line 25.
    //   self.data.len() // <4> `usize` is the type returned by `Vec<T>::len()`, which is sent directly through to the caller
    // }

    // Create a file with data from our 'new' ctor
    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    // Notice the self: keywork. This allows to extend a type for using .notation for method access
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}
fn open(f: &mut File) -> bool {
    true    
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let f3_data: Vec<u8> = vec![
        114,117,115,116,33
    ];

    let mut f3 = File::new_with_data("2.txt", f3_data);
    let mut buffer: Vec<u8> = vec![];
    open(&mut f3);
    let f3_length = f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
}
