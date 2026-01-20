fn main() {
    let x: i32 = 95;
    let grade = score(x);
    println!("Grade: {}", grade);
}

fn score(score: i32) -> char {
    match score {
        90..=100 => 'A', // you can also use: 90..=100 => {return 'A';}
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'I',
    }
}
