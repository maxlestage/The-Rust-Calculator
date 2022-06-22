use std::io;

fn main() {
    println!("Hello, world!");

    let mut operator = String::new();
    let mut number1 = String::new();
    let mut number2 = String::new();

    let set_operator: char = get_operator_choice(&mut operator);

    let nb1 = get_number_1(&mut number1);

    let nb2 = get_number_2(&mut number2);

    let result: f32 = set_calc(set_operator, nb1, nb2);
    println!("The result is: {}", result);
}

fn get_operator_choice(operator: &mut String) -> char {
    println!("Please choose an operator for your calculation.");
    println!(
        " \nYou have the choice: \n\nSoustraction: -\nAddition: +\nDivision: /\nMultiplication: *\n"
    );

    io::stdin()
        .read_line(operator)
        .expect("Failed to read line");

    let operator: char = operator.trim().chars().next().unwrap();

    println!("my operator choice: {}", operator);

    operator
}

fn get_number_1(number1: &mut String) -> f32 {
    println!("Please type a number: ");

    io::stdin().read_line(number1).expect("Failed to read line");

    let number1: f32 = number1.trim().parse().expect("Please type a number!");
    number1
}

fn get_number_2(number2: &mut String) -> f32 {
    println!("Please type a number: ");

    io::stdin().read_line(number2).expect("Failed to read line");

    let number2: f32 = number2.trim().parse().expect("Please type a number!");
    number2
}

fn set_calc(set_operator: char, number1: f32, number2: f32) -> f32 {
    let mut result: f32 = 0.0;

    if set_operator == '+' {
        result = number1 + number2
    } else if set_operator == '-' {
        result = number1 - number2
    } else if set_operator == '*' {
        result = number1 * number2
    } else if set_operator == '/' {
        result = number1 / number2
    }

    result
}
