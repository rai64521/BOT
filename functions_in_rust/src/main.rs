fn main() {
    let name = "Sumant";
    greet(name);

    let sum = add(5, 7);
    println!("5 + 7 = {}", sum);
}

/// Prints a greeting message
fn greet(name: &str) {
    println!("Hello, {}!", name);
}


fn add(a: i32, b: i32) -> i32 {
    a + b
}
