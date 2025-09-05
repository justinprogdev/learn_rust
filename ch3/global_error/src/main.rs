use core::panic;

use rand::random;

static mut ERROR: isize = 0;
struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[allow(unused_variables)]
fn main() {
    let f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);

    unsafe {
        if ERROR != 0 {
            panic!("an error has occurred!")
             // thread 'main' panicked at src/main.rs:31:7:
            // an error has occurred!
            // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        }
    }
}
