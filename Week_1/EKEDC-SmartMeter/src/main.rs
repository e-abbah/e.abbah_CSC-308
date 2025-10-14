use std::io;

fn main() {
    println!("Enter your electricity usage in kWh:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let usage: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let mut bill = 0;

    if usage <= 100 {
        bill = usage * 20;
    } else if usage <= 200 {
        // First 100 kWh at 20
        bill = 100 * 20;
        // Remaining kWh at 25
        bill += (usage - 100) * 25;
    } else {
        // First 100 kWh at 20
        bill = 100 * 20;
        // Next 100 kWh at 25
        bill += 100 * 25;
        // Remaining kWh above 200 at 30
        bill += (usage - 200) * 30;
    }

    println!("Your total electricity bill is: #{}", bill);
}
