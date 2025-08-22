use std::mem::{size_of, align_of};
use std::slice;

fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    // i32 is 4 bytes (4 bytes * 8 bits = 32)
    // Array of 5 i32'a is ( 5 * 4 = 20 bytes)
    println!("T = i32, size = {}, align = {}", size_of::<i32>(), align_of::<i32>());
    println!("arr size = {} bytes", size_of::<[i32; 5]>());
    println!("arr addr = {:p}", &arr); // Just the address to memory

    for i in 0..arr.len() {
        // Addresses of each element — expect +4 bytes each step on a 32-bit element
        let p = &arr[i] as *const i32;
        println!("  arr[{i}] @ {:p} = {}", p, arr[i]);
    }

    // Hex dump the entire array as raw bytes
    unsafe {
        let byte_len = size_of::<[i32; 5]>();
        let bytes: &[u8] = slice::from_raw_parts(arr.as_ptr() as *const u8, byte_len);
        print!("bytes: ");
        for (k, b) in bytes.iter().enumerate() {
            print!("{:02X}{}", b, if (k + 1) % 4 == 0 { " " } else { " " });
        }
        println!();
    }
}

// use std::mem::{size_of, align_of};
// use std::slice;

// #[repr(C)]               // Fix field order/ABI to make padding obvious
// struct Padded {
//     a: u8,               // 1 byte
//     // 3 bytes of padding likely here to align the next u32
//     b: u32,              // 4 bytes, aligned to 4
// }

// fn main() {
//     let arr: [Padded; 3] = [
//         Padded { a: 1, b: 0x11223344 },
//         Padded { a: 2, b: 0x55667788 },
//         Padded { a: 3, b: 0x99AABBCC },
//     ];

//     println!("T = Padded, size = {}, align = {}", size_of::<Padded>(), align_of::<Padded>());
//     println!("arr size = {} bytes", size_of::<[Padded; 3]>());
//     println!("arr addr = {:p}", &arr);

//     for i in 0..arr.len() {
//         let p = &arr[i] as *const Padded;
//         println!("  arr[{i}] @ {:p} (element span = {} bytes)", p, size_of::<Padded>());
//     }

//     unsafe {
//         let bytes = slice::from_raw_parts(arr.as_ptr() as *const u8, size_of::<[Padded; 3]>());
//         print!("bytes: ");
//         for (k, b) in bytes.iter().enumerate() {
//             print!("{:02X}{}", b, if (k + 1) % size_of::<Padded>() == 0 { " | " } else { " " });
//         }
//         println!();
//     }
// }
