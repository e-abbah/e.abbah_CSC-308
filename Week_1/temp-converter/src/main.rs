fn main() {
    let tc: f32 =  36.5;
    println!("Temperature: {}C", tc);
    let tf: f32 = 1.8 * tc + 32.0;
    println!("Converted: {}F", tf);

    let tf: f32 = 100.0;
    print!("");
    println!("Temperature: {}F", tf);

    let tc: f32 = 0.556 * (tf - 32.0);
    println!("Temperature: {:.1}C", tc);

}
