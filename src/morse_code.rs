#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        self.chars().filter_map(convert_to_pulse).collect()
    }
}

fn convert_to_pulse(c: char) -> Option<Vec<Pulse>> {
    use Pulse::*;
    match c {
        'A' | 'a' => Some(vec![Short, Long]),
        'B' | 'b' => Some(vec![Long, Short, Short, Short]),
        'C' | 'c' => Some(vec![Long, Short, Long, Short]),
        'D' | 'd' => Some(vec![Long, Short, Short]),
        'E' | 'e' => Some(vec![Short]),
        'F' | 'f' => Some(vec![Short, Short, Long, Short]),
        'G' | 'g' => Some(vec![Long, Long, Short]),
        'H' | 'h' => Some(vec![Short, Short, Short, Short, Short]),
        'I' | 'i' => Some(vec![Short, Short]),
        'J' | 'j' => Some(vec![Short, Long, Long, Long]),
        'K' | 'k' => Some(vec![Long, Short, Long]),
        'L' | 'l' => Some(vec![Short, Long, Short, Short]),
        'M' | 'm' => Some(vec![Long, Long]),
        'N' | 'n' => Some(vec![Long, Short]),
        'O' | 'o' => Some(vec![Long, Long, Long]),
        'P' | 'p' => Some(vec![Short, Long, Long, Short]),
        'Q' | 'q' => Some(vec![Long, Long, Short, Long]),
        'R' | 'r' => Some(vec![Short, Long, Short]),
        'S' | 's' => Some(vec![Short, Short, Short]),
        'T' | 't' => Some(vec![Long]),
        'U' | 'u' => Some(vec![Short, Short, Long]),
        'V' | 'v' => Some(vec![Short, Short, Short, Long]),
        'W' | 'w' => Some(vec![Short, Long, Long]),
        'X' | 'x' => Some(vec![Long, Short, Short, Long]),
        'Y' | 'y' => Some(vec![Long, Short, Long, Long]),
        'Z' | 'z' => Some(vec![Long, Long, Short, Short]),

        '1' => Some(vec![Short, Long, Long, Long, Long]),
        '2' => Some(vec![Short, Short, Long, Long, Long]),
        '3' => Some(vec![Short, Short, Short, Long, Long]),
        '4' => Some(vec![Short, Short, Short, Short, Long]),
        '5' => Some(vec![Short, Short, Short, Short, Short]),
        '6' => Some(vec![Long, Short, Short, Short, Short]),
        '7' => Some(vec![Long, Long, Short, Short, Short]),
        '8' => Some(vec![Long, Long, Long, Short, Short]),
        '9' => Some(vec![Long, Long, Long, Long, Short]),
        '0' => Some(vec![Long, Long, Long, Long, Long]),
        _ => None,
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        };
        print!(" ");
    };
    println!();
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}