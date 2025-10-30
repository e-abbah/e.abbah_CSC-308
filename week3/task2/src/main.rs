struct Circle {
    radius: f64,
}

impl Circle {
    const PI: f64 = 3.14159; //This is how you define a constant in an impl block
    
    fn area(&self) -> f64 {
        // const PI: f64 = 3.14159; This works too
        self.radius * self.radius * Circle::PI
    }

    fn circumference(&self) -> f64 {
        // const PI: f64 = 3.14159; This works too
        2.0 * Circle::PI * self.radius
    }
}


fn main() {
    let circle = Circle { radius: 5.0 };
    println!("The area of the circle is: {}", circle.area());
    println!("The circumference of the circle is: {}", circle.circumference());

    
}