use std::io;

fn main() {
    println!("ATM Simulator");
    let mut balance: f64 = 100.0;

    balance = loop {
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
                println!("Current balance: {}", balance);
            },
            2 => {
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read");
                let amount: f64 = amount.trim().parse().expect("Not a number");
                if amount > balance {
                    println!("Insufficient balance");
                } else {
                    balance -= amount;
                    println!("Withdrawal successful. New balance: {}", balance);
                }
            },
            3 => {
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read");
                let amount: f64 = amount.trim().parse().expect("Not a number");
                if amount <= 0.0 {
                    println!("Invalid amount");
                } else {
                        balance += amount;
                        println!("Deposit successful. New balance: {}", balance);
                }
            },
            4 => {
                break balance;
            },
            _ => {
                println!("Invalid choice");
            },
        }
    };

    println!("Thank you for using the ATM. Your final balance is: {}", balance);
}
