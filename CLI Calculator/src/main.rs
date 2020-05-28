use std::io;
use std::io::Write;
fn read(input: &mut String) {

    io::stdout().flush().expect("Unable to flush ");
    io::stdin().read_line(input).expect("Failed to read");


}

fn read_operator(input: &mut String) {
    io::stdout().flush().expect("Unable to flush ");
    io::stdin().read_line(input).expect("Failed to read");
}

fn isnumber(num: &str) -> bool {
    return num.trim().parse::<i64>().is_ok();
}

fn convert_to_number(num: &str) -> i64 {
    return num.trim().parse().expect("Can't convert to number");
}

fn result(output: &i64) {
    println!("The result is: {}", output);
    println!("Press any key to quit");
    exit();
}

fn exit() {
    let mut exit = String::new();
    read(&mut exit);

    io::stdin()
        .read_line(&mut exit)
        .expect("Doesn't matter. Its gonna exit anyway!");
}

fn main() {
    println!("WELCOME TO UMESH'S COMMAND LINE APPS FOR YOUR DAILY USE.");
    println!("This one is a CLI Calculator");

    calculator();
}

fn calculator() {

    let mut num1 = String::new();
    print!("Enter the first number: ");
    read(&mut num1);
    let num1 = num1.trim();



    let mut num2 = String::new();
    print!("Enter the second number: ");
    read(&mut num2);
    let num2 = num2.trim();

    let mut operator = String::new();
    print!("Enter the operation you want to perform(| + | - | * | / |): ");
    read_operator(&mut operator);
    let operator = operator.trim();

    match operator {
        "+"=> {
            if isnumber(num1) && isnumber(num2) {
                let num1: i64 = convert_to_number(&num1);
                let num2: i64 = convert_to_number(&num2);

                let output = num1 + num2;
                result(&output);
            } else {
                println!(
                    "Invalid Input. Please check whether both the numbers are integers or floating point decimalsor simply decimals"
                );
                calculator();
            }
        }
        "-" => {
            if isnumber(num1) && isnumber(num2) {
                let num1: i64 = num1.trim().parse().expect("Can't convert to number");
                let num2: i64 = num2.trim().parse().expect("Can't convert to number");

                let output = num1 - num2;
                result(&output);
            } else {
                println!(
                    "Invalid Input. Please check whether both the numbers are integers or
                floating point decimals(or simply decimals)"
                );
                calculator();
            }
        }
        "*" => {
            if isnumber(num1) && isnumber(num2) {
                let num1: i64 = num1.trim().parse().expect("Can't convert to number");
                let num2: i64 = num2.trim().parse().expect("Can't convert to number");

                let output = num1 * num2;
                result(&output);
            } else {
                println!(
                    "Invalid Input. Please check whether both the numbers are integers or
                floating point decimals(or simply decimals)",
                );
                calculator();
            }
        }
        "/" => {
            if isnumber(num1) && isnumber(num2) {
                let num1: i64 = num1.trim().parse().expect("Can't convert to number");
                let num2: i64 = num2.trim().parse().expect("Can't convert to number");

                if num2 == 0 {
                    println!("Not Defined");
                } else {
                let output = num1 / num2;
                result(&output);
                }

            } else {
                println!(
                    "Invalid Input. Please check whether both the numbers are integers or
                floating point decimals(or simply decimals)",
                );

            }
        }
        _ => {
            println!("Invalid Entries.\nPlease try again!");
            calculator();



        }
    }
}
