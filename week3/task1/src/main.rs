use std::io;
fn main() {
    println!("Enter some text:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = String::from(input);
    let mut count = 0;
    let mut last_word = "";
    

    for word in s.split_whitespace() {
        println!("{}", word);
        last_word = word;
        
    }
    println!("Last word: {}", last_word);

        
    // if let Some(index) = s.rfind('\t') {
    //     println!("Last tab character at index: {}", index);
    // }
    // if let Some(index) = s.rfind(|c: char| c.is_whitespace()) {
    //     println!("Last whitespace found at index: {}", index );
    //     if let Some(last_word) = s.split_whitespace().last() {
    //         println!("Last word is: {}", last_word);
    //     }
    // }
}