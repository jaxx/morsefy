extern crate kernel32;

use std::io::{Error, Result};

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
                    '.' => beep(750, 200).unwrap(),
                    '-' => beep(750, 600).unwrap(),
                    _ => panic!("Wrong morse signal")
                }
            }
        }
    }
}

fn clean_text(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    words.join(" ")
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
        Some(_) | None => panic!("Mapping not found."),
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
    assert_eq!(&clean_text("see    on    test"), "see on test");
    assert_eq!(&clean_text("see "), "see");
}