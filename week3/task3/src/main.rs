struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn check_balance(&self) -> f64 {
        self.balance
    }
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }
    fn withdraw(&mut self, amount: f64){
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }  
}

fn main() {
    let mut account = BankAccount { balance: 1000.0 };

    // Getting input from users
    // println!("Welcome to ATM operations");
    // let mut input1 = String::new();
    

    println!("Initial balance: ${}", account.check_balance());

    account.deposit(500.0);
    println!("Balance after deposit: ${}", account.check_balance());

    account.withdraw(200.0);
    println!("Balance after withdrawal: ${}", account.check_balance());
}
