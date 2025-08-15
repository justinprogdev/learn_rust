fn main() {
    let x = add(add(1, 2), add(1, 2));
    println!("X = {x}");
}

// fn name (arg: type, arg: type)
fn add(a: i32, b: i32) -> i32 {
    a + b
}
