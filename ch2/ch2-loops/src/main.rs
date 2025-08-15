fn main() {
    // If you loop over a collection,
    // rust ends that containers lifetime,
    // unless you use & for referencing it

    // for item in &container {
    // deez items are only referenced
    // }

    // Anonomous Loops
    let mut accumulator = 0;

    // Can use _ as placeholder if not used within block
    for _ in 0..20 {
        accumulator += 1;
    }
    print!("{}", accumulator);

    // Break from nested loops with labels
    'outer: for x in 0.. {
        for y in 0.. {
            'inner: for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer;       // <--- Break outer loop
                }
                else if z > 5 {
                    break 'inner;        // <--- Break inner loop
                }
            }
        }
    }
}
