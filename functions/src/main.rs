fn main() {
    let x: f32 = 55.0;
    another_function(x);
    let y = multiply(x, x);

    println!("The value of X^2 is {y}");
}

fn another_function(x: f32) {
    println!("The values of x is: {x}");
}

fn multiply(x: f32, y: f32) -> f32 {
    x * y
}
