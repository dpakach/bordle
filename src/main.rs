use rand::Rng;
use std::fs;
use std::io;
use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

const OFFSET_X: u16 = 25;
const OFFSET_Y: u16 = 5;

pub struct StdOutput {
    stdout: RawTerminal<io::Stdout>,
}

fn main() {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let mut output = StdOutput { stdout };

    const EMPTY: String = String::new();
    let mut words: [String; 6] = [EMPTY; 6];

    write!(
        output.stdout,
        r#"{}{}ctrl + q to exit"#,
        termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + 1),
        termion::clear::All
    )
    .unwrap();
    write!(
        output.stdout,
        "{}",
        termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + 2)
    )
    .unwrap();

    output.stdout.flush().unwrap();

    let og = get_b();

    let bordle: &str = og.as_str();

    let mut input = String::new();
    let mut input_len: u8 = 0;
    let mut word_count: u8 = 0;

    for c in stdin.keys() {
        let mut line: u16 = 0;

        write!(
            output.stdout,
            "{}{}",
            termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + 1),
            termion::clear::All
        )
        .unwrap();

        line += 1;

        write!(
            output.stdout,
            "{}",
            termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + line)
        )
        .unwrap();
        write!(output.stdout, "crtl + q to exit\n").unwrap();
        line += 3;
        write!(
            output.stdout,
            "{}",
            termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + line)
        )
        .unwrap();

        // Write the words list till now
        for i in 0..word_count {
            display_word(&mut output, &String::from(bordle), &words[usize::from(i)]); // String::from("cehoo"));
            line += 1;
            write!(
                output.stdout,
                "{}",
                termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + line)
            )
            .unwrap();
        }

        // Read the input
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Char('a') | Key::Char('A') => {
                input.push_str("A");
                input_len += 1;
            }
            Key::Char('b') | Key::Char('B') => {
                input.push_str("B");
                input_len += 1;
            }
            Key::Char('c') | Key::Char('C') => {
                input.push_str("C");
                input_len += 1;
            }
            Key::Char('d') | Key::Char('D') => {
                input.push_str("D");
                input_len += 1;
            }
            Key::Char('e') | Key::Char('E') => {
                input.push_str("E");
                input_len += 1;
            }
            Key::Char('f') | Key::Char('F') => {
                input.push_str("F");
                input_len += 1;
            }
            Key::Char('g') | Key::Char('G') => {
                input.push_str("G");
                input_len += 1;
            }
            Key::Char('h') | Key::Char('H') => {
                input.push_str("H");
                input_len += 1;
            }
            Key::Char('i') | Key::Char('I') => {
                input.push_str("I");
                input_len += 1;
            }
            Key::Char('j') | Key::Char('J') => {
                input.push_str("J");
                input_len += 1;
            }
            Key::Char('k') | Key::Char('K') => {
                input.push_str("K");
                input_len += 1;
            }
            Key::Char('l') | Key::Char('L') => {
                input.push_str("L");
                input_len += 1;
            }
            Key::Char('m') | Key::Char('M') => {
                input.push_str("M");
                input_len += 1;
            }
            Key::Char('n') | Key::Char('N') => {
                input.push_str("N");
                input_len += 1;
            }
            Key::Char('o') | Key::Char('O') => {
                input.push_str("O");
                input_len += 1;
            }
            Key::Char('p') | Key::Char('P') => {
                input.push_str("P");
                input_len += 1;
            }
            Key::Char('q') | Key::Char('Q') => {
                input.push_str("Q");
                input_len += 1;
            }
            Key::Char('r') | Key::Char('R') => {
                input.push_str("R");
                input_len += 1;
            }
            Key::Char('s') | Key::Char('S') => {
                input.push_str("S");
                input_len += 1;
            }
            Key::Char('t') | Key::Char('T') => {
                input.push_str("T");
                input_len += 1;
            }
            Key::Char('u') | Key::Char('U') => {
                input.push_str("U");
                input_len += 1;
            }
            Key::Char('v') | Key::Char('V') => {
                input.push_str("V");
                input_len += 1;
            }
            Key::Char('w') | Key::Char('W') => {
                input.push_str("W");
                input_len += 1;
            }
            Key::Char('x') | Key::Char('X') => {
                input.push_str("X");
                input_len += 1;
            }
            Key::Char('y') | Key::Char('Y') => {
                input.push_str("Y");
                input_len += 1;
            }
            Key::Char('z') | Key::Char('Z') => {
                input.push_str("Z");
                input_len += 1;
            }
            _ => (),
        }

        write!(output.stdout, " ").unwrap();
        // Write the current Character
        for c in input.chars() {
            write!(output.stdout, "{} ", c).unwrap();
        }

        // Goto the position of the next character
        write!(
            output.stdout,
            "{}",
            termion::cursor::Goto(OFFSET_X + u16::from(input_len) * 2 + 1, OFFSET_Y + line)
        )
        .unwrap();

        // If the current word's length is 5 - register the word and move to next line
        if input_len == 5 {
            words[usize::from(word_count)] = input;

            // Check if the word is found
            if words[usize::from(word_count)] == bordle {
                write!(
                    output.stdout,
                    "{}You found the word",
                    termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + line + 2)
                )
                .unwrap();
                return;
            }
            word_count += 1;

            input_len = 0;

            input = String::new();
        }

        if word_count >= 5 {
            write!(
                output.stdout,
                "{}The world was {}",
                termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + line + 2),
                bordle
            )
            .unwrap();

            break;
        }

        output.stdout.flush().unwrap();

        write!(
            output.stdout,
            "{}",
            termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + line + 1)
        )
        .unwrap();
        write!(
            output.stdout,
            r#"{}{}"#,
            termion::cursor::Goto(OFFSET_X + 1, OFFSET_Y + 1),
            termion::clear::All
        )
        .unwrap();
    }
}

fn has_char(word: &String, c: char) -> bool {
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
            print!("{} ", color::Fg(color::Green));
            write!(out.stdout, "{}", guess_chars[i]).unwrap();
        } else if has_char(&original, guess_chars[i]) {
            print!("{} ", color::Fg(color::Yellow));
            write!(out.stdout, "{}", guess_chars[i]).unwrap();
        } else {
            print!("{} ", color::Fg(color::White));
            write!(out.stdout, "{}", guess_chars[i]).unwrap();
        }
    }
    print!("{}\n", color::Fg(color::White));
}

fn get_b<'a>() -> String {
    let contents = fs::read_to_string("./words.txt").expect("unable to get words");

    let data: Vec<&str> = contents.split(" ").collect();

    let mut rng = rand::thread_rng();
    let bordle_index = rng.gen_range(0..(data.len() - 1));

    let b = String::from(data[bordle_index].to_uppercase());

    return b;
}
