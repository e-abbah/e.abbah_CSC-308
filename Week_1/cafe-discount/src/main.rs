fn main() {
    let bill: f32 = 12000.0;
    println!("Original Bill: N{}", bill);
    if bill > 5000.0 && bill < 10000.0{
        let fbill: f32 = 0.9 * bill;
        println!("Discount Applied: 10%");
        println!("Final bill: {}", fbill);

    } else if bill > 10000.0{
        let fbill: f32 = 0.85 * bill;
        println!("Discount Applied: 15%");
        println!("Final bill: {}", fbill);

    } else {
        println!("No Discount");
    }
}
