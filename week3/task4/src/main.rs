use std::io::{self, Write};

fn total_word(s: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    for word in s.split_whitespace() {
        words.push(word.to_string());
    }
    words
}

fn main() {
    print!("Input your sentence: "); 
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let input = input.trim();

    let words = total_word(&input);

    if words.is_empty() {
        println!("No words entered!");
        return;
    }

    let mut longest_word = &words[0];
    let mut shortest_word = &words[0];

    for word in &words[1..] {
        if word.len() > longest_word.len() {
            longest_word = word;
        } 
        if word.len() < shortest_word.len() {
            shortest_word = word;
        }
        
    }

    println!("The longest word is: {}", longest_word);
    println!("The shortest word is: {}", shortest_word);
}
