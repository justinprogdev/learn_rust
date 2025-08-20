// Lifetime is a name that tells the compiler how long the variable is valid
// Essentially makes sure never have a dangling pointer
fn add_w_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add_w_lifetimes(&a, &b);
    println!("{}", res);
} 