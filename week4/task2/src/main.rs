fn main() {
    
    let list: Vec<i32> = (1..=20).collect();
    let mut v: Vec<i32> = Vec::new();

    
    let is_even = |x: i32| x % 2 == 0;

    
    for &num in &list {
        if is_even(num) {
            v.push(num);
        }
    }

    println!("Even numbers: {:?}", v);
}
