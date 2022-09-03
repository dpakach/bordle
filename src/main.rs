use std::io;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::color;

enum CharState {
    Default,
    Red,
    Yellow,

}

pub struct StdOutput {
    stdout: RawTerminal<io::Stdout>,
}

fn main() {

    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let mut output = StdOutput{ stdout };

    const EMPTY: String = String::new();
    let mut words: [String; 6] = [EMPTY; 6];

    write!(
        output.stdout,
        r#"{}{}ctrl + q to exit"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    ).unwrap();
    write!(output.stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();

    output.stdout.flush().unwrap();

    let bordle = "HELLO";

    let mut input = String::new();
    let mut input_len: u8 = 0;
    let mut word_count: u8 = 0;

    for c in stdin.keys() {
        let mut line: u16 = 0;

        write!(
            output.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        line += 1;

        write!(output.stdout, "{}", termion::cursor::Goto(1, line)).unwrap();
        write!(output.stdout, "crtl + q to exit\n").unwrap();
        line += 1;
        write!(output.stdout, "{}", termion::cursor::Goto(1, line)).unwrap();


        for i in  0..word_count {
            display_word(&mut output, &String::from(bordle), &words[usize::from(i)]); // String::from("cehoo"));
            line += 1;
            write!(output.stdout, "{}", termion::cursor::Goto(1, line)).unwrap();
        }

        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Char('a') | Key::Char('A') => {input.push_str("A"); input_len += 1;},
            Key::Char('b') | Key::Char('B') => {input.push_str("B"); input_len += 1;},
            Key::Char('c') | Key::Char('C') => {input.push_str("C"); input_len += 1;},
            Key::Char('d') | Key::Char('D') => {input.push_str("D"); input_len += 1;},
            Key::Char('e') | Key::Char('E') => {input.push_str("E"); input_len += 1;},
            Key::Char('f') | Key::Char('F') => {input.push_str("F"); input_len += 1;},
            Key::Char('g') | Key::Char('G') => {input.push_str("G"); input_len += 1;},
            Key::Char('h') | Key::Char('H') => {input.push_str("H"); input_len += 1;},
            Key::Char('i') | Key::Char('I') => {input.push_str("I"); input_len += 1;},
            Key::Char('j') | Key::Char('J') => {input.push_str("J"); input_len += 1;},
            Key::Char('k') | Key::Char('K') => {input.push_str("K"); input_len += 1;},
            Key::Char('l') | Key::Char('L') => {input.push_str("L"); input_len += 1;},
            Key::Char('m') | Key::Char('M') => {input.push_str("M"); input_len += 1;},
            Key::Char('n') | Key::Char('N') => {input.push_str("N"); input_len += 1;},
            Key::Char('o') | Key::Char('O') => {input.push_str("O"); input_len += 1;},
            Key::Char('p') | Key::Char('P') => {input.push_str("P"); input_len += 1;},
            Key::Char('q') | Key::Char('Q') => {input.push_str("Q"); input_len += 1;},
            Key::Char('r') | Key::Char('R') => {input.push_str("R"); input_len += 1;},
            Key::Char('s') | Key::Char('S') => {input.push_str("S"); input_len += 1;},
            Key::Char('t') | Key::Char('T') => {input.push_str("T"); input_len += 1;},
            Key::Char('u') | Key::Char('U') => {input.push_str("U"); input_len += 1;},
            Key::Char('v') | Key::Char('V') => {input.push_str("V"); input_len += 1;},
            Key::Char('w') | Key::Char('W') => {input.push_str("W"); input_len += 1;},
            Key::Char('x') | Key::Char('X') => {input.push_str("X"); input_len += 1;},
            Key::Char('y') | Key::Char('Y') => {input.push_str("Y"); input_len += 1;},
            Key::Char('z') | Key::Char('Z') => {input.push_str("Z"); input_len += 1;},
            _ => (),
        }


        for c in input.chars() {
            write!(output.stdout, "{}", c).unwrap();
        }
        write!(output.stdout, "{}", termion::cursor::Goto(u16::from(input_len) + 1, line)).unwrap();
        if input_len == 5  {
            words[usize::from(word_count)] = input;
            word_count += 1;
            input_len = 0;

            input = String::new();
            continue;
        }

        if word_count >= 5 {
            break;
        }

        output.stdout.flush().unwrap();

        write!(
            output.stdout,
            r#"{}{}"#,
            termion::cursor::Goto(1, 1),
            termion::clear::All
        ).unwrap()
    }
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

fn display_word(out: &mut StdOutput, original: &String, guess: &String) {
    let original_chars: Vec<char> = original.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    for i in 0..5 {
        if original_chars[i] == guess_chars[i] {
            print!("{}", color::Fg(color::Green));
            write!(out.stdout, "{}", guess_chars[i]).unwrap();
        } else if has_char(&original, guess_chars[i]) {
            print!("{}", color::Fg(color::Yellow));
            write!(out.stdout, "{}", guess_chars[i]).unwrap();
        } else {
            print!("{}", color::Fg(color::White));
            write!(out.stdout, "{}", guess_chars[i]).unwrap();
        }
    }
    print!("{}\n", color::Fg(color::White));
}
