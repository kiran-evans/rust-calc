fn main() {
    println!("Welcome to rust-calc");

    println!("Please enter your first number.");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    
    println!("Please choose your operation. A | S | M | D");
    let mut operation_input = String::new();
    std::io::stdin().read_line(&mut operation_input).expect("Failed to read line");

    println!("Please enter your second number.");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");

    // Convert operation input to a char
    let selected_operation: char = operation_input.to_uppercase().chars().next().unwrap();

    // Get first character of input String (cut off the newline character) and then parse it. Match the Result to ensure a valid number is provided
    let num1: i32 = match String::from(input1.chars().next().unwrap()).parse() {
        Ok(num) => num,
        Err(error) => panic!("Failed to parse {input1}. {error}")
    };
    let num2: i32 = match String::from(input2.chars().next().unwrap()).parse() {
        Ok(num) => num,
        Err(error) => panic!("Failed to parse {input2}. {error}")
    };

    match selected_operation {
        'A' => add(num1, num2),
        'S' => subtract(num1, num2),
        'M' => multiply(num1, num2),
        'D' => divide(num1, num2),
        _ => println!("Not a valid option")
    }
}

fn add (x: i32, y: i32) {
    let result = x + y;
    println!("{x} + {y} = {result}");
}

fn subtract (x: i32, y: i32) {
    let result = x - y;
    println!("{x} - {y} = {result}");
}

fn multiply (x: i32, y: i32) {
    let result = x * y;
    println!("{x} x {y} = {result}");
}

fn divide (x: i32, y: i32) {
    let result = x / y;
    println!("{x} / {y} = {result}");
}