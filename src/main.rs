use std::io;

fn main() {
    println!("Hello, world!");

    let set_operator: char = get_operator_choice();

    let nb1: f32 = get_number();
    let nb2: f32 = get_number();

    let result: f32 = set_calc(set_operator, nb1, nb2);
    println!("The result is: {}", result);
}

fn get_operator_choice() -> char {
    let mut operator = String::new();
    println!("Please choose an operator for your calculation.");
    println!(
        " \nYou have the choice: \n\nSoustraction: -\nAddition: +\nDivision: /\nMultiplication: *\n"
    );

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let operator: char = operator.trim().chars().next().unwrap();

    println!("my operator choice: {}", operator);

    operator
}

fn get_number() -> f32 {
    let mut number = String::new();
    println!("Please type a number: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f32 = number.trim().parse().expect("Please type a number!");
    number
}

fn set_calc(set_operator: char, nb1: f32, nb2: f32) -> f32 {
    let mut result: f32 = 0.0;

    match set_operator {
        '+' => result = nb1 + nb2,
        '-' => result = nb1 - nb2,
        '*' => result = nb1 * nb2,
        '/' => result = nb1 / nb2,
        _ => println!("something else"),
    }

    result
}
