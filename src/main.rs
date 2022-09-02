use std::io::stdin;

fn main() {
    let word = String::from("hello");

    let original: Vec<char> = word.chars().collect();

    for _ in 1..6 {
        let mut read = String::new();

        stdin().read_line(&mut read)
            .ok()
            .expect("Failed");

        let input = String::from(read.trim());

        assert!(input.chars().count() == 5, "failed");

        let guess: Vec<char> = input.chars().collect();

        if input == word {
            print!("you found the word");
            return;
        }

        let mut res = String::new();
        for i in 0..5 {
            if original[i] == guess[i] {
                res.push_str("*");
            } else if has_char(&word, guess[i]) {
                res.push_str("_");
            } else {
                res.push_str("-");
            }
        }

        println!("{}", res);
    } 

    println!("You failed to find the word, you bozo!");
}

fn has_char(word: &String, c: char) -> bool{
    let arr: Vec<char> = word.chars().collect();
    for i in 0..5 {
        if arr[i] == c {
            return true;
        }
    }
    false
}
