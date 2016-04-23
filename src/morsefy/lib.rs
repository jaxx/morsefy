extern crate kernel32;

use std::io::{Error, Result};
use std::thread;
use std::time::Duration;

const DOT_DURATION: u32 = 200;
const DASH_DURATION: u32 = 3 * DOT_DURATION;
const WORD_SEPARATOR: u32 = 7 * DOT_DURATION;

#[derive(Debug)]
pub struct Morse {
    text: String
}

impl Morse {
    pub fn new(text: &str) -> Morse {
        Morse { text: clean_text(text) }
    }

    pub fn play(&self) {
        for ch in self.text.chars() {
            let morse_str = map_char(ch);

            for morse_char in morse_str.chars() {
                match morse_char {
                    '.' => beep(750, DOT_DURATION).unwrap(),
                    '-' => beep(750, DASH_DURATION).unwrap(),
                    ' ' => pause(WORD_SEPARATOR),
                    _ => panic!("Wrong morse signal")
                }

                pause(DOT_DURATION);
            }

            pause(DASH_DURATION);
        }
    }
}

fn clean_text(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    words.join(" ")
}

fn pause(duration: u32) {
    thread::sleep(Duration::from_millis(duration as u64));
}

fn map_char<'a>(c: char) -> &'a str {
    match c.to_lowercase().next() {
        Some('a') => ".-",
        Some('b') => "-...",
        Some('c') => "-.-.",
        Some('d') => "-..",
        Some('e') => ".",
        Some('f') => "..-.",
        Some('g') => "--.",
        Some('h') => "....",
        Some('i') => "..",
        Some('j') => ".---",
        Some('k') => "-.-",
        Some('l') => ".-..",
        Some('m') => "--",
        Some('n') => "-.",
        Some('o') => "---",
        Some('p') => ".--.",
        Some('q') => "--.-",
        Some('r') => ".-.",
        Some('s') => "...",
        Some('t') => "-",
        Some('u') => "..-",
        Some('v') => "...-",
        Some('w') => ".--",
        Some('x') => "-..-",
        Some('y') => "-.--",
        Some('z') => "--..",
        Some('1') => ".----",
        Some('2') => "..---",
        Some('3') => "...--",
        Some('4') => "....-",
        Some('5') => ".....",
        Some('6') => "-....",
        Some('7') => "--...",
        Some('8') => "---..",
        Some('9') => "----.",
        Some('0') => "-----",
        Some(' ') => " ",
        Some(_) | None => panic!("Character mapping not found.")
    }
}

fn beep(freq: u32, duration: u32) -> Result<()> {
    unsafe {
        match kernel32::Beep(freq, duration) {
            0 => Err(Error::last_os_error()),
            _ => Ok(()),
        }
    }
}


#[test]
fn clear_text_test() {
    assert_eq!(&clean_text("this    is    a test"), "this is a test");
    assert_eq!(&clean_text("this "), "this");
    assert_eq!(&clean_text(""), "");
}