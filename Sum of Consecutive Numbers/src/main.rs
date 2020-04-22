use std::io;

fn main() {
    println!("WELCOME TO UMESH'S COMMAND LINE APPS FOR YOUR DAILY USE.");
    println!("This one will help you to find your sum for consecutive numbers starting from 1 and ends till where you want it!");
    println!();

    main_setup();

}
fn main_setup() {
    loop {
        println!("Enter the last number you want to find the sum of or type exit to quit: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let valid_input = input.trim().parse::<i64>().is_ok();
        if valid_input {
            let input: i64 = input.trim().parse().expect("A number was not entered.");

            match &input {
                2..=90000 => {
                    let mut a = 1;
                    let mut b = 2;

                    let mut times = 2;

                    while b <= input {
                        a = a + b;

                        println!("{}. {}", times, a);
                        b += 1;
                        times += 1;
                    }
                },
                _ => {
                    println!("Out of Range!");
                    println!("Try Again!");
                    main_setup();
                }
            }
        }
        else {
            let input = input.trim();
            if input == "exit" || input == "quit" {
                break;
            }
            else{

            println!("invalid input! Please enter a number to see your results.");
            main_setup();
            }
        }
    }
}
