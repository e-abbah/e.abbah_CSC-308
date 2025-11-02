use std::io::{self, Write};

struct Student {
    username: String,
    score: f32,
}

impl Student {
    fn grade_student(&self) -> bool {
        self.score >= 50.0
    }
}

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    print!("Enter your score: ");
    io::stdout().flush().unwrap();
    let mut score = String::new();
    io::stdin().read_line(&mut score).expect("Failed to read line");
    let score: f32 = score.trim().parse().expect("Not a valid floating point");

    let student = Student {
        username: name.to_string(),
        score: score,
    };

    if student.grade_student() {
        println!("Student Passed");
    } else {
        println!("Student Failed");
    }
}
