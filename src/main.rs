use std::io;
fn main() {
    println!("Input a positive integer");
    let mut num_1 = String::new();
    io::stdin()
        .read_line(&mut num_1)
        .expect("Error");
    let num_1: u32 = match num_1.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("please input a number ({})", e);
            return;
        }
    };
    println!("Input another positive integer");
    let mut num_2 = String::new();
    io::stdin()
        .read_line(&mut num_2)
        .expect("Error");

    let num_2: u32 = match num_2.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("please input a number ({})", e);
            return;
        }
    };

    println!("Select an operation by inputting a number between 1 and 4:");
    println!("1. +");
    println!("2. -");
    println!("3. *");
    println!("4. /");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Error");

    let operation: u32 = match operation.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("please input a number ({})", e);
            return;
        }
    };

    let answer;

    match operation {
        1 => answer = add(num_1, num_2),
        2 => answer = subtract(num_1, num_2),
        3 => answer = multiply(num_1, num_2),
        4 => answer = divide(num_1, num_2),
        _ => panic!("{} is not a valid operation", operation) 
    };

    println!("{}", answer)
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn subtract(x: u32, y: u32) -> u32 {
    x - y
}

fn multiply(x: u32, y: u32) -> u32 {
    x * y
}

fn divide(x: u32, y: u32) -> u32 {
    x / y
}
