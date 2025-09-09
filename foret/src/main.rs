use std::io;

fn main() {
    println!("Welcome to the Simple Calculator!");

    
    let num1 = loop {
        println!("Enter the first number:");
        match read_number() {
            Ok(n) => break n,
            Err(e) => println!("Error: {}", e),
        }
    };

   
    let num2 = loop {
        println!("Enter the second number:");
        match read_number() {
            Ok(n) => break n,
            Err(e) => println!("Error: {}", e),
        }
    };

    
    let operation = loop {
        println!("Enter an operation (+, -, *, /):");
        let op = read_line().trim().to_string();
        if ["+", "-", "*", "/"].contains(&op.as_str()) {
            break op;
        } else {
            println!("Invalid operation. Please enter one of +, -, *, /.");
        }
    };

    
    let result = calculate(num1, num2, &operation);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Division by zero"),
    }
}


fn read_number() -> Result<f64, String> {
    let input = read_line();
    input.trim().parse::<f64>().map_err(|_| "Please enter a valid number.".to_string())
}


fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}


fn calculate(num1: f64, num2: f64, op: &str) -> Option<f64> {
    match op {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => None,
    }
}
