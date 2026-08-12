use std::io;

fn main() {
    println!("ATM Simulator");
    let mut balance: f64 = 0.0;

    loop {
        let mut input = String::new();
        println!("1. Check Balance");
        println!("2. Withdraw");
        println!("3. Deposit");
        println!("4. Exit");
        println!("Enter your choice: ");
        io::stdin().read_line(&mut input).expect("Failed to read");
        let input: u8 = input.trim().parse().expect("Not a number");

        match input {
            1 => {
                todo!()
            },
            2 => {
                todo!()
            },
            3 => {
                todo!()
            },
            4 => {
                todo!()
            },
            _ => {
                todo!()
            },
        }
    }
}
